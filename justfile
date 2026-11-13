set shell := ["bash", "-cu"]

# Host build. Also runs build.rs, which regenerates the Kotlin / Swift glue.
build:
    cargo build --release

test:
    cargo test --all-targets

fmt:
    cargo fmt --all

lint:
    cargo fmt --all --check
    cargo clippy --all-targets -- -D warnings
{% if is_app %}{% if has_android %}
# Android debug APK. The `io.github.sergioribera.istmo` Gradle plugin runs
# cargo for every ABI; `./gradlew istmoDoctor` checks the toolchain.
build-android:
    cd android && ./gradlew assembleDebug

# Play Store release bundle. Requires the ANDROID_RELEASE_* env vars.
release-android:
    cd android && ./gradlew bundleRelease
{% endif %}{% if has_ios %}
# iOS simulator build. `cargo check` writes the xcodegen fragment
# (`istmo-plugins.yml`) before the project is generated; Xcode's pre-build
# phase (`ios/.istmo/build-rust.sh`) then builds the Rust library.
build-ios:
    cargo check
    cd ios && xcodegen generate && \
        xcodebuild -project {{project-name | pascal_case}}App.xcodeproj \
                   -scheme  {{project-name | pascal_case}}App \
                   -destination 'generic/platform=iOS Simulator' build
{% endif %}{% if bundle_desktop %}
# Desktop distribution bundles via nix-bundle-app.
bundle-desktop:
    nix build .#bundle
{% endif %}{% if has_desktop %}
# Run the desktop shell (in-process runtime + Rust hosts / emulator thread).
run:
    cargo run
{% endif %}{% endif %}
