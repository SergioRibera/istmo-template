//! Desktop runtime. There is no Kotlin / Swift side on desktop, so the
//! plugins either run as Rust hosts in-process or are answered by the
//! small emulator below.

use std::sync::Arc;

use istmo::{Envelope, Frame, IstmoError, Runtime, RuntimeConfig, RuntimeInit};
{% if plugin_data_store %}use istmo_data_store::{DataStoreHost, DesktopDataStoreFactory};
{% endif %}
/// Start the desktop runtime: register the Rust hosts and answer every
/// other outbound call on a background thread.
///
/// The runtime is installed process-wide, as on mobile, so
/// `Runtime::global()` and the clients' `acquire()` constructors work the
/// same in every shell.
pub fn start() -> Result<Arc<Runtime>, IstmoError> {
    let RuntimeInit { runtime, outbound } = Runtime::init(RuntimeConfig::inline())?;
{% if plugin_data_store %}
    // Local hosts answer in-process; their calls never reach `outbound`.
    runtime.register_host(DataStoreHost::new(DesktopDataStoreFactory));
{% endif %}
    let emulator = Arc::clone(&runtime);
    let spawned = std::thread::Builder::new()
        .name("desktop-emulator".into())
        .spawn(move || {
            while let Ok(envelope) = outbound.recv() {
                emulate(&emulator, envelope);
            }
        });
    if let Err(err) = spawned {
        log::error!("cannot start the desktop emulator: {err}");
    }
    Ok(runtime)
}

/// Answer a frame meant for a native backend that does not exist on
/// desktop. Match on `plugin_id` / `method` to emulate one, encoding the
/// return value with `istmo::codec::encode`.
fn emulate(runtime: &Arc<Runtime>, envelope: Envelope) {
    let (call_id, what) = match envelope.frame {
        Frame::Call {
            call_id,
            plugin_id,
            method,
            ..
        } => (call_id, format!("{plugin_id}::{method}")),
        Frame::CreateInstance {
            call_id, plugin_id, ..
        } => (call_id, format!("{plugin_id} instance")),
        other => {
            log::debug!("desktop emulator ignores {other:?}");
            return;
        }
    };
    log::warn!("{what} has no desktop backend; failing the call");
    // An empty error payload reaches the caller as
    // `IstmoError::PluginError` instead of leaving it waiting forever.
    let reply = Envelope::new(Frame::Respond {
        call_id,
        result: Err(Vec::new()),
    });
    if let Err(err) = runtime.dispatch_inbound(reply) {
        log::debug!("desktop emulator reply for {what} dropped: {err}");
    }
}
