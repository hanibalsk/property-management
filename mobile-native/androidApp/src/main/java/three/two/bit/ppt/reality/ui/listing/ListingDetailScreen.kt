package three.two.bit.ppt.reality.ui.listing

import android.content.Intent
import android.net.Uri
import androidx.compose.foundation.ExperimentalFoundationApi
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.pager.HorizontalPager
import androidx.compose.foundation.pager.rememberPagerState
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import coil.compose.AsyncImage
import coil.request.ImageRequest
import kotlinx.coroutines.launch
import three.two.bit.ppt.reality.auth.AuthState
import three.two.bit.ppt.reality.auth.SsoService
import three.two.bit.ppt.reality.listing.*
import three.two.bit.ppt.reality.util.FormatUtils

/**
 * Listing detail screen for Reality Portal mobile app.
 *
 * Epic 48 - Story 48.2: Portal Mobile Listing View
 */
@OptIn(ExperimentalMaterial3Api::class, ExperimentalFoundationApi::class)
@Composable
fun ListingDetailScreen(
    listingId: String,
    repository: ListingRepository,
    ssoService: SsoService,
    onBackClick: () -> Unit,
    onInquirySuccess: () -> Unit
) {
    val scope = rememberCoroutineScope()
    val context = LocalContext.current
    val authState by ssoService.authState.collectAsState()

    var listing by remember { mutableStateOf<ListingDetail?>(null) }
    var isLoading by remember { mutableStateOf(true) }
    var errorMessage by remember { mutableStateOf<String?>(null) }
    var isFavorite by remember { mutableStateOf(false) }
    var showInquiryDialog by remember { mutableStateOf(false) }
    var showShareSheet by remember { mutableStateOf(false) }

    // Load listing details
    LaunchedEffect(listingId) {
        isLoading = true
        repository
            .getListingDetail(listingId)
            .fold(
                onSuccess = { detail ->
                    listing = detail
                    isLoading = false
                },
                onFailure = { error ->
                    errorMessage = error.message
                    isLoading = false
                }
            )
    }

    Scaffold(
        topBar = {
            TopAppBar(
                title = {},
                navigationIcon = {
                    IconButton(onClick = onBackClick) {
                        Icon(Icons.Default.ArrowBack, contentDescription = "Back")
                    }
                },
                actions = {
                    IconButton(onClick = { showShareSheet = true }) {
                        Icon(Icons.Default.Share, contentDescription = "Share")
                    }
                    IconButton(
                        onClick = {
                            if (authState is AuthState.Authenticated) {
                                isFavorite = !isFavorite
                                // TODO(Epic-48): Persist favorites via FavoritesRepository
                                // Current limitation: Favorites only toggle local UI state and are
                                // not persisted to backend. Changes will be lost on screen
                                // navigation or app restart.
                                // Required implementation:
                                // - Create FavoritesRepository instance with session token
                                // - Call addFavorite(listingId) when isFavorite becomes true
                                // - Call removeFavorite(listingId) when isFavorite becomes false
                                // - Handle Result.failure with error message display
                                // - Load initial favorite state via isFavorite(listingId) on mount
                            }
                        }
                    ) {
                        Icon(
                            if (isFavorite) Icons.Default.Favorite
                            else Icons.Default.FavoriteBorder,
                            contentDescription =
                                if (isFavorite) "Remove from favorites" else "Add to favorites",
                            tint =
                                if (isFavorite) MaterialTheme.colorScheme.error
                                else MaterialTheme.colorScheme.onSurface
                        )
                    }
                },
                colors = TopAppBarDefaults.topAppBarColors(containerColor = Color.Transparent)
            )
        },
        bottomBar = {
            listing?.let { detail ->
                BottomContactBar(
                    realtor = detail.realtor,
                    onCallClick = {
                        detail.realtor?.phone?.let { phone ->
                            val intent =
                                Intent(Intent.ACTION_DIAL).apply { data = Uri.parse("tel:$phone") }
                            context.startActivity(intent)
                        }
                    },
                    onEmailClick = {
                        detail.realtor?.email?.let { email ->
                            val intent =
                                Intent(Intent.ACTION_SENDTO).apply {
                                    data = Uri.parse("mailto:$email")
                                    putExtra(Intent.EXTRA_SUBJECT, "Inquiry about: ${detail.title}")
                                }
                            context.startActivity(intent)
                        }
                    },
                    onInquiryClick = { showInquiryDialog = true }
                )
            }
        }
    ) { paddingValues ->
        when {
            isLoading -> {
                Box(
                    modifier = Modifier.fillMaxSize().padding(paddingValues),
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
                            repository
                                .getListingDetail(listingId)
                                .fold(
                                    onSuccess = { detail -> listing = detail },
                                    onFailure = { error -> errorMessage = error.message }
                                )
                            isLoading = false
                        }
                    },
                    modifier = Modifier.padding(paddingValues)
                )
            }
            listing != null -> {
                ListingContent(listing = listing!!, modifier = Modifier.padding(paddingValues))
            }
        }
    }

    // Inquiry dialog
    if (showInquiryDialog && listing != null) {
        InquiryDialog(
            listing = listing!!,
            isAuthenticated = authState is AuthState.Authenticated,
            onDismiss = { showInquiryDialog = false },
            onSubmit = { message ->
                // TODO(Epic-48): Submit inquiry via InquiryRepository
                // Current limitation: Dialog collects inquiry message but does not submit to
                // backend. User will see success feedback without actual inquiry being sent.
                // Required implementation:
                // - Create InquiryRepository instance with session token from authState
                // - Build CreateInquiryRequest with listingId and message
                // - Call createInquiry(request) in coroutine scope
                // - Handle Result.success: close dialog and call onInquirySuccess()
                // - Handle Result.failure: display error message to user
                // - For unauthenticated users, include optional name/email/phone fields
                showInquiryDialog = false
                onInquirySuccess()
            }
        )
    }

    // Share sheet
    if (showShareSheet && listing != null) {
        ShareListingSheet(listing = listing!!, onDismiss = { showShareSheet = false })
    }
}

@OptIn(ExperimentalFoundationApi::class)
@Composable
private fun ListingContent(listing: ListingDetail, modifier: Modifier = Modifier) {
    LazyColumn(modifier = modifier.fillMaxSize()) {
        // Image gallery
        item { ImageGallery(images = listing.images) }

        // Price and type badge
        item { PriceSection(listing = listing) }

        // Title and location
        item { TitleSection(listing = listing) }

        // Property details grid
        item { PropertyDetailsGrid(listing = listing) }

        // Description
        item { DescriptionSection(description = listing.description) }

        // Features
        if (listing.features.isNotEmpty()) {
            item { FeaturesSection(features = listing.features) }
        }

        // Additional details
        item { AdditionalDetailsSection(listing = listing) }

        // Realtor info
        listing.realtor?.let { realtor -> item { RealtorSection(realtor = realtor) } }

        // Bottom spacing
        item { Spacer(modifier = Modifier.height(80.dp)) }
    }
}

@OptIn(ExperimentalFoundationApi::class)
@Composable
private fun ImageGallery(images: List<ListingImage>) {
    if (images.isEmpty()) {
        Box(
            modifier =
                Modifier.fillMaxWidth()
                    .height(300.dp)
                    .background(MaterialTheme.colorScheme.surfaceVariant),
            contentAlignment = Alignment.Center
        ) {
            Icon(
                Icons.Default.Image,
                contentDescription = null,
                modifier = Modifier.size(64.dp),
                tint = MaterialTheme.colorScheme.onSurfaceVariant
            )
        }
        return
    }

    val pagerState = rememberPagerState(pageCount = { images.size })

    Box(modifier = Modifier.fillMaxWidth()) {
        HorizontalPager(state = pagerState, modifier = Modifier.fillMaxWidth().height(300.dp)) {
            page ->
            AsyncImage(
                model =
                    ImageRequest.Builder(LocalContext.current)
                        .data(images[page].url)
                        .crossfade(true)
                        .build(),
                contentDescription = images[page].caption,
                contentScale = ContentScale.Crop,
                modifier = Modifier.fillMaxSize()
            )
        }

        // Page indicator
        if (images.size > 1) {
            Row(
                modifier = Modifier.align(Alignment.BottomCenter).padding(16.dp),
                horizontalArrangement = Arrangement.spacedBy(4.dp)
            ) {
                repeat(images.size) { index ->
                    Box(
                        modifier =
                            Modifier.size(8.dp)
                                .clip(CircleShape)
                                .background(
                                    if (index == pagerState.currentPage) Color.White
                                    else Color.White.copy(alpha = 0.5f)
                                )
                    )
                }
            }

            // Image counter
            Surface(
                modifier = Modifier.align(Alignment.TopEnd).padding(16.dp),
                shape = RoundedCornerShape(4.dp),
                color = Color.Black.copy(alpha = 0.6f)
            ) {
                Text(
                    text = "${pagerState.currentPage + 1}/${images.size}",
                    modifier = Modifier.padding(horizontal = 8.dp, vertical = 4.dp),
                    style = MaterialTheme.typography.labelMedium,
                    color = Color.White
                )
            }
        }
    }
}

@Composable
private fun PriceSection(listing: ListingDetail) {
    Row(
        modifier = Modifier.fillMaxWidth().padding(16.dp),
        horizontalArrangement = Arrangement.SpaceBetween,
        verticalAlignment = Alignment.CenterVertically
    ) {
        Column {
            Text(
                text = formatPrice(listing.price, listing.currency),
                style = MaterialTheme.typography.headlineMedium,
                fontWeight = FontWeight.Bold,
                color = MaterialTheme.colorScheme.primary
            )
            listing.pricePerSqm?.let { ppsm ->
                Text(
                    text = "${formatPrice(ppsm, listing.currency)}/m²",
                    style = MaterialTheme.typography.bodyMedium,
                    color = MaterialTheme.colorScheme.onSurfaceVariant
                )
            }
        }

        Surface(
            shape = RoundedCornerShape(8.dp),
            color =
                when (listing.type) {
                    ListingType.SALE -> MaterialTheme.colorScheme.primary
                    ListingType.RENT -> MaterialTheme.colorScheme.secondary
                }
        ) {
            Text(
                text =
                    when (listing.type) {
                        ListingType.SALE -> "For Sale"
                        ListingType.RENT -> "For Rent"
                    },
                modifier = Modifier.padding(horizontal = 12.dp, vertical = 6.dp),
                style = MaterialTheme.typography.labelLarge,
                color = Color.White
            )
        }
    }
}

@Composable
private fun TitleSection(listing: ListingDetail) {
    Column(modifier = Modifier.fillMaxWidth().padding(horizontal = 16.dp)) {
        Text(
            text = listing.title,
            style = MaterialTheme.typography.titleLarge,
            fontWeight = FontWeight.Bold
        )

        Spacer(modifier = Modifier.height(8.dp))

        Row(verticalAlignment = Alignment.CenterVertically) {
            Icon(
                Icons.Default.LocationOn,
                contentDescription = null,
                modifier = Modifier.size(18.dp),
                tint = MaterialTheme.colorScheme.primary
            )
            Spacer(modifier = Modifier.width(4.dp))
            Text(
                text = buildLocationString(listing.address),
                style = MaterialTheme.typography.bodyLarge,
                color = MaterialTheme.colorScheme.onSurfaceVariant
            )
        }
    }
}

@Composable
private fun PropertyDetailsGrid(listing: ListingDetail) {
    Card(modifier = Modifier.fillMaxWidth().padding(16.dp)) {
        Row(
            modifier = Modifier.fillMaxWidth().padding(16.dp),
            horizontalArrangement = Arrangement.SpaceEvenly
        ) {
            PropertyDetailItem(
                icon = Icons.Default.SquareFoot,
                value = "${listing.areaSqm.toInt()}",
                label = "m²"
            )

            listing.rooms?.let { rooms ->
                VerticalDivider(modifier = Modifier.height(48.dp))
                PropertyDetailItem(
                    icon = Icons.Default.MeetingRoom,
                    value = "$rooms",
                    label = "Rooms"
                )
            }

            listing.bedrooms?.let { bedrooms ->
                VerticalDivider(modifier = Modifier.height(48.dp))
                PropertyDetailItem(icon = Icons.Default.Bed, value = "$bedrooms", label = "Beds")
            }

            listing.bathrooms?.let { bathrooms ->
                VerticalDivider(modifier = Modifier.height(48.dp))
                PropertyDetailItem(
                    icon = Icons.Default.Bathtub,
                    value = "$bathrooms",
                    label = "Baths"
                )
            }
        }
    }
}

@Composable
private fun PropertyDetailItem(
    icon: androidx.compose.ui.graphics.vector.ImageVector,
    value: String,
    label: String
) {
    Column(horizontalAlignment = Alignment.CenterHorizontally) {
        Icon(
            icon,
            contentDescription = null,
            modifier = Modifier.size(24.dp),
            tint = MaterialTheme.colorScheme.primary
        )
        Spacer(modifier = Modifier.height(4.dp))
        Text(
            text = value,
            style = MaterialTheme.typography.titleMedium,
            fontWeight = FontWeight.Bold
        )
        Text(
            text = label,
            style = MaterialTheme.typography.bodySmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant
        )
    }
}

@Composable
private fun DescriptionSection(description: String) {
    Column(modifier = Modifier.fillMaxWidth().padding(16.dp)) {
        Text(
            text = "Description",
            style = MaterialTheme.typography.titleMedium,
            fontWeight = FontWeight.Bold
        )

        Spacer(modifier = Modifier.height(8.dp))

        Text(
            text = description,
            style = MaterialTheme.typography.bodyMedium,
            color = MaterialTheme.colorScheme.onSurfaceVariant
        )
    }
}

@Composable
private fun FeaturesSection(features: List<String>) {
    Column(modifier = Modifier.fillMaxWidth().padding(16.dp)) {
        Text(
            text = "Features",
            style = MaterialTheme.typography.titleMedium,
            fontWeight = FontWeight.Bold
        )

        Spacer(modifier = Modifier.height(8.dp))

        LazyRow(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
            items(features) { feature ->
                AssistChip(
                    onClick = {},
                    label = { Text(feature) },
                    leadingIcon = {
                        Icon(
                            Icons.Default.Check,
                            contentDescription = null,
                            modifier = Modifier.size(16.dp)
                        )
                    }
                )
            }
        }
    }
}

@Composable
private fun AdditionalDetailsSection(listing: ListingDetail) {
    Card(modifier = Modifier.fillMaxWidth().padding(16.dp)) {
        Column(modifier = Modifier.padding(16.dp)) {
            Text(
                text = "Additional Details",
                style = MaterialTheme.typography.titleMedium,
                fontWeight = FontWeight.Bold
            )

            Spacer(modifier = Modifier.height(12.dp))

            listing.yearBuilt?.let { DetailRow(label = "Year Built", value = "$it") }
            listing.yearRenovated?.let { DetailRow(label = "Year Renovated", value = "$it") }
            listing.floor?.let {
                val floorText =
                    if (listing.totalFloors != null) "$it/${listing.totalFloors}" else "$it"
                DetailRow(label = "Floor", value = floorText)
            }
            listing.usableAreaSqm?.let {
                DetailRow(label = "Usable Area", value = "${it.toInt()} m²")
            }
            listing.landAreaSqm?.let { DetailRow(label = "Land Area", value = "${it.toInt()} m²") }
            listing.energyRating?.let { DetailRow(label = "Energy Rating", value = it) }
            listing.heatingType?.let { DetailRow(label = "Heating", value = it) }
            listing.parking?.let { DetailRow(label = "Parking", value = it) }
        }
    }
}

@Composable
private fun DetailRow(label: String, value: String) {
    Row(
        modifier = Modifier.fillMaxWidth().padding(vertical = 4.dp),
        horizontalArrangement = Arrangement.SpaceBetween
    ) {
        Text(
            text = label,
            style = MaterialTheme.typography.bodyMedium,
            color = MaterialTheme.colorScheme.onSurfaceVariant
        )
        Text(
            text = value,
            style = MaterialTheme.typography.bodyMedium,
            fontWeight = FontWeight.Medium
        )
    }
}

@Composable
private fun RealtorSection(realtor: RealtorInfo) {
    Card(modifier = Modifier.fillMaxWidth().padding(16.dp)) {
        Row(
            modifier = Modifier.fillMaxWidth().padding(16.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            AsyncImage(
                model =
                    ImageRequest.Builder(LocalContext.current)
                        .data(realtor.avatarUrl ?: "")
                        .crossfade(true)
                        .build(),
                contentDescription = realtor.name,
                contentScale = ContentScale.Crop,
                modifier =
                    Modifier.size(60.dp)
                        .clip(CircleShape)
                        .background(MaterialTheme.colorScheme.surfaceVariant)
            )

            Spacer(modifier = Modifier.width(16.dp))

            Column(modifier = Modifier.weight(1f)) {
                Text(
                    text = realtor.name,
                    style = MaterialTheme.typography.titleMedium,
                    fontWeight = FontWeight.Bold
                )
                realtor.agency?.let { agency ->
                    Text(
                        text = agency.name,
                        style = MaterialTheme.typography.bodyMedium,
                        color = MaterialTheme.colorScheme.onSurfaceVariant
                    )
                }
            }
        }
    }
}

@Composable
private fun BottomContactBar(
    realtor: RealtorInfo?,
    onCallClick: () -> Unit,
    onEmailClick: () -> Unit,
    onInquiryClick: () -> Unit
) {
    Surface(modifier = Modifier.fillMaxWidth(), tonalElevation = 8.dp, shadowElevation = 8.dp) {
        Row(
            modifier = Modifier.fillMaxWidth().padding(16.dp),
            horizontalArrangement = Arrangement.spacedBy(8.dp)
        ) {
            if (realtor?.phone != null) {
                OutlinedButton(onClick = onCallClick, modifier = Modifier.weight(1f)) {
                    Icon(Icons.Default.Phone, contentDescription = null)
                    Spacer(modifier = Modifier.width(4.dp))
                    Text("Call")
                }
            }

            if (realtor?.email != null) {
                OutlinedButton(onClick = onEmailClick, modifier = Modifier.weight(1f)) {
                    Icon(Icons.Default.Email, contentDescription = null)
                    Spacer(modifier = Modifier.width(4.dp))
                    Text("Email")
                }
            }

            Button(onClick = onInquiryClick, modifier = Modifier.weight(1f)) {
                Icon(Icons.Default.Message, contentDescription = null)
                Spacer(modifier = Modifier.width(4.dp))
                Text("Inquire")
            }
        }
    }
}

@Composable
private fun ErrorContent(message: String, onRetry: () -> Unit, modifier: Modifier = Modifier) {
    Column(
        modifier = modifier.fillMaxSize().padding(32.dp),
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

@Composable
private fun InquiryDialog(
    listing: ListingDetail,
    isAuthenticated: Boolean,
    onDismiss: () -> Unit,
    onSubmit: (String) -> Unit
) {
    var message by remember {
        mutableStateOf(
            "Hi, I'm interested in ${listing.title}. Please contact me with more information."
        )
    }

    AlertDialog(
        onDismissRequest = onDismiss,
        title = { Text("Send Inquiry") },
        text = {
            Column {
                if (!isAuthenticated) {
                    Card(
                        colors =
                            CardDefaults.cardColors(
                                containerColor = MaterialTheme.colorScheme.tertiaryContainer
                            )
                    ) {
                        Row(
                            modifier = Modifier.padding(12.dp),
                            verticalAlignment = Alignment.CenterVertically
                        ) {
                            Icon(
                                Icons.Default.Info,
                                contentDescription = null,
                                tint = MaterialTheme.colorScheme.onTertiaryContainer
                            )
                            Spacer(modifier = Modifier.width(8.dp))
                            Text(
                                text = "Sign in to track your inquiries",
                                style = MaterialTheme.typography.bodySmall,
                                color = MaterialTheme.colorScheme.onTertiaryContainer
                            )
                        }
                    }
                    Spacer(modifier = Modifier.height(12.dp))
                }

                OutlinedTextField(
                    value = message,
                    onValueChange = { message = it },
                    label = { Text("Message") },
                    modifier = Modifier.fillMaxWidth(),
                    minLines = 4,
                    maxLines = 6
                )
            }
        },
        confirmButton = {
            Button(onClick = { onSubmit(message) }, enabled = message.isNotBlank()) { Text("Send") }
        },
        dismissButton = { TextButton(onClick = onDismiss) { Text("Cancel") } }
    )
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
private fun ShareListingSheet(listing: ListingDetail, onDismiss: () -> Unit) {
    val context = LocalContext.current

    ModalBottomSheet(onDismissRequest = onDismiss) {
        Column(modifier = Modifier.fillMaxWidth().padding(16.dp)) {
            Text(
                text = "Share Property",
                style = MaterialTheme.typography.titleLarge,
                fontWeight = FontWeight.Bold
            )

            Spacer(modifier = Modifier.height(16.dp))

            ListItem(
                headlineContent = { Text("Share Link") },
                leadingContent = { Icon(Icons.Default.Link, contentDescription = null) },
                modifier = Modifier.fillMaxWidth()
            )

            ListItem(
                headlineContent = { Text("Share via...") },
                leadingContent = { Icon(Icons.Default.Share, contentDescription = null) },
                modifier = Modifier.fillMaxWidth()
            )

            Spacer(modifier = Modifier.height(32.dp))
        }
    }
}

private fun formatPrice(price: Long, currency: String): String {
    return FormatUtils.formatPrice(price, currency)
}

private fun buildLocationString(address: Address): String {
    // Use detailed location for listing detail view (includes street and postal code)
    return FormatUtils.buildDetailedLocationString(address)
}
