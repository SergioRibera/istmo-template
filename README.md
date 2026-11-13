# {{project-name}}

{{description}}
{% if is_app %}
Multi-platform application built on [istmo](https://crates.io/crates/istmo):
one Rust crate drives {% if has_android %}Android {% endif %}{% if has_ios %}iOS {% endif %}{% if has_desktop %}desktop{% endif %}.
{% else %}
[istmo](https://crates.io/crates/istmo) plugin crate: an async Rust trait
(`src/lib.rs`) backed by native code (`native/`).
{% endif %}
Author: {{author_name}} <{{author_email}}>
License: {{crate_license}}

## Selected features

- Archetype: `{{kind}}`
- Platforms: {% if has_android %}android {% endif %}{% if has_ios %}ios {% endif %}{% if has_desktop %}desktop{% endif %}
- CI: {% if ci_github %}github-actions {% endif %}{% if ci_gitlab %}gitlab-ci{% endif %}
{%- if is_app %}
- Plugins: {% if plugin_app_lifecycle %}AppLifecycle {% endif %}{% if plugin_deep_links %}DeepLinks {% endif %}{% if plugin_safe_area %}SafeArea {% endif %}{% if plugin_google_sign_in %}istmo-google-sign-in {% endif %}{% if plugin_data_store %}istmo-data-store{% endif %}
- UI framework: `{{ui_framework}}` (see `src/app.rs`)
- Desktop packaging: {% if bundle_desktop %}nix-bundle-app (`nix build .#bundle`){% else %}not configured{% endif %}
{%- endif %}
- Publish targets: {% if publish_github %}github-release {% endif %}{% if publish_play %}play-store {% endif %}{% if publish_appstore %}app-store {% endif %}{% if publish_crates %}crates.io{% endif %}

Requires Rust 1.89 or newer.
{% if is_app %}
## Quickstart

```sh
cargo build         # also runs build.rs: Kotlin / Swift glue, istmo doctor
{% if has_desktop %}just run            # desktop shell
{% endif %}{% if has_android %}just build-android  # debug APK (Gradle runs cargo for every ABI)
{% endif %}{% if has_ios %}just build-ios      # xcodegen + simulator build (Xcode runs cargo)
{% endif %}{% if bundle_desktop %}just bundle-desktop # deb/rpm/appimage/dmg/msi via nix-bundle-app
{% endif %}```

## How it fits together

- `istmo.toml` is the single source of truth for the app id, name,
  version, icon and minimum OS versions.
- `build.rs` (`istmo_build::emit()`) runs on every cargo build. It reads
  `istmo.toml` and the manifest of every istmo plugin crate the app
  depends on, and generates the native glue: dispatchers, codecs and an
  `IstmoPluginRegistry` that registers every plugin backend. Warnings
  prefixed `istmo doctor:` point at setup mistakes.
- `src/lib.rs` declares the runtime (`istmo::runtime!`) and the
  `#[istmo::mobile_app]` entry point. Plugin crates are wired
  automatically; built-in `istmo::plugins` clients are listed there.
- `src/app.rs` holds the shared app state and UI.
{%- if has_desktop %}
- `src/main.rs` + `src/desktop.rs` run the same app on an in-process runtime;
  plugins with a desktop backend run in-process, calls to the others fail
  with a log line until you emulate them.
{%- endif %}
{%- if has_android %}
- `android/` is a plain Gradle project. The `io.github.sergioribera.istmo`
  plugin builds the crate for every ABI, links the plugins' Kotlin sources,
  manifests and dependencies, and applies `[app]`. The launcher activity is
  `dev.istmo.runtime.IstmoGameActivity`; subclass it to add Kotlin code.
  The repo ships no Gradle wrapper: run `gradle wrapper` once in
  `android/`, and `./gradlew istmoDoctor` to check the toolchain.
{%- endif %}
{%- if has_ios %}
- `ios/` is an xcodegen project. Run one `cargo build` (or `cargo check`)
  before `xcodegen generate`: it writes `ios/{{project-name | pascal_case}}App/istmo-plugins.yml`
  (plugin sources, app identity, link settings, the cargo pre-build phase)
  and `ios/.istmo/`. `main.swift` is `IstmoApp.run()`.
{%- endif %}

Adding a plugin crate is a `cargo add istmo-<plugin>`: the next build
wires its client in Rust and registers its native backend.
{% else %}
## Layout

- `src/lib.rs` — the `#[istmo::plugin]` trait, its config and error types.
  The macro emits `{{crate_name | pascal_case}}Client` (Rust callers) and
  `{{crate_name | pascal_case}}Host` (a Rust implementation, e.g. for
  desktop).
- `istmo.toml` — plugin id, client type, native backends and native
  dependencies. `build.rs` (`istmo_build::emit()`) hands it, plus the
  contract extracted from `src/lib.rs`, to every app that depends on this
  crate.
{%- if has_android %}
- `native/android/` — the reference Android backend, in the Kotlin
  package `android_package` names. The `io.github.sergioribera.istmo`
  Gradle plugin compiles it into every app that depends on this crate,
  next to the dispatcher and types the app's build generates into the
  same package.
{%- endif %}
{%- if has_ios %}
- `native/ios/` — the reference iOS backend, added to the app target by
  the generated `istmo-plugins.yml` xcodegen fragment.
{%- endif %}
{%- if has_android or has_ios %}
- Apps register the backends through their generated
  `IstmoPluginRegistry`; set `auto_register = false` in `istmo.toml` when
  each app has to construct the backend itself.
{%- endif %}
{%- unless has_android and has_ios %}
- Platforms without a reference backend: the app implements the
  generated `{{crate_name | pascal_case}}Factory` as
  `{{crate_name | pascal_case}}FactoryImpl`, or hosts the trait in Rust
  with `{{crate_name | pascal_case}}Host`.
{%- endunless %}
- `tests/smoke.rs` — the client against an in-memory Rust host.

```sh
just test
just lint
```
{% endif %}
## Continuous integration
{% if ci_github %}
`.github/workflows/ci.yml` runs `cargo fmt --check`, `cargo clippy` and
`cargo test`, plus an MSRV check{% if is_app and has_android %}, an Android APK build{% endif %}{% if is_app and has_ios %}, an iOS simulator build{% endif %}. Release
workflows fire on `v*` tags.
{% endif %}{% if ci_gitlab %}
`.gitlab-ci.yml` runs `cargo fmt --check`, `cargo clippy` and
`cargo test` on the MSRV image{% if is_app and has_android %}, builds the debug APK{% endif %}{% if is_app and has_ios %}, builds the iOS app on a macOS runner{% endif %}{% if bundle_desktop %}, builds the desktop bundle{% endif %}{% if publish_crates %}, and publishes to crates.io on `v*` tags{% endif %}.
{% endif %}
