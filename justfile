set shell := ["bash", "-cu"]

# Desktop build (host toolchain).
build:
    cargo build --release

test:
    cargo test --all-targets

fmt:
    cargo fmt --all
    cargo clippy --all-targets -- -D warnings

{% if has_android %}
# Android — assemble debug APK via Gradle, which drives cargo for every ABI.
build-android:
    cd android && ./gradlew assembleDebug

# Play Store internal release bundle. Requires ANDROID_RELEASE_* env.
release-android:
    cd android && ./gradlew bundleRelease
{% endif %}

{% if has_ios %}
# iOS — regenerate the .xcodeproj (idempotent) and build for the simulator.
build-ios:
    cd ios && xcodegen generate && \
        xcodebuild -project {{project-name | pascal_case}}App.xcodeproj \
                   -scheme  {{project-name | pascal_case}}App \
                   -destination 'generic/platform=iOS Simulator' build
{% endif %}

{% if bundle_desktop %}
# Desktop distribution bundles via nix-bundle-app.
bundle-desktop:
    nix build .#bundle
{% endif %}

# Run the desktop shell (uses the mock runtime + emulator thread).
run:
    cargo run
