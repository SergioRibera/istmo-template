fn main() {
    // Hands `istmo.toml` (`DEP_*_ISTMO_MANIFEST`) and the contract parsed
    // from `src/lib.rs` (`DEP_*_ISTMO_CONTRACTS`) to the app, whose own
    // `emit()` generates the Kotlin / Swift glue from them.
    istmo_build::emit();
}
