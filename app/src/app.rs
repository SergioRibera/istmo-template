//! Shared UI shell — same code drives desktop and mobile.
//!
//! Replace the {{ui_framework}} scaffold below if you picked a different
//! framework at `cargo generate` time. The Rust side is framework-agnostic:
//! the runtime `Arc` + plugin clients live in an app-owned struct, the UI
//! only draws from them.

{% if ui_framework == "egui" %}
use eframe::egui;

pub struct App {
    // TODO: hold `Arc<Runtime>` + your plugin client(s) + transient UI state.
    counter: u32,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self { counter: 0 }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("{{display_name}}");
            ui.label("Replace this scaffold with the real UI for your app.");
            if ui.button(format!("clicked {} times", self.counter)).clicked() {
                self.counter += 1;
            }
        });
    }
}
{% elsif ui_framework == "freya" %}
// TODO: freya scaffold. See https://freyaui.dev/ for `fn app()` + `launch(app)`.
pub struct App;
impl App {
    pub fn run_desktop() { /* freya::launch(app); */ }
    pub fn run_mobile()  { /* freya::launch(app); */ }
}
{% elsif ui_framework == "slint" %}
// TODO: slint scaffold — `slint::slint! { export component AppWindow { ... } }`.
pub struct App;
impl App {
    pub fn run_desktop() {}
    pub fn run_mobile()  {}
}
{% elsif ui_framework == "dioxus" %}
// TODO: dioxus scaffold — `dioxus::launch(fn app() -> Element { ... })`.
pub struct App;
impl App {
    pub fn run_desktop() {}
    pub fn run_mobile()  {}
}
{% else %}
// No UI framework selected. Wire your own here.
pub struct App;
impl App {
    pub fn run_desktop() {}
    pub fn run_mobile()  {}
}
{% endif %}
