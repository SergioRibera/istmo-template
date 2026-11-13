//! `istmo-{{project-name}}` — {{description}}
//!
//! Rust code calls the async [`{{crate_name | pascal_case}}`] trait through
//! the generated [`{{crate_name | pascal_case}}Client`]; the native side
//! implements it. Reference native implementations ship alongside this
//! crate under
//! [`native/`](https://github.com/{{gh_username}}/{{project-name}}/tree/main/native).
//!
//! Enable the `codegen` feature to expose the plugin's `Contract` for
//! downstream code generation without the build-script handover.

#![doc(html_root_url = "https://docs.rs/istmo-{{project-name}}")]

#[cfg(feature = "codegen")]
pub mod codegen;

use istmo_macros::{message, plugin};

/// Wire identifier for this plugin. Matches `istmo.toml`.
pub const {{crate_name | upcase}}_PLUGIN_ID: &str = "{{gh_username}}.{{crate_name}}";

/// Config handed to the native factory when a client is acquired with
/// `{{crate_name | pascal_case}}Client::acquire_with(config)`. Extend it
/// with the knobs your plugin needs (backend URLs, feature toggles).
///
/// Mark secrets with `#[redact]` to keep them out of the generated
/// `Debug` output.
#[message(wincode = "::wincode")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct {{crate_name | pascal_case}}Config {
    /// Example setting; replace with your own fields.
    pub example_setting: String,
}

impl {{crate_name | pascal_case}}Config {
    #[must_use]
    pub fn new(example_setting: impl Into<String>) -> Self {
        Self {
            example_setting: example_setting.into(),
        }
    }
}

/// Domain errors returned by every [`{{crate_name | pascal_case}}`] method.
///
/// The type travels on the wire, and `#[plugin]` requires it to
/// implement [`std::error::Error`].
#[message(wincode = "::wincode")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum {{crate_name | pascal_case}}Error {
    /// The native backend failed; carries the platform's message.
    Backend(String),
    /// The caller passed an argument the backend rejects.
    InvalidInput(String),
}

impl std::fmt::Display for {{crate_name | pascal_case}}Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Backend(msg) => write!(f, "{{project-name}} backend error: {msg}"),
            Self::InvalidInput(msg) => write!(f, "{{project-name}} invalid input: {msg}"),
        }
    }
}

impl std::error::Error for {{crate_name | pascal_case}}Error {}

/// Plugin surface. `#[plugin]` keeps the trait and emits
/// `{{crate_name | pascal_case}}Client` (called from Rust) and
/// `{{crate_name | pascal_case}}Host` (for a Rust implementation, e.g. on
/// desktop).
///
/// Replace the example methods with your real API:
///
/// - streams: a non-`async` method marked `#[stream]`
///   (`istmo_macros::stream`) returning the item type, `T` or
///   `Result<T, E>` (the first `Err` ends the stream);
/// - cancellation: an `istmo_core::CancelToken` argument, filled by the
///   runtime and left off the wire;
/// - native handles: `#[handle(T)]` on an `istmo_core::NativeHandleId`
///   field plus `#[owned]` on the method add a `*_owned` client method
///   that adopts the handle.
#[plugin(
    name = "{{gh_username}}.{{crate_name}}",
    init = {{crate_name | pascal_case}}Config,
    crate = "::istmo_core",
)]
pub trait {{crate_name | pascal_case}} {
    /// Read the value stored under `key`, if any.
    async fn get_value(&self, key: String) -> Result<Option<String>, {{crate_name | pascal_case}}Error>;

    /// Store `value` under `key`.
    async fn set_value(&self, key: String, value: String) -> Result<(), {{crate_name | pascal_case}}Error>;
}
