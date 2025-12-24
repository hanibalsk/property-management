package three.two.bit.ppt.reality.ui.favorites

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.launch
import three.two.bit.ppt.reality.auth.AuthState
import three.two.bit.ppt.reality.auth.SsoService
import three.two.bit.ppt.reality.favorites.*
import three.two.bit.ppt.reality.listing.ListingRepository
import three.two.bit.ppt.reality.ui.search.ListingCard

/**
 * Favorites screen for Reality Portal mobile app.
 *
 * Epic 48 - Story 48.3: Portal Mobile Favorites
 */
@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun FavoritesScreen(
    repository: ListingRepository,
    ssoService: SsoService,
    onListingClick: (String) -> Unit,
    onBackClick: () -> Unit
) {
    val scope = rememberCoroutineScope()
    val authState by ssoService.authState.collectAsState()

    var selectedTab by remember { mutableIntStateOf(0) }
    var favorites by remember { mutableStateOf<List<FavoriteEntry>>(emptyList()) }
    var savedSearches by remember { mutableStateOf<List<SavedSearch>>(emptyList()) }
    var isLoading by remember { mutableStateOf(true) }
    var errorMessage by remember { mutableStateOf<String?>(null) }

    // Create favorites repository with session token
    val favoritesRepository =
        remember(authState) {
            val token = (authState as? AuthState.Authenticated)?.sessionToken
            FavoritesRepository(sessionToken = token)
        }

    // Load data
    LaunchedEffect(authState) {
        if (authState is AuthState.Authenticated) {
            isLoading = true
            errorMessage = null

            // Load favorites
            favoritesRepository
                .getFavorites()
                .fold(
                    onSuccess = { response -> favorites = response.favorites },
                    onFailure = { error -> errorMessage = error.message }
                )

            // Load saved searches
            favoritesRepository
                .getSavedSearches()
                .fold(
                    onSuccess = { response -> savedSearches = response.searches },
                    onFailure = { /* Already showing error if favorites failed */}
                )

            isLoading = false
        } else {
            isLoading = false
        }
    }

    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text("Favorites") },
                navigationIcon = {
                    IconButton(onClick = onBackClick) {
                        Icon(Icons.Default.ArrowBack, contentDescription = "Back")
                    }
                }
            )
        }
    ) { paddingValues ->
        when (authState) {
            is AuthState.Unauthenticated,
            is AuthState.Error -> {
                NotSignedInContent(modifier = Modifier.padding(paddingValues))
            }
            is AuthState.Loading -> {
                Box(
                    modifier = Modifier.fillMaxSize().padding(paddingValues),
                    contentAlignment = Alignment.Center
                ) {
                    CircularProgressIndicator()
                }
            }
            is AuthState.Authenticated -> {
                Column(modifier = Modifier.fillMaxSize().padding(paddingValues)) {
                    // Tab row
                    TabRow(selectedTabIndex = selectedTab) {
                        Tab(
                            selected = selectedTab == 0,
                            onClick = { selectedTab = 0 },
                            text = {
                                Row(verticalAlignment = Alignment.CenterVertically) {
                                    Text("Properties")
                                    if (favorites.isNotEmpty()) {
                                        Spacer(modifier = Modifier.width(4.dp))
                                        Badge { Text("${favorites.size}") }
                                    }
                                }
                            },
                            icon = { Icon(Icons.Default.Favorite, contentDescription = null) }
                        )
                        Tab(
                            selected = selectedTab == 1,
                            onClick = { selectedTab = 1 },
                            text = {
                                Row(verticalAlignment = Alignment.CenterVertically) {
                                    Text("Searches")
                                    if (savedSearches.isNotEmpty()) {
                                        Spacer(modifier = Modifier.width(4.dp))
                                        Badge { Text("${savedSearches.size}") }
                                    }
                                }
                            },
                            icon = { Icon(Icons.Default.Search, contentDescription = null) }
                        )
                    }

                    // Content
                    when {
                        isLoading -> {
                            Box(
                                modifier = Modifier.fillMaxSize(),
                                contentAlignment = Alignment.Center
                            ) {
                                CircularProgressIndicator()
                            }
                        }
                        errorMessage != null -> {
                            ErrorContent(
                                message = errorMessage!!,
                                onRetry = {
                                    scope.launch {
                                        isLoading = true
                                        errorMessage = null
                                        favoritesRepository
                                            .getFavorites()
                                            .fold(
                                                onSuccess = { response ->
                                                    favorites = response.favorites
                                                },
                                                onFailure = { error ->
                                                    errorMessage = error.message
                                                }
                                            )
                                        isLoading = false
                                    }
                                }
                            )
                        }
                        selectedTab == 0 -> {
                            FavoritesList(
                                favorites = favorites,
                                onListingClick = onListingClick,
                                onRemoveFavorite = { listingId ->
                                    scope.launch {
                                        favoritesRepository
                                            .removeFavorite(listingId)
                                            .fold(
                                                onSuccess = {
                                                    favorites =
                                                        favorites.filter {
                                                            it.listingId != listingId
                                                        }
                                                },
                                                onFailure = { /* Show error */}
                                            )
                                    }
                                }
                            )
                        }
                        selectedTab == 1 -> {
                            SavedSearchesList(
                                searches = savedSearches,
                                onSearchClick = { /* Navigate to search with filters */},
                                onToggleAlert = { searchId, enabled ->
                                    scope.launch {
                                        favoritesRepository
                                            .toggleSearchAlert(searchId, enabled)
                                            .fold(
                                                onSuccess = { updated ->
                                                    savedSearches =
                                                        savedSearches.map {
                                                            if (it.id == searchId) updated else it
                                                        }
                                                },
                                                onFailure = { /* Show error */}
                                            )
                                    }
                                },
                                onDeleteSearch = { searchId ->
                                    scope.launch {
                                        favoritesRepository
                                            .deleteSavedSearch(searchId)
                                            .fold(
                                                onSuccess = {
                                                    savedSearches =
                                                        savedSearches.filter { it.id != searchId }
                                                },
                                                onFailure = { /* Show error */}
                                            )
                                    }
                                }
                            )
                        }
                    }
                }
            }
        }
    }
}

@Composable
private fun NotSignedInContent(modifier: Modifier = Modifier) {
    Column(
        modifier = modifier.fillMaxSize().padding(32.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.Center
    ) {
        Icon(
            Icons.Default.FavoriteBorder,
            contentDescription = null,
            modifier = Modifier.size(64.dp),
            tint = MaterialTheme.colorScheme.onSurfaceVariant
        )
        Spacer(modifier = Modifier.height(16.dp))
        Text(
            text = "Sign in to save favorites",
            style = MaterialTheme.typography.titleMedium,
            color = MaterialTheme.colorScheme.onSurfaceVariant
        )
        Spacer(modifier = Modifier.height(8.dp))
        Text(
            text = "Keep track of properties you're interested in",
            style = MaterialTheme.typography.bodyMedium,
            color = MaterialTheme.colorScheme.onSurfaceVariant
        )
    }
}

@Composable
private fun FavoritesList(
    favorites: List<FavoriteEntry>,
    onListingClick: (String) -> Unit,
    onRemoveFavorite: (String) -> Unit
) {
    if (favorites.isEmpty()) {
        EmptyFavorites()
    } else {
        LazyColumn(
            contentPadding = PaddingValues(16.dp),
            verticalArrangement = Arrangement.spacedBy(12.dp)
        ) {
            items(favorites, key = { it.id }) { favorite ->
                favorite.listing?.let { listing ->
                    ListingCard(
                        listing = listing,
                        onClick = { onListingClick(favorite.listingId) },
                        showFavoriteButton = true,
                        isFavorite = true,
                        onFavoriteClick = { onRemoveFavorite(favorite.listingId) }
                    )
                }
            }
        }
    }
}

@Composable
private fun EmptyFavorites() {
    Column(
        modifier = Modifier.fillMaxSize().padding(32.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.Center
    ) {
        Icon(
            Icons.Default.FavoriteBorder,
            contentDescription = null,
            modifier = Modifier.size(64.dp),
            tint = MaterialTheme.colorScheme.onSurfaceVariant
        )
        Spacer(modifier = Modifier.height(16.dp))
        Text(
            text = "No favorites yet",
            style = MaterialTheme.typography.titleMedium,
            color = MaterialTheme.colorScheme.onSurfaceVariant
        )
        Spacer(modifier = Modifier.height(8.dp))
        Text(
            text = "Save properties to compare later",
            style = MaterialTheme.typography.bodyMedium,
            color = MaterialTheme.colorScheme.onSurfaceVariant
        )
    }
}

@Composable
private fun SavedSearchesList(
    searches: List<SavedSearch>,
    onSearchClick: (SavedSearch) -> Unit,
    onToggleAlert: (String, Boolean) -> Unit,
    onDeleteSearch: (String) -> Unit
) {
    if (searches.isEmpty()) {
        EmptySavedSearches()
    } else {
        LazyColumn(
            contentPadding = PaddingValues(16.dp),
            verticalArrangement = Arrangement.spacedBy(12.dp)
        ) {
            items(searches, key = { it.id }) { search ->
                SavedSearchCard(
                    search = search,
                    onClick = { onSearchClick(search) },
                    onToggleAlert = { onToggleAlert(search.id, !search.alertEnabled) },
                    onDelete = { onDeleteSearch(search.id) }
                )
            }
        }
    }
}

@Composable
private fun SavedSearchCard(
    search: SavedSearch,
    onClick: () -> Unit,
    onToggleAlert: () -> Unit,
    onDelete: () -> Unit
) {
    var showDeleteDialog by remember { mutableStateOf(false) }

    Card(modifier = Modifier.fillMaxWidth(), onClick = onClick) {
        Row(
            modifier = Modifier.fillMaxWidth().padding(16.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            Column(modifier = Modifier.weight(1f)) {
                Row(verticalAlignment = Alignment.CenterVertically) {
                    Text(
                        text = search.name,
                        style = MaterialTheme.typography.titleMedium,
                        fontWeight = FontWeight.Bold
                    )
                    if (search.newCount > 0) {
                        Spacer(modifier = Modifier.width(8.dp))
                        Badge(containerColor = MaterialTheme.colorScheme.primary) {
                            Text("${search.newCount} new")
                        }
                    }
                }

                Spacer(modifier = Modifier.height(4.dp))

                // Build filter description
                val filterParts = mutableListOf<String>()
                search.filters?.let { filters ->
                    filters.type?.let { filterParts.add(it) }
                    filters.category?.let { filterParts.add(it) }
                    filters.city?.let { filterParts.add(it) }
                    if (filters.minPrice != null || filters.maxPrice != null) {
                        val priceRange = buildString {
                            append("€")
                            append(filters.minPrice?.toString() ?: "0")
                            append(" - €")
                            append(filters.maxPrice?.toString() ?: "∞")
                        }
                        filterParts.add(priceRange)
                    }
                }

                if (filterParts.isNotEmpty()) {
                    Text(
                        text = filterParts.joinToString(" • "),
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant
                    )
                }
            }

            // Alert toggle
            IconButton(onClick = onToggleAlert) {
                Icon(
                    if (search.alertEnabled) Icons.Default.Notifications
                    else Icons.Default.NotificationsOff,
                    contentDescription =
                        if (search.alertEnabled) "Disable alerts" else "Enable alerts",
                    tint =
                        if (search.alertEnabled) MaterialTheme.colorScheme.primary
                        else MaterialTheme.colorScheme.onSurfaceVariant
                )
            }

            // Delete button
            IconButton(onClick = { showDeleteDialog = true }) {
                Icon(
                    Icons.Default.Delete,
                    contentDescription = "Delete",
                    tint = MaterialTheme.colorScheme.error
                )
            }
        }
    }

    if (showDeleteDialog) {
        AlertDialog(
            onDismissRequest = { showDeleteDialog = false },
            title = { Text("Delete Saved Search") },
            text = { Text("Are you sure you want to delete \"${search.name}\"?") },
            confirmButton = {
                TextButton(
                    onClick = {
                        onDelete()
                        showDeleteDialog = false
                    },
                    colors =
                        ButtonDefaults.textButtonColors(
                            contentColor = MaterialTheme.colorScheme.error
                        )
                ) {
                    Text("Delete")
                }
            },
            dismissButton = {
                TextButton(onClick = { showDeleteDialog = false }) { Text("Cancel") }
            }
        )
    }
}

@Composable
private fun EmptySavedSearches() {
    Column(
        modifier = Modifier.fillMaxSize().padding(32.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.Center
    ) {
        Icon(
            Icons.Default.SearchOff,
            contentDescription = null,
            modifier = Modifier.size(64.dp),
            tint = MaterialTheme.colorScheme.onSurfaceVariant
        )
        Spacer(modifier = Modifier.height(16.dp))
        Text(
            text = "No saved searches",
            style = MaterialTheme.typography.titleMedium,
            color = MaterialTheme.colorScheme.onSurfaceVariant
        )
        Spacer(modifier = Modifier.height(8.dp))
        Text(
            text = "Save your search to get notified about new listings",
            style = MaterialTheme.typography.bodyMedium,
            color = MaterialTheme.colorScheme.onSurfaceVariant
        )
    }
}

@Composable
private fun ErrorContent(message: String, onRetry: () -> Unit) {
    Column(
        modifier = Modifier.fillMaxSize().padding(32.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.Center
    ) {
        Icon(
            Icons.Default.Error,
            contentDescription = null,
            modifier = Modifier.size(64.dp),
            tint = MaterialTheme.colorScheme.error
        )
        Spacer(modifier = Modifier.height(16.dp))
        Text(
            text = message,
            style = MaterialTheme.typography.bodyLarge,
            color = MaterialTheme.colorScheme.onSurfaceVariant
        )
        Spacer(modifier = Modifier.height(16.dp))
        Button(onClick = onRetry) { Text("Retry") }
    }
}
