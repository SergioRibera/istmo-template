//! End-to-end smoke: a mock runtime hosts an in-memory Rust implementation
//! of the plugin, and every method goes through the generated client.

use std::collections::HashMap;
use std::sync::Mutex;

use istmo_core::{IstmoError, Runtime, codec};
// One item per line: rustfmt keeps that layout whatever the crate name's
// length, so a freshly generated crate passes `cargo fmt --check`.
use istmo_{{crate_name}}::{{crate_name | pascal_case}};
use istmo_{{crate_name}}::{{crate_name | pascal_case}}Client;
use istmo_{{crate_name}}::{{crate_name | pascal_case}}Config;
use istmo_{{crate_name}}::{{crate_name | pascal_case}}Error;
use istmo_{{crate_name}}::{{crate_name | pascal_case}}Factory;
use istmo_{{crate_name}}::{{crate_name | pascal_case}}Host;

#[derive(Debug, Default)]
struct MemoryBackend {
    values: Mutex<HashMap<String, String>>,
}

impl {{crate_name | pascal_case}} for MemoryBackend {
    async fn get_value(&self, key: String) -> Result<Option<String>, {{crate_name | pascal_case}}Error> {
        Ok(istmo_core::sync::lock(&self.values).get(&key).cloned())
    }

    async fn set_value(&self, key: String, value: String) -> Result<(), {{crate_name | pascal_case}}Error> {
        if key.is_empty() {
            return Err({{crate_name | pascal_case}}Error::InvalidInput("empty key".into()));
        }
        istmo_core::sync::lock(&self.values).insert(key, value);
        Ok(())
    }
}

#[derive(Debug)]
struct MemoryFactory;

impl {{crate_name | pascal_case}}Factory for MemoryFactory {
    type Instance = MemoryBackend;

    fn create(&self, _config: {{crate_name | pascal_case}}Config) -> MemoryBackend {
        MemoryBackend::default()
    }
}

#[test]
fn client_round_trips_through_rust_host() {
    let init = Runtime::mock();
    init.runtime.register_host({{crate_name | pascal_case}}Host::new(MemoryFactory));

    let client = pollster::block_on({{crate_name | pascal_case}}Client::from_runtime_with(
        &init.runtime,
        {{crate_name | pascal_case}}Config::new("smoke"),
    ))
    .expect("acquire client");

    pollster::block_on(client.set_value("greeting".into(), "hello".into())).expect("set_value");
    let value = pollster::block_on(client.get_value("greeting".into())).expect("get_value");
    assert_eq!(value.as_deref(), Some("hello"));

    let missing = pollster::block_on(client.get_value("missing".into())).expect("get_value");
    assert_eq!(missing, None);
}

#[test]
fn domain_error_reaches_the_caller() {
    let init = Runtime::mock();
    init.runtime.register_host({{crate_name | pascal_case}}Host::new(MemoryFactory));

    let client = pollster::block_on({{crate_name | pascal_case}}Client::from_runtime_with(
        &init.runtime,
        {{crate_name | pascal_case}}Config::new("smoke"),
    ))
    .expect("acquire client");

    let err = pollster::block_on(client.set_value(String::new(), "x".into()))
        .expect_err("empty key is rejected");
    let IstmoError::PluginError { bytes } = err else {
        panic!("expected a domain error, got {err:?}");
    };
    let (decoded, _) = codec::decode::<{{crate_name | pascal_case}}Error>(&bytes).expect("decode domain error");
    assert_eq!(decoded, {{crate_name | pascal_case}}Error::InvalidInput("empty key".into()));
}
