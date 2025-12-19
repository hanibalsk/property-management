plugins {
    // Kotlin Multiplatform
    kotlin("multiplatform") version "1.9.21" apply false
    kotlin("plugin.serialization") version "1.9.21" apply false

    // Android
    id("com.android.application") version "8.2.0" apply false
    id("com.android.library") version "8.2.0" apply false
}

allprojects {
    group = "com.propertymanagement"
    version = "0.1.0"
}

tasks.register("clean", Delete::class) {
    delete(rootProject.layout.buildDirectory)
}
