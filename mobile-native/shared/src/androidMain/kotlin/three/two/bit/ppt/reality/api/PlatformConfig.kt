package three.two.bit.ppt.reality.api

import three.two.bit.ppt.reality.shared.BuildConfig

/**
 * Android implementation of PlatformConfig.
 *
 * Reads configuration values from BuildConfig fields that are set by Gradle product flavors.
 *
 * Epic 85 - Story 85.1: Environment Variable Setup
 */
actual object PlatformConfig {
    /**
     * Base URL for the Reality Portal API.
     *
     * Set by buildConfigField in build.gradle.kts for each product flavor: - development:
     * http://10.0.2.2:8081 - staging: https://staging-reality.ppt.example.com - production:
     * https://reality.ppt.example.com
     */
    actual val baseUrl: String
        get() = BuildConfig.API_BASE_URL

    /**
     * Current environment: development, staging, or production.
     *
     * Set by buildConfigField in build.gradle.kts for each product flavor.
     */
    actual val environment: String
        get() = BuildConfig.ENVIRONMENT

    /**
     * Whether debug mode is enabled.
     *
     * True for debug builds, false for release builds.
     */
    actual val isDebug: Boolean
        get() = BuildConfig.DEBUG

    /**
     * Whether logging is enabled.
     *
     * Set by buildConfigField in build.gradle.kts for each product flavor.
     */
    actual val enableLogging: Boolean
        get() = BuildConfig.ENABLE_LOGGING
}
