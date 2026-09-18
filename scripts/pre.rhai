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

// Liquid `replace` filter cannot be used in file names (cargo-generate's
// filename parser rejects quoted literals). Precompute the slash-separated
// form of `bundle_id` here so templates can reference `{{bundle_id_path}}`.
// Rhai's `String::replace` mutates in place and returns unit, so we copy
// first and then mutate.
let bid_path = variable::get("bundle_id");
bid_path.replace(".", "/");
variable::set("bundle_id_path", bid_path);
