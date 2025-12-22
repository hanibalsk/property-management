package three.two.bit.ppt.reality.auth

import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.client.plugins.defaultRequest
import io.ktor.client.request.*
import io.ktor.http.*
import io.ktor.serialization.kotlinx.json.*
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.serialization.json.Json

/**
 * SSO Service for Reality Portal (Epic 10A-SSO).
 *
 * Handles mobile deep-link SSO flow:
 * 1. PM app calls createMobileToken() with PM access token
 * 2. PM app opens Reality Portal via deep-link: reality://sso?token=xxx
 * 3. Reality Portal validates token and creates session
 */
class SsoService(private val baseUrl: String = "http://localhost:8081") {
    private val json = Json {
        ignoreUnknownKeys = true
        isLenient = true
        encodeDefaults = true
    }

    private val client = HttpClient {
        install(ContentNegotiation) { json(json) }
        defaultRequest { contentType(ContentType.Application.Json) }
    }

    private val _authState = MutableStateFlow<AuthState>(AuthState.Unauthenticated)
    val authState: StateFlow<AuthState> = _authState.asStateFlow()

    private var sessionToken: String? = null

    /**
     * Create a mobile SSO token from PM access token.
     *
     * Call this from PM app to get a short-lived token for SSO.
     */
    suspend fun createMobileToken(pmAccessToken: String): Result<MobileSsoTokenResponse> {
        return try {
            val response =
                client.post("$baseUrl/api/v1/sso/mobile/token") {
                    setBody(CreateMobileSsoTokenRequest(pmAccessToken))
                }

            if (response.status.isSuccess()) {
                Result.success(response.body())
            } else {
                val error: SsoError = response.body()
                Result.failure(SsoException(error.error, error.errorDescription))
            }
        } catch (e: Exception) {
            Result.failure(e)
        }
    }

    /**
     * Validate SSO token from deep-link and create session.
     *
     * Call this when Reality Portal receives the deep-link.
     */
    suspend fun validateAndLogin(ssoToken: String): Result<SessionResponse> {
        _authState.value = AuthState.Loading

        return try {
            val response =
                client.post("$baseUrl/api/v1/sso/mobile/validate") {
                    setBody(ValidateMobileSsoTokenRequest(ssoToken))
                }

            if (response.status.isSuccess()) {
                val session: SessionResponse = response.body()
                sessionToken = session.sessionToken
                _authState.value = AuthState.Authenticated(session.user, session.sessionToken)
                Result.success(session)
            } else {
                val error: SsoError = response.body()
                _authState.value = AuthState.Error(error.errorDescription ?: error.error)
                Result.failure(SsoException(error.error, error.errorDescription))
            }
        } catch (e: Exception) {
            _authState.value = AuthState.Error(e.message ?: "Unknown error")
            Result.failure(e)
        }
    }

    /** Get current session information. */
    suspend fun getSession(): Result<SessionInfo> {
        val token =
            sessionToken ?: return Result.failure(SsoException("no_session", "Not authenticated"))

        return try {
            val response =
                client.get("$baseUrl/api/v1/sso/session") {
                    header(HttpHeaders.Authorization, "Bearer $token")
                }

            if (response.status.isSuccess()) {
                Result.success(response.body())
            } else {
                if (response.status == HttpStatusCode.Unauthorized) {
                    logout()
                }
                val error: SsoError = response.body()
                Result.failure(SsoException(error.error, error.errorDescription))
            }
        } catch (e: Exception) {
            Result.failure(e)
        }
    }

    /** Refresh the current session. */
    suspend fun refreshSession(): Result<SessionInfo> {
        val token =
            sessionToken ?: return Result.failure(SsoException("no_session", "Not authenticated"))

        return try {
            val response =
                client.post("$baseUrl/api/v1/sso/refresh") {
                    header(HttpHeaders.Authorization, "Bearer $token")
                }

            if (response.status.isSuccess()) {
                Result.success(response.body())
            } else {
                if (response.status == HttpStatusCode.Unauthorized) {
                    logout()
                }
                val error: SsoError = response.body()
                Result.failure(SsoException(error.error, error.errorDescription))
            }
        } catch (e: Exception) {
            Result.failure(e)
        }
    }

    /** Logout and clear session. */
    fun logout() {
        sessionToken = null
        _authState.value = AuthState.Unauthenticated
    }

    /** Check if user is authenticated. */
    fun isAuthenticated(): Boolean = sessionToken != null

    /** Get the current session token for API calls. */
    fun getSessionToken(): String? = sessionToken

    /**
     * Restore session from stored token (e.g., from SharedPreferences/UserDefaults).
     *
     * Call this on app startup with the stored token.
     */
    suspend fun restoreSession(token: String): Boolean {
        sessionToken = token
        _authState.value = AuthState.Loading

        return getSession()
            .fold(
                onSuccess = { session ->
                    _authState.value =
                        AuthState.Authenticated(
                            user =
                                SsoUserInfo(
                                    userId = session.userId,
                                    email = session.email,
                                    name = session.name
                                ),
                            sessionToken = token
                        )
                    true
                },
                onFailure = {
                    sessionToken = null
                    _authState.value = AuthState.Unauthenticated
                    false
                }
            )
    }
}

/** SSO-specific exception. */
class SsoException(val error: String, val errorDescription: String?) :
    Exception(errorDescription ?: error)
