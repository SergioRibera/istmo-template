// post-generate hook: prune unused archetype / platform / CI branches,
// then flatten the chosen archetype into the project root.

let is_app       = variable::get("is_app");
let is_plugin    = variable::get("is_plugin");
let has_android  = variable::get("has_android");
let has_ios      = variable::get("has_ios");
let has_desktop  = variable::get("has_desktop");
let ci_github    = variable::get("ci_github");
let ci_gitlab    = variable::get("ci_gitlab");
let bundle       = variable::get("bundle_desktop");
let publish_play = variable::get("publish_play");
let publish_appstore = variable::get("publish_appstore");
let publish_crates = variable::get("publish_crates");
let plugin_google = variable::get("plugin_google_sign_in");
let plugin_data  = variable::get("plugin_data_store");
let plugin_lifecycle = variable::get("plugin_app_lifecycle");
let plugin_deep = variable::get("plugin_deep_links");
let plugin_safe = variable::get("plugin_safe_area");

fn rm(path) {
    if file::exists(path) {
        file::delete(path);
    }
}

// 1. drop the unpicked archetype tree
if is_app    { rm("plugin"); }
if is_plugin { rm("app"); }

// 2. platform / CI-driven prune inside the chosen archetype
if is_app {
    if !has_android { rm("app/android"); }
    if !has_ios     { rm("app/ios"); }
    if !has_desktop {
        rm("app/flake.nix");
        rm("app/src/desktop.rs");
    }
}
if is_plugin {
    if !has_android { rm("plugin/native/android"); }
    if !has_ios     { rm("plugin/native/ios"); }
}

if !ci_github { rm(".github"); }
if !ci_gitlab { rm(".gitlab-ci.yml"); }

if ci_github {
    if !publish_play     { rm(".github/workflows/release-play.yml"); }
    if !publish_appstore { rm(".github/workflows/release-appstore.yml"); }
    if !publish_crates   { rm(".github/workflows/release-crates.yml"); }
}

// 3. license file(s)
let lic = variable::get("crate_license");
if lic != "MIT" && lic != "MIT OR Apache-2.0" { rm("LICENSE-MIT"); }
if lic != "Apache-2.0" && lic != "MIT OR Apache-2.0" { rm("LICENSE-APACHE"); }
if lic != "GPL-3.0-or-later" { rm("LICENSE-GPL"); }
if lic != "Unlicense" { rm("UNLICENSE"); }

// 4. flatten: move every child of the chosen archetype dir up to root.
//    Rhai's `file::rename` handles files and dirs, so we can enumerate.
let src_root = if is_app { "app" } else { "plugin" };

fn move_up(src_root, name) {
    let src = src_root + "/" + name;
    if file::exists(src) {
        file::rename(src, name);
    }
}

let entries = [
    "Cargo.toml", "build.rs", "istmo.toml",
    "src", "tests", "native",
    "android", "ios", "flake.nix",
];
for name in entries {
    move_up(src_root, name);
}

// scrub the now-empty archetype dir
rm(src_root);

// self-delete: the hook scripts are only useful during templating.
rm("scripts");

print("template scaffold complete. next: read README.md for wiring steps.");
