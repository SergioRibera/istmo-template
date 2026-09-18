//! `{{project-name}}` — {{description}}
//!
//! Same Rust code compiled three ways:
//!
//! - Desktop `bin`  — `src/main.rs`.
//! - Android `cdylib` linked from `NativeActivity` — `mobile_main` below.
//! - iOS `staticlib` linked into the SwiftUI shell — same `mobile_main`.

pub mod app;

#[cfg(not(any(
    target_os = "android",
    target_os = "ios",
    target_os = "tvos",
    target_os = "watchos",
    target_os = "visionos",
)))]
pub mod desktop;

// The macro auto-wires every plugin the app depends on:
//   - `istmo.toml` [[remote_override]] entries move plugins to :remote.
//   - `DEP_*_ISTMO_MANIFEST` env vars (set by each plugin's build.rs)
//     supply the fully-qualified `<Trait>Client` paths.
// Manual `plugins: [ClientA, ClientB, ...]` / `remote: [...]` sections
// still work for anything not carried by a plugin crate's manifest.
istmo::runtime!();

#[cfg(any(
    target_os = "android",
    target_os = "ios",
    target_os = "tvos",
    target_os = "watchos",
    target_os = "visionos",
))]
#[istmo::mobile_app]
fn mobile_main() {
    #[cfg(target_os = "android")]
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );
    log::info!("{{project-name}} mobile entry point running");

    // TODO: acquire your plugin clients here with the right config type.
    // Example (data-store):
    //
    //     use istmo_data_store::{DataStoreClient, DataStoreConfig};
    //     let cfg = DataStoreConfig::new("{{crate_name}}");
    //     let client = pollster::block_on(DataStoreClient::acquire_with(cfg))
    //         .expect("acquire data store client");

    {% if ui_framework == "egui" %}
    #[cfg(target_os = "android")]
    let event_loop_builder: Option<
        Box<dyn FnOnce(&mut winit::event_loop::EventLoopBuilder<eframe::UserEvent>)>,
    > = {
        let android_app = istmo::android::android_app()
            .expect("android_main should have stashed the AndroidApp handle");
        Some(Box::new(move |builder| {
            use winit::platform::android::EventLoopBuilderExtAndroid;
            builder.with_android_app(android_app);
        }))
    };
    #[cfg(not(target_os = "android"))]
    let event_loop_builder: Option<
        Box<dyn FnOnce(&mut winit::event_loop::EventLoopBuilder<eframe::UserEvent>)>,
    > = None;

    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        event_loop_builder,
        ..Default::default()
    };

    if let Err(err) = eframe::run_native(
        "{{display_name}}",
        options,
        Box::new(move |cc| Ok(Box::new(app::App::new(cc)))),
    ) {
        log::error!("eframe exited with error: {err:?}");
    }
    {% else %}
    // TODO: replace with the `{{ui_framework}}` entry point of your choice.
    app::App::run_mobile();
    {% endif %}
}
