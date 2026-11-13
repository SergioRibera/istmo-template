package {{bundle_id}}.{{crate_name | remove: "_"}}

import android.content.Context

/**
 * Builds one backend per `{{crate_name | pascal_case}}Client::acquire_with(config)`.
 * Named by `android_backend` in `istmo.toml`; the app's generated
 * `IstmoPluginRegistry` constructs it with the application context.
 */
class {{crate_name | pascal_case}}FactoryImpl(private val context: Context) : {{crate_name | pascal_case}}Factory {
    override suspend fun create(config: {{crate_name | pascal_case}}Config): {{crate_name | pascal_case}}Backend =
        {{crate_name | pascal_case}}BackendImpl(context.applicationContext, config)
}
