# {{project-name}}

{{description}}

{% if is_app %}
Multi-platform application built on [istmo](https://crates.io/crates/istmo).
{% else %}
[istmo](https://crates.io/crates/istmo) plugin crate.
{% endif %}

Author: {{author_name}} <{{author_email}}>
License: {{crate_license}}

## Selected features

- Archetype: `{{kind}}`
- Platforms: {% if has_android %}android {% endif %}{% if has_ios %}ios {% endif %}{% if has_desktop %}desktop{% endif %}
- CI: {% if ci_github %}github-actions {% endif %}{% if ci_gitlab %}gitlab-ci{% endif %}
{% if is_app %}
- Bundled plugins: {% if plugin_app_lifecycle %}AppLifecycle {% endif %}{% if plugin_deep_links %}DeepLinks {% endif %}{% if plugin_safe_area %}SafeAreaPublisher {% endif %}{% if plugin_google_sign_in %}google-sign-in {% endif %}{% if plugin_data_store %}data-store{% endif %}
- UI framework hint: `{{ui_framework}}` (see comments in `src/app.rs`)
- Desktop packaging: {% if bundle_desktop %}nix-bundle-app (`nix build .#bundle`){% else %}not configured{% endif %}
{% endif %}
- Publish targets: {% if publish_github %}github-release {% endif %}{% if publish_play %}play-store {% endif %}{% if publish_appstore %}app-store {% endif %}{% if publish_crates %}crates.io{% endif %}

## Quickstart

```sh
just build          # cargo build for the current host
{% if has_android %}just build-android  # cargo build for aarch64-linux-android + Gradle assemble
{% endif %}{% if has_ios %}just build-ios      # cargo build for aarch64-apple-ios + xcodegen + xcodebuild
{% endif %}{% if bundle_desktop %}just bundle-desktop # nix build .#bundle (deb/rpm/appimage/dmg/msi via nix-bundle-app)
{% endif %}```

## Wiring notes

{% if is_app %}
See `src/lib.rs` for the `istmo::runtime!` macro invocation and the commented
placeholder where extra plugin clients get registered. `src/app.rs` is the
shared UI shell — replace the `{{ui_framework}}` scaffold if you picked a
different toolkit.
{% else %}
See `src/lib.rs` for the plugin trait, config, and error types. The reference
native implementations live under `native/{android,ios}/` and are shipped in the
crate's published artifact via `include_str!`.
{% endif %}

## Continuous integration

{% if ci_github %}
`.github/workflows/ci.yml` runs `cargo test`, `cargo fmt --check`, `cargo clippy`
on every push. `release.yml` fires on tags matching `v*`.
{% endif %}
{% if ci_gitlab %}
`.gitlab-ci.yml` mirrors the GitHub matrix.
{% endif %}
