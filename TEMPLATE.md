# istmo template

Usage:

```sh
cargo generate --git https://github.com/{{org-placeholder}}/istmo-template \
               --name my-app
```

The template asks:

| Prompt | Purpose |
|--------|---------|
| `kind` | `app` (multi-platform binary) or `plugin` (published istmo-* crate) |
| `platforms` | Multi-select android / ios / desktop |
| `plugins` | Which pre-existing istmo plugins to wire into the app scaffold |
| `publish_targets` | github-release / play-store / app-store / crates.io |
| `ci` | github-actions and/or gitlab-ci pipelines |
| `use_nix_bundle` | Whether to include `nix-bundle-app` flake output for desktop packaging |
| `license` | MIT / Apache-2.0 / dual / GPL-3.0 / Unlicense |
| `ui_framework` | egui / freya / slint / dioxus / none (only affects the scaffold in `src/app.rs`) |

After generation, the tree contains only the files relevant to the choices —
the rhai post-hook prunes unused archetype / platform / CI branches.
