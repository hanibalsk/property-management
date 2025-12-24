plugins {
    alias(libs.plugins.android.application)
    alias(libs.plugins.kotlin.android)
    alias(libs.plugins.kotlin.compose)
}

// Read version from VERSION file (single source of truth)
val versionFile = rootProject.file("../VERSION")
val appVersion =
    if (versionFile.exists()) {
        versionFile.readText().trim()
    } else {
        "0.1.0" // Fallback version
    }

// Calculate versionCode: MAJOR * 10000 + MINOR * 100 + PATCH
val versionParts = appVersion.split(".")
val calculatedVersionCode =
    versionParts[0].toInt() * 10000 + versionParts[1].toInt() * 100 + versionParts[2].toInt()

android {
    namespace = "three.two.bit.ppt.reality"
    compileSdk = libs.versions.compileSdk.get().toInt()

    defaultConfig {
        applicationId = "three.two.bit.ppt.reality"
        minSdk = libs.versions.minSdk.get().toInt()
        targetSdk = libs.versions.targetSdk.get().toInt()
        versionCode = calculatedVersionCode
        versionName = appVersion

        // API base URL - use emulator localhost for debug, configure for production
        buildConfigField("String", "API_BASE_URL", "\"http://10.0.2.2:8081\"")
    }

    buildTypes {
        release {
            isMinifyEnabled = true
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
            // Production API URL - must use HTTPS
            buildConfigField("String", "API_BASE_URL", "\"https://api.realityportal.example.com\"")
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    kotlinOptions { jvmTarget = "17" }

    buildFeatures {
        compose = true
        buildConfig = true
    }
}

dependencies {
    implementation(project(":shared"))

    // Compose BOM
    implementation(platform(libs.compose.bom))
    implementation(libs.compose.ui)
    implementation(libs.compose.material3)
    implementation(libs.compose.material.icons.extended)
    implementation(libs.compose.ui.tooling.preview)
    debugImplementation(libs.compose.ui.tooling)

    // AndroidX
    implementation(libs.androidx.activity.compose)
    implementation(libs.androidx.lifecycle.runtime.compose)
    implementation(libs.androidx.navigation.compose)

    // Coroutines
    implementation(libs.kotlinx.coroutines.android)

    // Ktor (needed for repository default parameters)
    implementation(libs.ktor.client.core)
    implementation(libs.ktor.client.android)

    // Image loading
    implementation(libs.coil.compose)

    // Location services
    implementation(libs.play.services.location)

    // DataStore for preferences
    implementation(libs.datastore.preferences)
}
