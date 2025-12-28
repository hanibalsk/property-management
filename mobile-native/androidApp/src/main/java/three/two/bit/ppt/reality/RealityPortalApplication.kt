package three.two.bit.ppt.reality

import android.app.Application
import three.two.bit.ppt.reality.api.HttpClientProvider

/**
 * Application class for Reality Portal.
 *
 * Manages application-level lifecycle, including shared resource cleanup.
 */
class RealityPortalApplication : Application() {

    override fun onTerminate() {
        super.onTerminate()
        // Clean up shared HttpClient resources when application terminates
        HttpClientProvider.close()
    }
}
