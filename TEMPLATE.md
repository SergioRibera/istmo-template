# istmo template

```sh
cargo generate --git https://github.com/SergioRibera/istmo-template --name my-app
```

Requires `cargo-generate` 0.21+. Generated projects need Rust 1.89+.

The template asks:

| Prompt | Purpose |
|--------|---------|
| `kind` | `app` (multi-platform app) or `plugin` (istmo plugin crate) |
| `platforms` | android / ios / desktop |
| `plugins` | app only: built-in `istmo::plugins` clients (`app_lifecycle`, `deep_links`, `safe_area`) and plugin crates (`istmo-google-sign-in`, `istmo-data-store`, `istmo-biometric`, `istmo-share`, `istmo-file-picker`, `istmo-opener`, `istmo-camera`, `istmo-barcode-scanner`); the desktop shell registers each crate's Rust desktop host where it ships one (all but sign-in and share) |
| `publish_targets` | github-release / play-store / app-store / crates.io (plugins only) |
| `ci` | github-actions and/or gitlab-ci |
| `use_nix_bundle` | `nix-bundle-app` flake output for desktop packaging |
| `crate_license` | MIT / Apache-2.0 / dual / GPL-3.0 / Unlicense |
| `istmo_version` | one version for every istmo crate, first-party plugin, the Maven runtime + Gradle plugin and the Swift package tag |
| `ui_framework` | egui scaffold, or a plain shell for freya / slint / dioxus / none |

The pre-hook (`scripts/pre.rhai`) derives the flags the templates use;
the post-hook (`scripts/post.rhai`) prunes the unpicked archetype,
platforms and CI files, then flattens the chosen archetype into the
project root.

Generated apps use the `io.github.sergioribera.istmo` Gradle plugin and
`io.github.sergioribera:istmo-runtime` from Maven Central, and the
`IstmoRuntime` Swift package, all at `istmo_version` (the Maven
artefacts are published from the same release tag as the crates).

Plugin crates put their `native/android/` sources in the Kotlin package
`<bundle_id>.<crate name without underscores>` (`android_package` in
`istmo.toml`); consuming apps generate the plugin's dispatcher and types
into that package. A platform left out of `platforms` gets no reference
backend and no `android_backend` / `ios_backend` entry.

## Checking the template

Generate every variant non-interactively (`cargo generate --path . --name
my-app --template-values-file values.toml --silent --vcs none`) into a
scratch directory, point the istmo crates at a checkout with
`[patch.crates-io]` (`istmo`, `istmo-core`, `istmo-macros`,
`istmo-build`, `istmo-plugins`, `istmo-plugins-schema`, `istmo-window`,
`istmo-android`, `istmo-ios`, and the plugin crates), then run
`cargo clippy --all-targets --all-features -- -D warnings`, `cargo test`
and `cargo fmt --check` in it.

For a plugin, also build an app that depends on it: its `build.rs`
writes the plugin's Kotlin into `android/app/src/main/java/<package>/`
and its Swift into `ios/<App>/Plugins/<Trait>/Generated/`. Compiling
those with the plugin's `native/` sources and the istmo Kotlin runtime
(`kotlinc -Werror`) / Swift runtime (`swiftc -warnings-as-errors`, see
`runtime/ios/ci/` in istmo) checks the native side.
