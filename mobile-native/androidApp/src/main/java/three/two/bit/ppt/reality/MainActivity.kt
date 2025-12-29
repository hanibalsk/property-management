package three.two.bit.ppt.reality

import android.content.Intent
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.lightColorScheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.lifecycle.lifecycleScope
import androidx.navigation.compose.rememberNavController
import kotlinx.coroutines.launch
import three.two.bit.ppt.reality.api.ApiConfig
import three.two.bit.ppt.reality.auth.SsoService
import three.two.bit.ppt.reality.listing.ListingRepository
import three.two.bit.ppt.reality.navigation.RealityNavHost

/**
 * Main activity for Reality Portal mobile app.
 *
 * Epic 48: Reality Portal Mobile (KMP)
 */
class MainActivity : ComponentActivity() {
    private val ssoService = SsoService()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // API configuration is now provided by PlatformConfig using expect/actual pattern.
        // See: shared/src/androidMain/kotlin/.../api/PlatformConfig.kt
        // The API_BASE_URL is configured via Gradle product flavors in shared/build.gradle.kts:
        // - Development: http://10.0.2.2:8081 (Android emulator localhost)
        // - Staging: https://staging-api.realityportal.example.com
        // - Production: https://api.realityportal.example.com
        require(ApiConfig.isInitialized) { "ApiConfig not initialized - check PlatformConfig" }

        // Handle deep-link on initial launch
        handleDeepLink(intent)

        setContent {
            RealityPortalTheme {
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {
                    RealityPortalApp(ssoService = ssoService)
                }
            }
        }
    }

    override fun onNewIntent(intent: Intent) {
        super.onNewIntent(intent)
        // Handle deep-link when app is already running
        handleDeepLink(intent)
    }

    private fun handleDeepLink(intent: Intent?) {
        val uri = intent?.data ?: return

        // Handle SSO deep-link: reality://sso?token=xxx
        if (uri.scheme == "reality" && uri.host == "sso") {
            val token = uri.getQueryParameter("token")
            if (token != null) {
                // Validate token and login using lifecycleScope to prevent leaks
                lifecycleScope.launch { ssoService.validateAndLogin(token) }
            }
        }
    }
}

@Composable
fun RealityPortalApp(ssoService: SsoService) {
    val navController = rememberNavController()

    // Create listing repository - in production this would be injected via DI
    val listingRepository = remember { ListingRepository(baseUrl = ApiConfig.requireBaseUrl()) }

    RealityNavHost(
        navController = navController,
        ssoService = ssoService,
        listingRepository = listingRepository
    )
}

@Composable
fun RealityPortalTheme(content: @Composable () -> Unit) {
    MaterialTheme(colorScheme = lightColorScheme(), content = content)
}
