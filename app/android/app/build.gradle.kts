plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
    // Builds the Rust crate for every ABI and build type, links the istmo
    // plugins it depends on (Kotlin sources, manifests, resources, Gradle
    // dependencies) and applies istmo.toml `[app]` (applicationId,
    // version, minSdk, label, icon).
    id("io.github.sergioribera.istmo")
}

android {
    namespace  = "{{bundle_id}}"
    compileSdk = 34

    defaultConfig {
        targetSdk = 34
        ndk { abiFilters += setOf("arm64-v8a", "x86_64") }
    }

    signingConfigs {
        getByName("debug") {
            storeFile     = file(System.getenv("ANDROID_DEBUG_KEYSTORE")
                ?: "${System.getProperty("user.home")}/.android/debug.keystore")
            storePassword = "android"
            keyAlias      = "androiddebugkey"
            keyPassword   = "android"
        }
{%- if publish_play %}
        // Play Store release signing — supply the keystore via CI secrets.
        create("release") {
            val keystorePath = System.getenv("ANDROID_RELEASE_KEYSTORE")
            if (keystorePath != null) {
                storeFile     = file(keystorePath)
                storePassword = System.getenv("ANDROID_RELEASE_STORE_PASSWORD")
                keyAlias      = System.getenv("ANDROID_RELEASE_KEY_ALIAS")
                keyPassword   = System.getenv("ANDROID_RELEASE_KEY_PASSWORD")
            }
        }
{%- endif %}
    }

    buildTypes {
        getByName("debug") { signingConfig = signingConfigs.getByName("debug") }
        getByName("release") {
            isMinifyEnabled = false
{%- if publish_play %}
            signingConfig = signingConfigs.getByName("release")
{%- endif %}
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
    kotlinOptions { jvmTarget = "17" }
    packaging { jniLibs { useLegacyPackaging = false } }
}

dependencies {
    // Kotlin runtime; shares its version with the istmo crates and the Gradle plugin.
    implementation("io.github.sergioribera:istmo-runtime:{{istmo_version}}")
    // `IstmoGameActivity` extends GameActivity.
    implementation("androidx.games:games-activity:4.4.0")
    implementation("androidx.core:core-ktx:1.13.1")
    implementation("androidx.appcompat:appcompat:1.7.0")
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-android:1.8.1")
}
