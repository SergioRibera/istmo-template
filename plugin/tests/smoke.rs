//! End-to-end smoke: spin a mock runtime, install a fake host, exercise
//! every method through the generated client.

use istmo::core::{Runtime, RuntimeInit};
use istmo_{{crate_name}}::{
    {{crate_name | pascal_case}}Client, {{crate_name | pascal_case}}Config,
    {{crate_name | pascal_case}}Error,
};

#[test]
fn client_roundtrip() {
    let RuntimeInit { runtime, outbound: _ } = Runtime::mock();

    // TODO: register a fake `{{crate_name | pascal_case}}Host<Impl>` here that
    // answers `get_value` / `set_value` with in-memory state, then acquire a
    // client and assert the round-trip returns the expected payloads.
    let _rt = runtime;
    let _cfg = {{crate_name | pascal_case}}Config::new("smoke");

    // Placeholder assertion so the file compiles until the real fake host lands.
    let err = {{crate_name | pascal_case}}Error::InvalidInput("todo".into());
    assert_eq!(format!("{err}"), "{{project-name}} invalid input: todo");
}
