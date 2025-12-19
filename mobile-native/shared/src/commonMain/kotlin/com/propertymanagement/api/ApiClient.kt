package com.propertymanagement.api

import io.ktor.client.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.client.request.*
import io.ktor.http.*
import io.ktor.serialization.kotlinx.json.*
import kotlinx.serialization.json.Json

/**
 * Property Management API Client.
 *
 * This client will be generated from OpenAPI spec using openapi-generator.
 * Run: `openapi-generator generate -i docs/api/generated/openapi.yaml -g kotlin -o mobile-native/shared/src/commonMain/kotlin/api`
 */
class ApiClient(
    private val baseUrl: String,
    private val accessToken: String? = null,
    private val tenantId: String? = null
) {
    private val client = HttpClient {
        install(ContentNegotiation) {
            json(Json {
                prettyPrint = true
                isLenient = true
                ignoreUnknownKeys = true
            })
        }
    }

    private fun HttpRequestBuilder.configureRequest() {
        accessToken?.let {
            header(HttpHeaders.Authorization, "Bearer $it")
        }
        tenantId?.let {
            header("X-Tenant-ID", it)
        }
        contentType(ContentType.Application.Json)
    }

    suspend fun healthCheck(): String {
        return client.get("$baseUrl/health") {
            configureRequest()
        }.toString()
    }

    // TODO: Add generated API methods from OpenAPI spec
}
