//! Feature-gated `Contract` accessor. Downstream apps enable
//! `istmo-{{project-name}}/codegen` in their `[build-dependencies]` and call
//! [`contract()`] to feed the Kotlin / Swift generators.

use istmo_build::{Contract, extract_contract};

// `env!` expands while this crate compiles, so the path keeps pointing at
// this crate's sources whichever build script calls `contract()`.
const PLUGIN_SRC: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs");

#[must_use]
pub fn contract() -> Contract {
    extract_contract(PLUGIN_SRC, "{{crate_name | pascal_case}}").expect("src/lib.rs declares the {{crate_name | pascal_case}} plugin trait")
}
