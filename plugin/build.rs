fn main() {
    // Emits `istmo.toml` (`DEP_*_ISTMO_MANIFEST`) + the parsed
    // `Contract` (`DEP_*_ISTMO_CONTRACT`) so downstream apps can wire
    // codegen without hand-authoring a builder.
    istmo_build::emit();
}
