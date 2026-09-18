//! `istmo-{{project-name}}` — {{description}}
//!
//! This crate exposes a single async trait ([`{{crate_name | pascal_case}}`])
//! backed by native `SharedPreferences`-shaped code on Android and
//! `UserDefaults`-shaped code on Apple platforms. Reference native
//! implementations ship alongside this crate under
//! [`native/`](https://github.com/{{gh_username}}/{{project-name}}/tree/main/native).

#![doc(html_root_url = "https://docs.rs/istmo-{{project-name}}")]

#[cfg(feature = "codegen")]
pub mod codegen;

use istmo::{message, plugin};

/// Wire identifier for this plugin. Matches [`istmo.toml`](../istmo.toml).
pub const {{crate_name | upcase}}_PLUGIN_ID: &str = "{{gh_username}}.{{crate_name}}";

/// Config handed to the plugin at `acquire_with(cfg)` time. Extend with
/// the knobs your plugin needs (API keys, backend URLs, feature toggles).
#[message(bincode = "::bincode")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct {{crate_name | pascal_case}}Config {
    // TODO: replace with your own fields.
    pub example_setting: String,
}

/// Domain error surface. Adjust variants to match the failure modes of the
/// underlying platform APIs — the wire codec is derived automatically.
#[message(bincode = "::bincode")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum {{crate_name | pascal_case}}Error {
    Backend(String),
    InvalidInput(String),
}

impl std::fmt::Display for {{crate_name | pascal_case}}Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Backend(msg)      => write!(f, "{{project-name}} backend error: {msg}"),
            Self::InvalidInput(msg) => write!(f, "{{project-name}} invalid input: {msg}"),
        }
    }
}

impl std::error::Error for {{crate_name | pascal_case}}Error {}

impl {{crate_name | pascal_case}}Config {
    #[must_use]
    pub fn new(example_setting: impl Into<String>) -> Self {
        Self { example_setting: example_setting.into() }
    }
}

/// Plugin trait. Every method is `async`; the macro emits
/// `{{crate_name | pascal_case}}Client` (used from Rust) and the wire codec.
///
/// **Replace the example methods below with your real API surface.**
#[plugin(
    name = "{{gh_username}}.{{crate_name}}",
    init = {{crate_name | pascal_case}}Config,
    crate = "::istmo",
)]
pub trait {{crate_name | pascal_case}} {
    /// Fetch a value the native side maintains.
    async fn get_value(&self, key: String) -> Result<Option<String>, {{crate_name | pascal_case}}Error>;

    /// Write a value the native side will persist.
    async fn set_value(&self, key: String, value: String) -> Result<(), {{crate_name | pascal_case}}Error>;
}
