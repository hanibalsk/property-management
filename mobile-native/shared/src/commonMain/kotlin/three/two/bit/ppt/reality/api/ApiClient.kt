package three.two.bit.ppt.reality.api

import io.ktor.client.request.*
import io.ktor.http.*

/**
 * Reality Portal API Client.
 *
 * This client will be generated from OpenAPI spec using openapi-generator. Run: `openapi-generator
 * generate -i docs/api/generated/by-service/reality-server.yaml -g kotlin -o
 * mobile-native/shared/src/commonMain/kotlin/api`
 *
 * Note: Uses shared HttpClientProvider to avoid resource leaks (Epic 48 - Code Review Fix).
 */
class ApiClient(
    private val baseUrl: String = ApiConfig.requireBaseUrl(),
    private val accessToken: String? = null,
    private val tenantId: String? = null
) {
    private val client = HttpClientProvider.client

    private fun HttpRequestBuilder.configureRequest() {
        accessToken?.let { header(HttpHeaders.Authorization, "Bearer $it") }
        tenantId?.let { header("X-Tenant-ID", it) }
    }

    suspend fun healthCheck(): String {
        return client.get("$baseUrl/health") { configureRequest() }.toString()
    }

    // TODO: Add generated API methods from OpenAPI spec
}
