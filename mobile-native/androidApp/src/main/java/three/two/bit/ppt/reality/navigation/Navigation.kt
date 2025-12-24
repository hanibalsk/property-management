package three.two.bit.ppt.reality.navigation

import androidx.compose.runtime.Composable
import androidx.navigation.NavHostController
import androidx.navigation.NavType
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import androidx.navigation.navArgument
import three.two.bit.ppt.reality.auth.SsoService
import three.two.bit.ppt.reality.listing.ListingRepository
import three.two.bit.ppt.reality.ui.account.AccountScreen
import three.two.bit.ppt.reality.ui.favorites.FavoritesScreen
import three.two.bit.ppt.reality.ui.home.HomeScreen
import three.two.bit.ppt.reality.ui.inquiries.InquiriesScreen
import three.two.bit.ppt.reality.ui.listing.ListingDetailScreen
import three.two.bit.ppt.reality.ui.search.SearchScreen

/**
 * Navigation routes for Reality Portal.
 *
 * Epic 48 - All Stories
 */
sealed class Screen(val route: String) {
    data object Home : Screen("home")

    data object Search : Screen("search")

    data object ListingDetail : Screen("listing/{listingId}") {
        fun createRoute(listingId: String) = "listing/$listingId"
    }

    data object Favorites : Screen("favorites")

    data object Alerts : Screen("alerts")

    data object Account : Screen("account")

    data object Inquiries : Screen("inquiries")
}

@Composable
fun RealityNavHost(
    navController: NavHostController = rememberNavController(),
    ssoService: SsoService,
    listingRepository: ListingRepository,
    startDestination: String = Screen.Home.route
) {
    NavHost(navController = navController, startDestination = startDestination) {
        composable(Screen.Home.route) {
            HomeScreen(
                repository = listingRepository,
                ssoService = ssoService,
                onSearchClick = { navController.navigate(Screen.Search.route) },
                onListingClick = { id ->
                    navController.navigate(Screen.ListingDetail.createRoute(id))
                },
                onFavoritesClick = { navController.navigate(Screen.Favorites.route) },
                onAccountClick = { navController.navigate(Screen.Account.route) },
                onInquiriesClick = { navController.navigate(Screen.Inquiries.route) }
            )
        }

        composable(Screen.Search.route) {
            SearchScreen(
                repository = listingRepository,
                onListingClick = { id ->
                    navController.navigate(Screen.ListingDetail.createRoute(id))
                },
                onBackClick = { navController.popBackStack() }
            )
        }

        composable(
            route = Screen.ListingDetail.route,
            arguments = listOf(navArgument("listingId") { type = NavType.StringType })
        ) { backStackEntry ->
            val listingId = backStackEntry.arguments?.getString("listingId") ?: return@composable
            ListingDetailScreen(
                listingId = listingId,
                repository = listingRepository,
                ssoService = ssoService,
                onBackClick = { navController.popBackStack() },
                onInquirySuccess = { navController.navigate(Screen.Inquiries.route) }
            )
        }

        composable(Screen.Favorites.route) {
            FavoritesScreen(
                repository = listingRepository,
                ssoService = ssoService,
                onListingClick = { id ->
                    navController.navigate(Screen.ListingDetail.createRoute(id))
                },
                onBackClick = { navController.popBackStack() }
            )
        }

        composable(Screen.Account.route) {
            AccountScreen(
                ssoService = ssoService,
                onBackClick = { navController.popBackStack() },
                onLogout = {
                    ssoService.logout()
                    navController.popBackStack(Screen.Home.route, inclusive = false)
                }
            )
        }

        composable(Screen.Inquiries.route) {
            InquiriesScreen(
                repository = listingRepository,
                ssoService = ssoService,
                onListingClick = { id ->
                    navController.navigate(Screen.ListingDetail.createRoute(id))
                },
                onBackClick = { navController.popBackStack() }
            )
        }
    }
}
