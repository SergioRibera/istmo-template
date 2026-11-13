//! Desktop entry point. Mobile targets get an empty `main` so the same
//! manifest builds the `cdylib`, the `staticlib` and this binary.

#[cfg(not(any(
    target_os = "android",
    target_os = "ios",
    target_os = "tvos",
    target_os = "watchos",
    target_os = "visionos",
)))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use {{crate_name}}::{app::App, desktop};

    let runtime = desktop::start()?;
{% if ui_framework == "egui" %}
    let mut viewport = eframe::egui::ViewportBuilder::default().with_inner_size([960.0, 640.0]);
    // `[app.icon]` in istmo.toml (`desktop`, else `image`); `None` without one.
    if let Some(png) = istmo::app_icon!() {
        match eframe::icon_data::from_png_bytes(png) {
            Ok(icon) => viewport = viewport.with_icon(icon),
            Err(err) => log::warn!("app icon is not a valid PNG: {err}"),
        }
    }
    let options = eframe::NativeOptions {
        viewport,
        ..eframe::NativeOptions::default()
    };
    eframe::run_native(
        "{{display_name}}",
        options,
        Box::new(move |_cc| Ok(Box::new(App::new(runtime)))),
    )?;
{% else %}
    // `istmo::app_icon!()` holds the `[app.icon]` PNG for the window icon.
    App::new(runtime).run_desktop();
{% endif %}    Ok(())
}

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
