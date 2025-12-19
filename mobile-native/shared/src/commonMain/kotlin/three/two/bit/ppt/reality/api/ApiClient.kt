package three.two.bit.ppt.reality.api

import io.ktor.client.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.client.plugins.defaultRequest
import io.ktor.client.plugins.logging.*
import io.ktor.client.request.*
import io.ktor.http.*
import io.ktor.serialization.kotlinx.json.*
import kotlinx.serialization.json.Json

/**
 * Reality Portal API Client.
 *
 * This client will be generated from OpenAPI spec using openapi-generator. Run: `openapi-generator
 * generate -i docs/api/generated/by-service/reality-server.yaml -g kotlin -o
 * mobile-native/shared/src/commonMain/kotlin/api`
 */
class ApiClient(
    private val baseUrl: String,
    private val accessToken: String? = null,
    private val tenantId: String? = null
) {
    private val json = Json {
        ignoreUnknownKeys = true
        isLenient = true
        encodeDefaults = true
        prettyPrint = false
    }

    private val client = HttpClient {
        install(ContentNegotiation) { json(json) }

        install(Logging) { level = LogLevel.HEADERS }

        defaultRequest { contentType(ContentType.Application.Json) }
    }

    private fun HttpRequestBuilder.configureRequest() {
        accessToken?.let { header(HttpHeaders.Authorization, "Bearer $it") }
        tenantId?.let { header("X-Tenant-ID", it) }
    }

    suspend fun healthCheck(): String {
        return client.get("$baseUrl/health") { configureRequest() }.toString()
    }

    // TODO: Add generated API methods from OpenAPI spec
}
