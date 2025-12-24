package three.two.bit.ppt.reality.api

import io.ktor.client.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.client.plugins.defaultRequest
import io.ktor.client.plugins.logging.*
import io.ktor.http.*
import io.ktor.serialization.kotlinx.json.*
import kotlinx.serialization.json.Json

/**
 * Shared HttpClient provider to avoid resource leaks.
 *
 * Creates a single HttpClient instance that can be shared across repositories. This prevents
 * connection pool exhaustion and memory leaks from creating multiple HttpClient instances.
 *
 * Lifecycle management:
 * - The HttpClient is created lazily on first access
 * - Call close() when the application is shutting down to properly release resources
 * - On Android: Consider calling close() in Application.onTerminate() or MainActivity.onDestroy()
 * - On iOS: Consider calling close() in the app delegate's applicationWillTerminate
 *
 * Epic 48 - Code Review Fix: HttpClient resource management
 */
object HttpClientProvider {
    private val json = Json {
        ignoreUnknownKeys = true
        isLenient = true
        encodeDefaults = true
        prettyPrint = false
    }

    /**
     * Shared HttpClient instance for all API calls. This client is configured with JSON
     * serialization and minimal logging (NONE to avoid exposing sensitive headers in production).
     */
    val client: HttpClient by lazy {
        HttpClient {
            install(ContentNegotiation) { json(json) }
            install(Logging) { level = LogLevel.NONE }
            defaultRequest { contentType(ContentType.Application.Json) }
        }
    }

    /** Closes the shared HttpClient. Call this when the application is shutting down. */
    fun close() {
        client.close()
    }
}
