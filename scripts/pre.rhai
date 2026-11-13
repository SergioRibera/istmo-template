// pre-generate hook: normalize inputs and derive convenience flags for
// downstream liquid templates + the post-generate cleanup pass.

let kind      = variable::get("kind");
let platforms = variable::get("platforms");
let plugins   = variable::get("plugins");
let publish   = variable::get("publish_targets");
let ci        = variable::get("ci");

fn contains(arr, needle) {
    for entry in arr {
        if entry == needle { return true; }
    }
    false
}

// archetype flags
variable::set("is_app",    kind == "app");
variable::set("is_plugin", kind == "plugin");

// platform flags
variable::set("has_android", contains(platforms, "android"));
variable::set("has_ios",     contains(platforms, "ios"));
variable::set("has_desktop", contains(platforms, "desktop"));

// initial plugin flags (app archetype only)
variable::set("plugin_app_lifecycle",  contains(plugins, "app_lifecycle"));
variable::set("plugin_deep_links",     contains(plugins, "deep_links"));
variable::set("plugin_safe_area",      contains(plugins, "safe_area"));
variable::set("plugin_google_sign_in", contains(plugins, "google_sign_in"));
variable::set("plugin_data_store",     contains(plugins, "data_store"));
variable::set("plugin_biometric",      contains(plugins, "biometric"));
variable::set("plugin_share",          contains(plugins, "share"));
variable::set("plugin_file_picker",    contains(plugins, "file_picker"));
variable::set("plugin_opener",         contains(plugins, "opener"));
variable::set("plugin_camera",         contains(plugins, "camera"));
variable::set("plugin_barcode_scanner", contains(plugins, "barcode_scanner"));

// plugin crates the app depends on, and the ones with a Rust desktop host
let crate_plugins = ["google_sign_in", "data_store", "biometric", "share", "file_picker", "opener", "camera", "barcode_scanner"];
let host_plugins  = ["data_store", "biometric", "file_picker", "opener", "camera", "barcode_scanner"];
let any_crate = false;
let any_host  = false;
for p in crate_plugins { if contains(plugins, p) { any_crate = true; } }
for p in host_plugins  { if contains(plugins, p) { any_host = true; } }
variable::set("plugin_crates", any_crate);
variable::set("desktop_hosts", any_host);

// publish targets
variable::set("publish_github",   contains(publish, "github_release"));
variable::set("publish_play",     contains(publish, "play_store"));
variable::set("publish_appstore", contains(publish, "app_store"));
variable::set("publish_crates",   contains(publish, "crates_io"));

// ci providers
variable::set("ci_github", contains(ci, "github"));
variable::set("ci_gitlab", contains(ci, "gitlab"));

// nix-bundle only makes sense for desktop apps
let use_bundle = variable::get("use_nix_bundle") == true
    && contains(platforms, "desktop")
    && kind == "app";
variable::set("bundle_desktop", use_bundle);

// crates_io publishing is only offered for plugins
if kind != "plugin" && contains(publish, "crates_io") {
    print("note: crates_io publish target ignored for app archetype");
    variable::set("publish_crates", false);
}

