//! Desktop entry point. Mobile targets get an empty `main` so the same
//! manifest builds the `cdylib`, the `staticlib` and this binary.

#[cfg(not(any(
    target_os = "android",
    target_os = "ios",
    target_os = "tvos",
    target_os = "watchos",
    target_os = "visionos",
)))]
fn main() {% if ui_framework == "egui" %}-> Result<(), eframe::Error> {% endif %}{
    use {{crate_name}}::{app::App, desktop};

    let runtime = desktop::start();
{% if ui_framework == "egui" %}
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default().with_inner_size([960.0, 640.0]),
        ..eframe::NativeOptions::default()
    };
    eframe::run_native(
        "{{display_name}}",
        options,
        Box::new(move |_cc| Ok(Box::new(App::new(runtime)))),
    )
{% else %}
    App::new(runtime).run_desktop();
{% endif %}}

#[cfg(any(
    target_os = "android",
    target_os = "ios",
    target_os = "tvos",
    target_os = "watchos",
    target_os = "visionos",
))]
#[allow(
    clippy::missing_const_for_fn,
    reason = "fires only for the test harness build; `main` cannot be `const`"
)]
fn main() {}
