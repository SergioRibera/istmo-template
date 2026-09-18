package {{gh_username}}.{{crate_name}}

import android.content.Context

// Reference backend for the {{crate_name | pascal_case}} plugin.
//
// The generated `{{crate_name | pascal_case}}Dispatcher` requires a
// `{{crate_name | pascal_case}}Backend` interface — implement each trait
// method with the real platform APIs.
class {{crate_name | pascal_case}}BackendImpl(private val context: Context) /* : {{crate_name | pascal_case}}Backend */ {

    // TODO: implement the generated interface methods here.
    //
    // suspend fun getValue(key: String): String? = ...
    // suspend fun setValue(key: String, value: String) { ... }
}
