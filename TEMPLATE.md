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
| `plugins` | app only: built-in `istmo::plugins` clients (`app_lifecycle`, `deep_links`, `safe_area`) and plugin crates (`istmo-google-sign-in`, `istmo-data-store`) |
| `publish_targets` | github-release / play-store / app-store / crates.io (plugins only) |
| `ci` | github-actions and/or gitlab-ci |
| `use_nix_bundle` | `nix-bundle-app` flake output for desktop packaging |
| `crate_license` | MIT / Apache-2.0 / dual / GPL-3.0 / Unlicense |
| `istmo_version` | version of every istmo crate and first-party plugin (they share one) |
| `ui_framework` | egui scaffold, or a plain shell for freya / slint / dioxus / none |

The pre-hook (`scripts/pre.rhai`) derives the flags the templates use;
the post-hook (`scripts/post.rhai`) prunes the unpicked archetype,
platforms and CI files, then flattens the chosen archetype into the
project root.

Generated apps use the `io.github.sergioribera.istmo` Gradle plugin and
`io.github.sergioribera:istmo-runtime` from Maven Central (versioned
apart from the crates, pinned in `android/`), and the `IstmoRuntime`
Swift package at the `istmo_version` tag.

## Checking the template

Generate into a scratch directory, point the istmo crates at a checkout
with `[patch.crates-io]` (`istmo`, `istmo-core`, `istmo-macros`,
`istmo-build`, `istmo-plugins`, `istmo-android`, `istmo-ios`, and the
plugin crates), then run `cargo clippy --all-targets -- -D warnings`,
`cargo test` and `cargo fmt --check` in it.
