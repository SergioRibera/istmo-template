//! Shared UI shell — the same code drives desktop and mobile.
//!
//! [`App`] owns the runtime handle, and later your plugin clients
//! (`DataStoreClient::from_runtime_with(&runtime, config)`, …); the UI only
//! reads from it.

use std::sync::Arc;
{% if ui_framework == "egui" %}
use eframe::egui;
use istmo::Runtime;

/// Application state shared by every shell.
#[derive(Debug)]
pub struct App {
    runtime: Arc<Runtime>,
    clicks: u32,
}

impl App {
    #[must_use]
    pub const fn new(runtime: Arc<Runtime>) -> Self {
        Self { runtime, clicks: 0 }
    }

    /// Runtime the plugin clients are acquired from.
    #[must_use]
    pub const fn runtime(&self) -> &Arc<Runtime> {
        &self.runtime
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("{{display_name}}");
            ui.label("Replace this scaffold with the real UI for your app.");
            let label = format!("clicked {} times", self.clicks);
            if ui.button(label).clicked() {
                self.clicks += 1;
            }
        });
    }
}
{% else %}
use istmo::Runtime;

/// Application state shared by every shell.
#[derive(Debug)]
pub struct App {
    runtime: Arc<Runtime>,
}

impl App {
    #[must_use]
    pub const fn new(runtime: Arc<Runtime>) -> Self {
        Self { runtime }
    }

    /// Runtime the plugin clients are acquired from.
    #[must_use]
    pub const fn runtime(&self) -> &Arc<Runtime> {
        &self.runtime
    }

    /// Desktop entry point of the `{{ui_framework}}` shell.
    pub fn run_desktop(self) {
        // TODO: launch the {{ui_framework}} UI here.
        log::info!("{{project-name}} desktop shell started: {self:?}");
    }

    /// Mobile entry point of the `{{ui_framework}}` shell.
    pub fn run_mobile(self) {
        // TODO: launch the {{ui_framework}} UI here.
        log::info!("{{project-name}} mobile shell started: {self:?}");
    }
}
{% endif -%}
