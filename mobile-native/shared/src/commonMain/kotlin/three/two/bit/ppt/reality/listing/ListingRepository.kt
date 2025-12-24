package three.two.bit.ppt.reality.listing

import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.client.plugins.defaultRequest
import io.ktor.client.plugins.logging.*
import io.ktor.client.request.*
import io.ktor.http.*
import io.ktor.serialization.kotlinx.json.*
import kotlinx.serialization.json.Json

/**
 * Repository for listing operations.
 *
 * Epic 48 - Story 48.1: Portal Mobile Search
 */
class ListingRepository(
    private val baseUrl: String = "http://localhost:8081",
    private val sessionToken: String? = null
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
        sessionToken?.let { header(HttpHeaders.Authorization, "Bearer $it") }
    }

    /** Search listings with filters and pagination. */
    suspend fun searchListings(request: ListingSearchRequest): Result<ListingSearchResponse> {
        return try {
            val response =
                client.post("$baseUrl/api/v1/listings/search") {
                    configureRequest()
                    setBody(request)
                }

            if (response.status.isSuccess()) {
                Result.success(response.body())
            } else {
                Result.failure(ListingException("Search failed: ${response.status}"))
            }
        } catch (e: Exception) {
            Result.failure(e)
        }
    }

    /** Get listing details by ID. */
    suspend fun getListingDetail(listingId: String): Result<ListingDetail> {
        return try {
            val response = client.get("$baseUrl/api/v1/listings/$listingId") { configureRequest() }

            if (response.status.isSuccess()) {
                Result.success(response.body())
            } else if (response.status == HttpStatusCode.NotFound) {
                Result.failure(ListingException("Listing not found"))
            } else {
                Result.failure(ListingException("Failed to load listing: ${response.status}"))
            }
        } catch (e: Exception) {
            Result.failure(e)
        }
    }

    /** Get featured listings for homepage. */
    suspend fun getFeaturedListings(): Result<FeaturedListingsResponse> {
        return try {
            val response = client.get("$baseUrl/api/v1/listings/featured") { configureRequest() }

            if (response.status.isSuccess()) {
                Result.success(response.body())
            } else {
                Result.failure(ListingException("Failed to load featured: ${response.status}"))
            }
        } catch (e: Exception) {
            Result.failure(e)
        }
    }

    /** Get recent listings. */
    suspend fun getRecentListings(limit: Int = 10): Result<RecentListingsResponse> {
        return try {
            val response =
                client.get("$baseUrl/api/v1/listings/recent") {
                    configureRequest()
                    parameter("limit", limit)
                }

            if (response.status.isSuccess()) {
                Result.success(response.body())
            } else {
                Result.failure(ListingException("Failed to load recent: ${response.status}"))
            }
        } catch (e: Exception) {
            Result.failure(e)
        }
    }

    /** Get search suggestions based on partial query. */
    suspend fun getSearchSuggestions(query: String): Result<SearchSuggestionsResponse> {
        return try {
            val response =
                client.get("$baseUrl/api/v1/listings/suggestions") {
                    configureRequest()
                    parameter("q", query)
                }

            if (response.status.isSuccess()) {
                Result.success(response.body())
            } else {
                Result.failure(ListingException("Failed to load suggestions: ${response.status}"))
            }
        } catch (e: Exception) {
            Result.failure(e)
        }
    }

    /** Get listings near a location. */
    suspend fun getListingsNearby(
        latitude: Double,
        longitude: Double,
        radiusKm: Double = 10.0,
        limit: Int = 20
    ): Result<ListingSearchResponse> {
        return try {
            val response =
                client.get("$baseUrl/api/v1/listings/nearby") {
                    configureRequest()
                    parameter("lat", latitude)
                    parameter("lng", longitude)
                    parameter("radius_km", radiusKm)
                    parameter("limit", limit)
                }

            if (response.status.isSuccess()) {
                Result.success(response.body())
            } else {
                Result.failure(ListingException("Failed to load nearby: ${response.status}"))
            }
        } catch (e: Exception) {
            Result.failure(e)
        }
    }
}

/** Listing-specific exception. */
class ListingException(message: String) : Exception(message)
