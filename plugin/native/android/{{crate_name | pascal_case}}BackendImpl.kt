package {{bundle_id}}.{{crate_name | remove: "_"}}

import android.content.Context
import android.content.SharedPreferences
import dev.istmo.runtime.BackendException

/**
 * Reference backend for the `{{gh_username}}.{{crate_name}}` plugin.
 *
 * Implements the `{{crate_name | pascal_case}}Backend` interface the app's
 * build generates from the Rust trait (method names keep the Rust
 * spelling). Replace the `SharedPreferences` example with the platform
 * API your plugin wraps; throw `BackendException({{crate_name | pascal_case}}Error.…)`
 * for domain errors.
 */
class {{crate_name | pascal_case}}BackendImpl(
    context: Context,
    config: {{crate_name | pascal_case}}Config,
) : {{crate_name | pascal_case}}Backend {

    private val prefs: SharedPreferences = context.getSharedPreferences(
        "${context.packageName}.{{crate_name}}.${config.exampleSetting}",
        Context.MODE_PRIVATE,
    )

    override suspend fun get_value(key: String): String? = prefs.getString(key, null)

    override suspend fun set_value(key: String, value: String) {
        if (key.isEmpty()) {
            throw BackendException({{crate_name | pascal_case}}Error.InvalidInput("empty key"))
        }
        if (!prefs.edit().putString(key, value).commit()) {
            throw BackendException({{crate_name | pascal_case}}Error.Backend("SharedPreferences commit failed"))
        }
    }
}
