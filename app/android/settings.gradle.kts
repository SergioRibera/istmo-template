pluginManagement {
    repositories {
        google()
        mavenCentral()
        gradlePluginPortal()
        // `dev.istmo.plugin-loader` is published to GitHub Packages
        // alongside `dev.istmo:istmo-runtime`. Uses the same creds
        // as the dependency repo declared below.
        maven {
            url = uri("https://maven.pkg.github.com/SergioRibera/istmo")
            credentials {
                username = System.getenv("GITHUB_ACTOR")
                    ?: providers.gradleProperty("gpr.user").orNull
                password = System.getenv("GITHUB_TOKEN")
                    ?: providers.gradleProperty("gpr.key").orNull
            }
        }
    }
    resolutionStrategy {
        eachPlugin {
            if (requested.id.id == "dev.istmo.plugin-loader") {
                useModule("dev.istmo:istmo-plugin-loader:${requested.version}")
            }
        }
    }
}

dependencyResolutionManagement {
    repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)
    repositories {
        google()
        mavenCentral()
        // istmo-runtime is published to GitHub Packages. Provide creds via
        // env vars (CI: GITHUB_ACTOR / GITHUB_TOKEN) or gradle.properties.
        maven {
            url = uri("https://maven.pkg.github.com/SergioRibera/istmo")
            credentials {
                username = System.getenv("GITHUB_ACTOR")
                    ?: providers.gradleProperty("gpr.user").orNull
                password = System.getenv("GITHUB_TOKEN")
                    ?: providers.gradleProperty("gpr.key").orNull
            }
        }
    }
}

rootProject.name = "{{project-name}}"
include(":app")
