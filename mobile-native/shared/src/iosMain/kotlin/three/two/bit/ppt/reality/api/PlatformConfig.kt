package three.two.bit.ppt.reality.api

import platform.Foundation.NSBundle

/**
 * iOS implementation of PlatformConfig.
 *
 * Reads configuration values from Info.plist which are set by xcconfig files for each build scheme.
 *
 * Epic 85 - Story 85.1: Environment Variable Setup
 */
actual object PlatformConfig {
    /**
     * Base URL for the Reality Portal API.
     *
     * Read from Info.plist API_BASE_URL key, set by xcconfig: - Development.xcconfig:
     * http://localhost:8081 - Staging.xcconfig: https://staging-reality.ppt.example.com -
     * Release.xcconfig: https://reality.ppt.example.com
     */
    actual val baseUrl: String
        get() =
            NSBundle.mainBundle.objectForInfoDictionaryKey("API_BASE_URL") as? String
                ?: "https://reality.ppt.example.com"

    /**
     * Current environment: development, staging, or production.
     *
     * Read from Info.plist ENVIRONMENT key, set by xcconfig.
     */
    actual val environment: String
        get() =
            NSBundle.mainBundle.objectForInfoDictionaryKey("ENVIRONMENT") as? String ?: "production"

    /**
     * Whether debug mode is enabled.
     *
     * Determined by environment: true for development and staging, false for production.
     */
    actual val isDebug: Boolean
        get() = environment != "production"

    /**
     * Whether logging is enabled.
     *
     * Read from Info.plist ENABLE_LOGGING key, set by xcconfig. Defaults to true for non-production
     * environments.
     */
    actual val enableLogging: Boolean
        get() =
            (NSBundle.mainBundle.objectForInfoDictionaryKey("ENABLE_LOGGING") as? String)
                ?.lowercase() == "true" || isDebug
}
