package three.two.bit.ppt.reality.api

/**
 * API configuration for Reality Portal.
 *
 * This object provides centralized configuration for API endpoints using the expect/actual pattern
 * for platform-specific implementations.
 *
 * Epic 48 - Code Review Fix: Remove hardcoded localhost URL Epic 85 - Story 85.1: Environment
 * Variable Setup
 */
object ApiConfig {
    /**
     * Base URL for the Reality Portal API.
     *
     * This is provided by the platform-specific implementation: - Android: Read from BuildConfig -
     * iOS: Read from Info.plist
     */
    val baseUrl: String
        get() = PlatformConfig.baseUrl

    /**
     * WebSocket URL for real-time communication.
     *
     * Derived from baseUrl by replacing http(s) with ws(s).
     */
    val wsUrl: String
        get() = baseUrl.replace("http", "ws")

    /** Current environment: development, staging, or production. */
    val environment: String
        get() = PlatformConfig.environment

    /** Whether debug mode is enabled. */
    val isDebug: Boolean
        get() = PlatformConfig.isDebug

    /** Whether logging is enabled. */
    val enableLogging: Boolean
        get() = PlatformConfig.enableLogging

    /** Check if the API configuration has been initialized. */
    val isInitialized: Boolean
        get() = baseUrl.isNotBlank()

    /** Get the base URL, throwing if not initialized. */
    fun requireBaseUrl(): String {
        check(isInitialized) { "ApiConfig not initialized. baseUrl is blank." }
        return baseUrl
    }
}

/**
 * Platform-specific configuration provider.
 *
 * This uses the expect/actual pattern to provide configuration values from: - Android: BuildConfig
 * fields set by Gradle product flavors - iOS: Info.plist values set by xcconfig files
 */
expect object PlatformConfig {
    /** Base URL for the Reality Portal API. */
    val baseUrl: String

    /** Current environment: development, staging, or production. */
    val environment: String

    /** Whether debug mode is enabled. */
    val isDebug: Boolean

    /** Whether logging is enabled. */
    val enableLogging: Boolean
}
