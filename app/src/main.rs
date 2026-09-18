// Desktop entry point. Mobile targets get a stub `main` so the same
// `Cargo.toml` produces `cdylib` + `staticlib` + a `bin` without extra cfgs.

#[cfg(not(any(
    target_os = "android",
    target_os = "ios",
    target_os = "tvos",
    target_os = "watchos",
    target_os = "visionos",
)))]
fn main() {% if ui_framework == "egui" %}-> Result<(), eframe::Error>{% endif %} {
    use istmo::core::{Runtime, RuntimeInit};

    let RuntimeInit { runtime, outbound } = Runtime::mock();
    {{project-name | snake_case}}::desktop::spawn(std::sync::Arc::clone(&runtime), outbound);

    // TODO: acquire your plugin clients through the mock runtime so the
    // desktop build can exercise the same code path the mobile shell uses.

    {% if ui_framework == "egui" %}
    let opts = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default().with_inner_size([960.0, 640.0]),
        ..eframe::NativeOptions::default()
    };
    eframe::run_native(
        "{{display_name}}",
        opts,
        Box::new(move |cc| Ok(Box::new({{project-name | snake_case}}::app::App::new(cc)))),
    )
    {% else %}
    // TODO: replace with your `{{ui_framework}}` runner.
    {{project-name | snake_case}}::app::App::run_desktop();
    {% endif %}
}

#[cfg(any(
    target_os = "android",
    target_os = "ios",
    target_os = "tvos",
    target_os = "visionos",
    target_os = "watchos",
))]
fn main() {}
