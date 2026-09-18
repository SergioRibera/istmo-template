import Foundation

// Reference backend for the {{crate_name | pascal_case}} plugin.
//
// The generated `{{crate_name | pascal_case}}Dispatcher` requires a
// `{{crate_name | pascal_case}}Backend` protocol conformance — implement
// each trait method with the real platform APIs.
final class {{crate_name | pascal_case}}BackendImpl /* : {{crate_name | pascal_case}}Backend */ {

    init() {}

    // TODO: implement the generated protocol methods here.
    //
    // func getValue(key: String) async throws -> String? { ... }
    // func setValue(key: String, value: String) async throws { ... }
}
