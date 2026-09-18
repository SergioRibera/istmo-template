package {{bundle_id}}

import android.app.NativeActivity
import android.os.Bundle
import dev.istmo.runtime.IstmoRuntime

// NativeActivity subclass that boots IstmoRuntime + registers plugin
// dispatchers BEFORE `android_main` fires in the Rust cdylib.
class {{project-name | pascal_case}}Activity : NativeActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        IstmoRuntime.instance.start(this)

        // TODO: register your plugin dispatchers here — the generated
        // classes live under `{{bundle_id}}.gen`.
        //
        // IstmoRuntime.instance.registerHandler(
        //     DataStoreDispatcher.PLUGIN_ID,
        //     DataStoreDispatcher(DataStoreBackendImpl(this), DataStoreCodecsImpl()),
        // )

        super.onCreate(savedInstanceState)
    }
}
