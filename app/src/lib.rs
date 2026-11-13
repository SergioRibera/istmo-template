//! `{{project-name}}` — {{description}}
//!
//! One crate, three shells:
//!
//! - Desktop binary — `src/main.rs`, against a mock runtime.
//! - Android `cdylib`, loaded by `IstmoGameActivity` — `mobile_main` below.
//! - iOS `staticlib`, linked into the Xcode app — same `mobile_main`.

pub mod app;
{% if has_desktop %}
#[cfg(not(any(
    target_os = "android",
    target_os = "ios",
    target_os = "tvos",
    target_os = "watchos",
    target_os = "visionos",
)))]
pub mod desktop;
{% endif %}{% if plugin_app_lifecycle or plugin_deep_links or plugin_safe_area %}
{% if plugin_app_lifecycle %}use istmo::plugins::AppLifecycle;
{% endif %}{% if plugin_deep_links %}use istmo::plugins::DeepLinks;
{% endif %}{% if plugin_safe_area %}use istmo::plugins::SafeArea;
{% endif %}
// Plugin crates (`istmo-data-store`, …) are wired automatically from
// their manifests; built-in `istmo::plugins` clients are listed here.
istmo::runtime!(
    plugins: [
{% if plugin_app_lifecycle %}        AppLifecycle,
{% endif %}{% if plugin_deep_links %}        DeepLinks,
{% endif %}{% if plugin_safe_area %}        SafeArea,
{% endif %}    ],
);
{% else %}
// Every plugin crate this app depends on is wired automatically from its
// manifest. List extra clients (built-in `istmo::plugins`, local plugin
// traits) with `plugins: [..]`, and the ones that run in the `:remote`
// Android process with `remote: [..]`.
istmo::runtime!();
{% endif %}
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

    let runtime = match istmo::Runtime::global() {
        Ok(runtime) => runtime,
        Err(err) => {
            log::error!("istmo runtime is not running: {err}");
            return;
        }
    };
{% if ui_framework == "egui" %}
    #[cfg(target_os = "android")]
    let event_loop_builder = istmo::android::android_app().map(|android_app| {
        let hook: eframe::EventLoopBuilderHook = Box::new(move |builder| {
            use winit::platform::android::EventLoopBuilderExtAndroid;
            builder.with_android_app(android_app);
        });
        hook
    });
    #[cfg(not(target_os = "android"))]
    let event_loop_builder = None;

    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        event_loop_builder,
        ..Default::default()
    };
    if let Err(err) = eframe::run_native(
        "{{display_name}}",
        options,
        Box::new(move |_cc| Ok(Box::new(app::App::new(runtime)))),
    ) {
        log::error!("eframe exited with error: {err}");
    }
{% else %}
    app::App::new(runtime).run_mobile();
{% endif %}}
