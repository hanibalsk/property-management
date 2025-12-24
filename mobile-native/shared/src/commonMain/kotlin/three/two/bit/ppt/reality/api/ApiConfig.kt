package three.two.bit.ppt.reality.api

/**
 * API configuration for Reality Portal.
 *
 * This object provides centralized configuration for API endpoints. The baseUrl should be set at
 * application startup based on build configuration.
 *
 * Epic 48 - Code Review Fix: Remove hardcoded localhost URL
 */
object ApiConfig {
    /**
     * Base URL for the Reality Portal API.
     *
     * IMPORTANT: This must be set at application startup before making API calls. In production,
     * this should use HTTPS.
     *
     * Example:
     * - Development: "http://10.0.2.2:8081" (Android emulator)
     * - Production: "https://api.realityportal.example.com"
     */
    var baseUrl: String = ""
        private set

    /**
     * Initialize the API configuration.
     *
     * @param baseUrl The base URL for the Reality Portal API (must use HTTPS in production)
     */
    fun initialize(baseUrl: String) {
        require(baseUrl.isNotBlank()) { "baseUrl must not be blank" }
        this.baseUrl = baseUrl
    }

    /** Check if the API configuration has been initialized. */
    val isInitialized: Boolean
        get() = baseUrl.isNotBlank()

    /** Get the base URL, throwing if not initialized. */
    fun requireBaseUrl(): String {
        check(isInitialized) { "ApiConfig not initialized. Call ApiConfig.initialize() first." }
        return baseUrl
    }
}
