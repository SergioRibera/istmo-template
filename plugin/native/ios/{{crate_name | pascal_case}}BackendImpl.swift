import Foundation

/// Reference backend for the `{{gh_username}}.{{crate_name}}` plugin.
///
/// Conforms to the `{{crate_name | pascal_case}}Backend` protocol the app's
/// build generates from the Rust trait (method names keep the Rust
/// spelling). Replace the `UserDefaults` example with the platform API
/// your plugin wraps; throw `{{crate_name | pascal_case}}Error` cases for
/// domain errors.
final class {{crate_name | pascal_case}}BackendImpl: {{crate_name | pascal_case}}Backend {

    private let defaults: UserDefaults

    init(config: {{crate_name | pascal_case}}Config) throws {
        let app = Bundle.main.bundleIdentifier ?? "app"
        let suiteName = "\(app).{{crate_name}}.\(config.exampleSetting)"
        guard let store = UserDefaults(suiteName: suiteName) else {
            throw {{crate_name | pascal_case}}Error.backend("UserDefaults(suiteName: \(suiteName)) is unavailable")
        }
        self.defaults = store
    }

    func get_value(key: String) async throws -> String? {
        defaults.string(forKey: key)
    }

    func set_value(key: String, value: String) async throws {
        guard !key.isEmpty else {
            throw {{crate_name | pascal_case}}Error.invalidInput("empty key")
        }
        defaults.set(value, forKey: key)
    }
}

/// Builds one backend per `{{crate_name | pascal_case}}Client::acquire_with(config)`.
/// Named by `ios_backend` in `istmo.toml`; the app's generated
/// `IstmoPluginRegistry` constructs it.
final class {{crate_name | pascal_case}}FactoryImpl: {{crate_name | pascal_case}}Factory {
    func create(config: {{crate_name | pascal_case}}Config) async throws -> {{crate_name | pascal_case}}Backend {
        try {{crate_name | pascal_case}}BackendImpl(config: config)
    }
}
