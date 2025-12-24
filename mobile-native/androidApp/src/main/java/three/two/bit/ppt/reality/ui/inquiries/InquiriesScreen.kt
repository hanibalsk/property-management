package three.two.bit.ppt.reality.ui.inquiries

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import coil.compose.AsyncImage
import coil.request.ImageRequest
import kotlinx.coroutines.launch
import three.two.bit.ppt.reality.auth.AuthState
import three.two.bit.ppt.reality.auth.SsoService
import three.two.bit.ppt.reality.inquiry.*
import three.two.bit.ppt.reality.listing.ListingRepository

/**
 * Inquiries screen for Reality Portal mobile app.
 *
 * Epic 48 - Story 48.6: Portal Mobile Inquiries
 */
@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun InquiriesScreen(
    repository: ListingRepository,
    ssoService: SsoService,
    onListingClick: (String) -> Unit,
    onBackClick: () -> Unit
) {
    val scope = rememberCoroutineScope()
    val authState by ssoService.authState.collectAsState()

    var selectedTab by remember { mutableIntStateOf(0) }
    var inquiries by remember { mutableStateOf<List<Inquiry>>(emptyList()) }
    var viewings by remember { mutableStateOf<List<ViewingRequest>>(emptyList()) }
    var isLoading by remember { mutableStateOf(true) }
    var errorMessage by remember { mutableStateOf<String?>(null) }

    // Create inquiry repository with session token
    val inquiryRepository =
        remember(authState) {
            val token = (authState as? AuthState.Authenticated)?.sessionToken
            InquiryRepository(sessionToken = token)
        }

    // Load data
    LaunchedEffect(authState) {
        if (authState is AuthState.Authenticated) {
            isLoading = true
            errorMessage = null

            // Load inquiries
            inquiryRepository
                .getInquiries()
                .fold(
                    onSuccess = { response -> inquiries = response.inquiries },
                    onFailure = { error -> errorMessage = error.message }
                )

            // Load viewings
            inquiryRepository
                .getViewings()
                .fold(
                    onSuccess = { response -> viewings = response.viewings },
                    onFailure = { /* Already showing error if inquiries failed */}
                )

            isLoading = false
        } else {
            isLoading = false
        }
    }

    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text("Inquiries") },
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
                                    Text("Messages")
                                    val pendingCount =
                                        inquiries.count { it.status == InquiryStatus.RESPONDED }
                                    if (pendingCount > 0) {
                                        Spacer(modifier = Modifier.width(4.dp))
                                        Badge { Text("$pendingCount") }
                                    }
                                }
                            },
                            icon = { Icon(Icons.Default.Email, contentDescription = null) }
                        )
                        Tab(
                            selected = selectedTab == 1,
                            onClick = { selectedTab = 1 },
                            text = {
                                Row(verticalAlignment = Alignment.CenterVertically) {
                                    Text("Viewings")
                                    val upcomingCount =
                                        viewings.count { it.status == ViewingStatus.CONFIRMED }
                                    if (upcomingCount > 0) {
                                        Spacer(modifier = Modifier.width(4.dp))
                                        Badge { Text("$upcomingCount") }
                                    }
                                }
                            },
                            icon = { Icon(Icons.Default.CalendarMonth, contentDescription = null) }
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
                                        inquiryRepository
                                            .getInquiries()
                                            .fold(
                                                onSuccess = { response ->
                                                    inquiries = response.inquiries
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
                            InquiriesList(
                                inquiries = inquiries,
                                onInquiryClick = { inquiry -> onListingClick(inquiry.listingId) }
                            )
                        }
                        selectedTab == 1 -> {
                            ViewingsList(
                                viewings = viewings,
                                onViewingClick = { viewing -> onListingClick(viewing.listingId) },
                                onCancelViewing = { viewingId ->
                                    scope.launch {
                                        inquiryRepository
                                            .cancelViewing(viewingId)
                                            .fold(
                                                onSuccess = {
                                                    viewings =
                                                        viewings.filter { it.id != viewingId }
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
            Icons.Default.Email,
            contentDescription = null,
            modifier = Modifier.size(64.dp),
            tint = MaterialTheme.colorScheme.onSurfaceVariant
        )
        Spacer(modifier = Modifier.height(16.dp))
        Text(
            text = "Sign in to view inquiries",
            style = MaterialTheme.typography.titleMedium,
            color = MaterialTheme.colorScheme.onSurfaceVariant
        )
        Spacer(modifier = Modifier.height(8.dp))
        Text(
            text = "Track your property inquiries and viewing requests",
            style = MaterialTheme.typography.bodyMedium,
            color = MaterialTheme.colorScheme.onSurfaceVariant
        )
    }
}

@Composable
private fun InquiriesList(inquiries: List<Inquiry>, onInquiryClick: (Inquiry) -> Unit) {
    if (inquiries.isEmpty()) {
        EmptyInquiries()
    } else {
        LazyColumn(
            contentPadding = PaddingValues(16.dp),
            verticalArrangement = Arrangement.spacedBy(12.dp)
        ) {
            items(inquiries, key = { it.id }) { inquiry ->
                InquiryCard(inquiry = inquiry, onClick = { onInquiryClick(inquiry) })
            }
        }
    }
}

@Composable
private fun InquiryCard(inquiry: Inquiry, onClick: () -> Unit) {
    Card(modifier = Modifier.fillMaxWidth(), onClick = onClick) {
        Row(modifier = Modifier.fillMaxWidth().padding(12.dp), verticalAlignment = Alignment.Top) {
            // Listing image
            AsyncImage(
                model =
                    ImageRequest.Builder(LocalContext.current)
                        .data(inquiry.listing?.primaryImage?.url ?: "")
                        .crossfade(true)
                        .build(),
                contentDescription = null,
                contentScale = ContentScale.Crop,
                modifier = Modifier.size(80.dp).clip(RoundedCornerShape(8.dp))
            )

            Spacer(modifier = Modifier.width(12.dp))

            Column(modifier = Modifier.weight(1f)) {
                // Status badge
                Row(verticalAlignment = Alignment.CenterVertically) {
                    StatusBadge(status = inquiry.status)
                    Spacer(modifier = Modifier.width(8.dp))
                    Text(
                        text = formatDate(inquiry.createdAt),
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant
                    )
                }

                Spacer(modifier = Modifier.height(4.dp))

                // Property title
                Text(
                    text = inquiry.listing?.title ?: "Property",
                    style = MaterialTheme.typography.titleSmall,
                    fontWeight = FontWeight.Bold,
                    maxLines = 1,
                    overflow = TextOverflow.Ellipsis
                )

                Spacer(modifier = Modifier.height(4.dp))

                // Latest message preview
                val latestMessage = inquiry.responses.lastOrNull()?.message ?: inquiry.message
                Text(
                    text = latestMessage,
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                    maxLines = 2,
                    overflow = TextOverflow.Ellipsis
                )

                // Response count
                if (inquiry.responses.isNotEmpty()) {
                    Spacer(modifier = Modifier.height(4.dp))
                    Text(
                        text = "${inquiry.responses.size} responses",
                        style = MaterialTheme.typography.labelSmall,
                        color = MaterialTheme.colorScheme.primary
                    )
                }
            }
        }
    }
}

@Composable
private fun StatusBadge(status: InquiryStatus) {
    val (color, text) =
        when (status) {
            InquiryStatus.PENDING -> Pair(MaterialTheme.colorScheme.tertiary, "Pending")
            InquiryStatus.RESPONDED -> Pair(MaterialTheme.colorScheme.primary, "Responded")
            InquiryStatus.CLOSED -> Pair(MaterialTheme.colorScheme.outline, "Closed")
        }

    Surface(shape = RoundedCornerShape(4.dp), color = color.copy(alpha = 0.12f)) {
        Text(
            text = text,
            modifier = Modifier.padding(horizontal = 6.dp, vertical = 2.dp),
            style = MaterialTheme.typography.labelSmall,
            color = color
        )
    }
}

@Composable
private fun EmptyInquiries() {
    Column(
        modifier = Modifier.fillMaxSize().padding(32.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.Center
    ) {
        Icon(
            Icons.Default.EmailRead,
            contentDescription = null,
            modifier = Modifier.size(64.dp),
            tint = MaterialTheme.colorScheme.onSurfaceVariant
        )
        Spacer(modifier = Modifier.height(16.dp))
        Text(
            text = "No inquiries yet",
            style = MaterialTheme.typography.titleMedium,
            color = MaterialTheme.colorScheme.onSurfaceVariant
        )
        Spacer(modifier = Modifier.height(8.dp))
        Text(
            text = "Contact property owners to start a conversation",
            style = MaterialTheme.typography.bodyMedium,
            color = MaterialTheme.colorScheme.onSurfaceVariant
        )
    }
}

@Composable
private fun ViewingsList(
    viewings: List<ViewingRequest>,
    onViewingClick: (ViewingRequest) -> Unit,
    onCancelViewing: (String) -> Unit
) {
    if (viewings.isEmpty()) {
        EmptyViewings()
    } else {
        LazyColumn(
            contentPadding = PaddingValues(16.dp),
            verticalArrangement = Arrangement.spacedBy(12.dp)
        ) {
            items(viewings, key = { it.id }) { viewing ->
                ViewingCard(
                    viewing = viewing,
                    onClick = { onViewingClick(viewing) },
                    onCancel = { onCancelViewing(viewing.id) }
                )
            }
        }
    }
}

@Composable
private fun ViewingCard(viewing: ViewingRequest, onClick: () -> Unit, onCancel: () -> Unit) {
    var showCancelDialog by remember { mutableStateOf(false) }

    Card(modifier = Modifier.fillMaxWidth(), onClick = onClick) {
        Column(modifier = Modifier.fillMaxWidth().padding(16.dp)) {
            Row(verticalAlignment = Alignment.CenterVertically) {
                ViewingStatusBadge(status = viewing.status)
                Spacer(modifier = Modifier.weight(1f))

                if (
                    viewing.status == ViewingStatus.PENDING ||
                        viewing.status == ViewingStatus.CONFIRMED
                ) {
                    IconButton(onClick = { showCancelDialog = true }) {
                        Icon(
                            Icons.Default.Cancel,
                            contentDescription = "Cancel",
                            tint = MaterialTheme.colorScheme.error
                        )
                    }
                }
            }

            Spacer(modifier = Modifier.height(8.dp))

            // Property
            Text(
                text = viewing.listing?.title ?: "Property Viewing",
                style = MaterialTheme.typography.titleMedium,
                fontWeight = FontWeight.Bold
            )

            Spacer(modifier = Modifier.height(8.dp))

            // Date and time
            Row(verticalAlignment = Alignment.CenterVertically) {
                Icon(
                    Icons.Default.CalendarToday,
                    contentDescription = null,
                    modifier = Modifier.size(16.dp),
                    tint = MaterialTheme.colorScheme.primary
                )
                Spacer(modifier = Modifier.width(4.dp))

                val displayDate = viewing.confirmedDate ?: viewing.preferredDate
                val displayTime = viewing.confirmedTime ?: viewing.preferredTime

                Text(
                    text = "$displayDate at $displayTime",
                    style = MaterialTheme.typography.bodyMedium
                )

                if (viewing.confirmedDate != null) {
                    Spacer(modifier = Modifier.width(8.dp))
                    Text(
                        text = "(Confirmed)",
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.primary
                    )
                }
            }

            // Message
            viewing.message?.let { message ->
                Spacer(modifier = Modifier.height(8.dp))
                Text(
                    text = message,
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                    maxLines = 2,
                    overflow = TextOverflow.Ellipsis
                )
            }
        }
    }

    if (showCancelDialog) {
        AlertDialog(
            onDismissRequest = { showCancelDialog = false },
            title = { Text("Cancel Viewing") },
            text = { Text("Are you sure you want to cancel this viewing request?") },
            confirmButton = {
                TextButton(
                    onClick = {
                        onCancel()
                        showCancelDialog = false
                    },
                    colors =
                        ButtonDefaults.textButtonColors(
                            contentColor = MaterialTheme.colorScheme.error
                        )
                ) {
                    Text("Cancel Viewing")
                }
            },
            dismissButton = { TextButton(onClick = { showCancelDialog = false }) { Text("Keep") } }
        )
    }
}

@Composable
private fun ViewingStatusBadge(status: ViewingStatus) {
    val (color, text, icon) =
        when (status) {
            ViewingStatus.PENDING ->
                Triple(MaterialTheme.colorScheme.tertiary, "Pending", Icons.Default.Schedule)
            ViewingStatus.CONFIRMED ->
                Triple(MaterialTheme.colorScheme.primary, "Confirmed", Icons.Default.CheckCircle)
            ViewingStatus.COMPLETED ->
                Triple(MaterialTheme.colorScheme.outline, "Completed", Icons.Default.Done)
            ViewingStatus.CANCELLED ->
                Triple(MaterialTheme.colorScheme.error, "Cancelled", Icons.Default.Cancel)
        }

    Surface(shape = RoundedCornerShape(4.dp), color = color.copy(alpha = 0.12f)) {
        Row(
            modifier = Modifier.padding(horizontal = 8.dp, vertical = 4.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            Icon(icon, contentDescription = null, modifier = Modifier.size(14.dp), tint = color)
            Spacer(modifier = Modifier.width(4.dp))
            Text(text = text, style = MaterialTheme.typography.labelSmall, color = color)
        }
    }
}

@Composable
private fun EmptyViewings() {
    Column(
        modifier = Modifier.fillMaxSize().padding(32.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.Center
    ) {
        Icon(
            Icons.Default.CalendarMonth,
            contentDescription = null,
            modifier = Modifier.size(64.dp),
            tint = MaterialTheme.colorScheme.onSurfaceVariant
        )
        Spacer(modifier = Modifier.height(16.dp))
        Text(
            text = "No viewings scheduled",
            style = MaterialTheme.typography.titleMedium,
            color = MaterialTheme.colorScheme.onSurfaceVariant
        )
        Spacer(modifier = Modifier.height(8.dp))
        Text(
            text = "Schedule viewings to see properties in person",
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

private fun formatDate(dateString: String): String {
    // Simple date formatting - in production use kotlinx-datetime
    return dateString.take(10)
}
