package three.two.bit.ppt.reality.ui.account

import android.util.Log
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.Logout
import androidx.compose.material.icons.filled.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import coil.compose.AsyncImage
import coil.request.ImageRequest
import kotlinx.coroutines.launch
import three.two.bit.ppt.reality.BuildConfig
import three.two.bit.ppt.reality.R
import three.two.bit.ppt.reality.api.ApiConfig
import three.two.bit.ppt.reality.auth.AuthState
import three.two.bit.ppt.reality.auth.SsoService
import three.two.bit.ppt.reality.notifications.NotificationPreferences
import three.two.bit.ppt.reality.notifications.NotificationRepository

private const val TAG = "AccountScreen"

/**
 * Account screen for Reality Portal mobile app.
 *
 * Epic 48 - Story 48.5: Portal Mobile Account
 */
@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun AccountScreen(ssoService: SsoService, onBackClick: () -> Unit, onLogout: () -> Unit) {
    val scope = rememberCoroutineScope()
    val authState by ssoService.authState.collectAsState()

    var showLogoutDialog by remember { mutableStateOf(false) }
    var notificationPrefs by remember { mutableStateOf<NotificationPreferences?>(null) }

    // Create notification repository with session token
    val notificationRepository =
        remember(authState) {
            val token = (authState as? AuthState.Authenticated)?.sessionToken
            NotificationRepository(baseUrl = ApiConfig.requireBaseUrl(), sessionToken = token)
        }

    // Load notification preferences
    LaunchedEffect(authState) {
        if (authState is AuthState.Authenticated) {
            notificationRepository
                .getPreferences()
                .fold(
                    onSuccess = { prefs -> notificationPrefs = prefs },
                    onFailure = { error ->
                        Log.e(TAG, "Failed to load notification preferences", error)
                        // Use default preferences if loading fails
                    }
                )
        }
    }

    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text(stringResource(R.string.tab_account)) },
                navigationIcon = {
                    IconButton(onClick = onBackClick) {
                        Icon(
                            Icons.Default.ArrowBack,
                            contentDescription = stringResource(R.string.back)
                        )
                    }
                }
            )
        }
    ) { paddingValues ->
        when (val state = authState) {
            is AuthState.Unauthenticated,
            is AuthState.Error -> {
                NotSignedInContent(
                    onSignInClick = { /* Open SSO login via PM app */},
                    modifier = Modifier.padding(paddingValues)
                )
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
                LazyColumn(
                    modifier = Modifier.fillMaxSize().padding(paddingValues),
                    contentPadding = PaddingValues(16.dp),
                    verticalArrangement = Arrangement.spacedBy(16.dp)
                ) {
                    // Profile section
                    item { ProfileSection(user = state.user) }

                    // Notification preferences section
                    item {
                        notificationPrefs?.let { prefs ->
                            NotificationPreferencesSection(
                                preferences = prefs,
                                onPreferenceChange = { newPrefs ->
                                    scope.launch {
                                        notificationRepository
                                            .updatePreferences(newPrefs)
                                            .fold(
                                                onSuccess = { notificationPrefs = it },
                                                onFailure = { error ->
                                                    Log.e(
                                                        TAG,
                                                        "Failed to update notification preferences",
                                                        error
                                                    )
                                                    // Revert to previous preferences on failure
                                                }
                                            )
                                    }
                                }
                            )
                        }
                    }

                    // App settings section
                    item { AppSettingsSection() }

                    // About section
                    item { AboutSection() }

                    // Logout button
                    item {
                        OutlinedButton(
                            onClick = { showLogoutDialog = true },
                            modifier = Modifier.fillMaxWidth(),
                            colors =
                                ButtonDefaults.outlinedButtonColors(
                                    contentColor = MaterialTheme.colorScheme.error
                                )
                        ) {
                            Icon(
                                Icons.AutoMirrored.Filled.Logout,
                                contentDescription = null,
                                modifier = Modifier.size(18.dp)
                            )
                            Spacer(modifier = Modifier.width(8.dp))
                            Text(stringResource(R.string.sign_out))
                        }
                    }
                }
            }
        }
    }

    // Logout confirmation dialog
    if (showLogoutDialog) {
        AlertDialog(
            onDismissRequest = { showLogoutDialog = false },
            title = { Text(stringResource(R.string.sign_out)) },
            text = { Text(stringResource(R.string.confirm_sign_out_message)) },
            confirmButton = {
                TextButton(
                    onClick = {
                        showLogoutDialog = false
                        onLogout()
                    },
                    colors =
                        ButtonDefaults.textButtonColors(
                            contentColor = MaterialTheme.colorScheme.error
                        )
                ) {
                    Text(stringResource(R.string.sign_out))
                }
            },
            dismissButton = {
                TextButton(onClick = { showLogoutDialog = false }) {
                    Text(stringResource(R.string.cancel))
                }
            }
        )
    }
}

@Composable
private fun NotSignedInContent(onSignInClick: () -> Unit, modifier: Modifier = Modifier) {
    Column(
        modifier = modifier.fillMaxSize().padding(32.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.Center
    ) {
        Icon(
            Icons.Default.AccountCircle,
            contentDescription = null,
            modifier = Modifier.size(80.dp),
            tint = MaterialTheme.colorScheme.onSurfaceVariant
        )

        Spacer(modifier = Modifier.height(24.dp))

        Text(
            text = stringResource(R.string.sign_in_title),
            style = MaterialTheme.typography.titleLarge,
            fontWeight = FontWeight.Bold
        )

        Spacer(modifier = Modifier.height(8.dp))

        Text(
            text = stringResource(R.string.sign_in_benefits),
            style = MaterialTheme.typography.bodyMedium,
            color = MaterialTheme.colorScheme.onSurfaceVariant
        )

        Spacer(modifier = Modifier.height(32.dp))

        Button(onClick = onSignInClick, modifier = Modifier.fillMaxWidth()) {
            Icon(Icons.Default.Login, contentDescription = null, modifier = Modifier.size(18.dp))
            Spacer(modifier = Modifier.width(8.dp))
            Text(stringResource(R.string.sign_in_pm_app))
        }

        Spacer(modifier = Modifier.height(16.dp))

        Text(
            text = stringResource(R.string.sign_in_redirect_notice),
            style = MaterialTheme.typography.bodySmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant
        )
    }
}

@Composable
private fun ProfileSection(user: three.two.bit.ppt.reality.auth.SsoUserInfo) {
    Card(modifier = Modifier.fillMaxWidth()) {
        Row(
            modifier = Modifier.fillMaxWidth().padding(16.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            AsyncImage(
                model =
                    ImageRequest.Builder(LocalContext.current)
                        .data(user.avatarUrl ?: "")
                        .crossfade(true)
                        .build(),
                contentDescription = user.name,
                contentScale = ContentScale.Crop,
                modifier =
                    Modifier.size(72.dp)
                        .clip(CircleShape)
                        .background(MaterialTheme.colorScheme.surfaceVariant)
            )

            Spacer(modifier = Modifier.width(16.dp))

            Column(modifier = Modifier.weight(1f)) {
                Text(
                    text = user.name,
                    style = MaterialTheme.typography.titleLarge,
                    fontWeight = FontWeight.Bold
                )
                Text(
                    text = user.email,
                    style = MaterialTheme.typography.bodyMedium,
                    color = MaterialTheme.colorScheme.onSurfaceVariant
                )
            }

            IconButton(onClick = { /* Edit profile */}) {
                Icon(Icons.Default.Edit, contentDescription = stringResource(R.string.edit_profile))
            }
        }
    }
}

@Composable
private fun NotificationPreferencesSection(
    preferences: NotificationPreferences,
    onPreferenceChange: (NotificationPreferences) -> Unit
) {
    Card(modifier = Modifier.fillMaxWidth()) {
        Column(modifier = Modifier.padding(16.dp)) {
            Text(
                text = stringResource(R.string.notifications),
                style = MaterialTheme.typography.titleMedium,
                fontWeight = FontWeight.Bold
            )

            Spacer(modifier = Modifier.height(16.dp))

            NotificationPreferenceItem(
                title = stringResource(R.string.notification_new_listings),
                description = stringResource(R.string.notification_new_listings_desc),
                checked = preferences.newListings,
                onCheckedChange = { onPreferenceChange(preferences.copy(newListings = it)) }
            )

            HorizontalDivider()

            NotificationPreferenceItem(
                title = stringResource(R.string.notification_price_drops),
                description = stringResource(R.string.notification_price_drops_desc),
                checked = preferences.priceDrops,
                onCheckedChange = { onPreferenceChange(preferences.copy(priceDrops = it)) }
            )

            HorizontalDivider()

            NotificationPreferenceItem(
                title = stringResource(R.string.notification_inquiry_responses),
                description = stringResource(R.string.notification_inquiry_responses_desc),
                checked = preferences.inquiryResponses,
                onCheckedChange = { onPreferenceChange(preferences.copy(inquiryResponses = it)) }
            )

            HorizontalDivider()

            NotificationPreferenceItem(
                title = stringResource(R.string.notification_listing_updates),
                description = stringResource(R.string.notification_listing_updates_desc),
                checked = preferences.listingUpdates,
                onCheckedChange = { onPreferenceChange(preferences.copy(listingUpdates = it)) }
            )

            HorizontalDivider()

            NotificationPreferenceItem(
                title = stringResource(R.string.notification_marketing),
                description = stringResource(R.string.notification_marketing_desc),
                checked = preferences.marketing,
                onCheckedChange = { onPreferenceChange(preferences.copy(marketing = it)) }
            )
        }
    }
}

@Composable
private fun NotificationPreferenceItem(
    title: String,
    description: String,
    checked: Boolean,
    onCheckedChange: (Boolean) -> Unit
) {
    Row(
        modifier = Modifier.fillMaxWidth().padding(vertical = 12.dp),
        horizontalArrangement = Arrangement.SpaceBetween,
        verticalAlignment = Alignment.CenterVertically
    ) {
        Column(modifier = Modifier.weight(1f)) {
            Text(text = title, style = MaterialTheme.typography.bodyLarge)
            Text(
                text = description,
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant
            )
        }

        Switch(checked = checked, onCheckedChange = onCheckedChange)
    }
}

@Composable
private fun AppSettingsSection() {
    Card(modifier = Modifier.fillMaxWidth()) {
        Column(modifier = Modifier.padding(16.dp)) {
            Text(
                text = stringResource(R.string.section_app_settings),
                style = MaterialTheme.typography.titleMedium,
                fontWeight = FontWeight.Bold
            )

            Spacer(modifier = Modifier.height(8.dp))

            SettingsItem(
                icon = Icons.Default.Language,
                title = stringResource(R.string.setting_language),
                value = stringResource(R.string.language_english),
                onClick = { /* Open language picker */}
            )

            HorizontalDivider()

            SettingsItem(
                icon = Icons.Default.Euro,
                title = stringResource(R.string.setting_currency),
                value = stringResource(R.string.currency_eur),
                onClick = { /* Open currency picker */}
            )

            HorizontalDivider()

            SettingsItem(
                icon = Icons.Default.Straighten,
                title = stringResource(R.string.setting_units),
                value = stringResource(R.string.units_metric),
                onClick = { /* Open units picker */}
            )

            HorizontalDivider()

            SettingsItem(
                icon = Icons.Default.DarkMode,
                title = stringResource(R.string.setting_theme),
                value = stringResource(R.string.theme_system),
                onClick = { /* Open theme picker */}
            )
        }
    }
}

@Composable
private fun SettingsItem(
    icon: androidx.compose.ui.graphics.vector.ImageVector,
    title: String,
    value: String,
    onClick: () -> Unit
) {
    Row(
        modifier = Modifier.fillMaxWidth().padding(vertical = 12.dp).clickable(onClick = onClick),
        verticalAlignment = Alignment.CenterVertically
    ) {
        Icon(
            icon,
            contentDescription = null,
            modifier = Modifier.size(24.dp),
            tint = MaterialTheme.colorScheme.primary
        )

        Spacer(modifier = Modifier.width(16.dp))

        Column(modifier = Modifier.weight(1f)) {
            Text(text = title, style = MaterialTheme.typography.bodyLarge)
        }

        Text(
            text = value,
            style = MaterialTheme.typography.bodyMedium,
            color = MaterialTheme.colorScheme.onSurfaceVariant
        )

        Icon(
            Icons.Default.ChevronRight,
            contentDescription = null,
            tint = MaterialTheme.colorScheme.onSurfaceVariant
        )
    }
}

@Composable
private fun AboutSection() {
    Card(modifier = Modifier.fillMaxWidth()) {
        Column(modifier = Modifier.padding(16.dp)) {
            Text(
                text = stringResource(R.string.about),
                style = MaterialTheme.typography.titleMedium,
                fontWeight = FontWeight.Bold
            )

            Spacer(modifier = Modifier.height(8.dp))

            AboutItem(
                icon = Icons.Default.Info,
                title = stringResource(R.string.version),
                value = BuildConfig.VERSION_NAME
            )

            HorizontalDivider()

            AboutItem(
                icon = Icons.Default.Description,
                title = stringResource(R.string.terms_of_service),
                onClick = { /* Open terms */}
            )

            HorizontalDivider()

            AboutItem(
                icon = Icons.Default.PrivacyTip,
                title = stringResource(R.string.privacy_policy),
                onClick = { /* Open privacy policy */}
            )

            HorizontalDivider()

            AboutItem(
                icon = Icons.Default.Help,
                title = stringResource(R.string.help_support),
                onClick = { /* Open support */}
            )

            HorizontalDivider()

            AboutItem(
                icon = Icons.Default.Feedback,
                title = stringResource(R.string.send_feedback),
                onClick = { /* Open feedback */}
            )
        }
    }
}

@Composable
private fun AboutItem(
    icon: androidx.compose.ui.graphics.vector.ImageVector,
    title: String,
    value: String? = null,
    onClick: (() -> Unit)? = null
) {
    Row(
        modifier = Modifier.fillMaxWidth().padding(vertical = 12.dp),
        verticalAlignment = Alignment.CenterVertically
    ) {
        Icon(
            icon,
            contentDescription = null,
            modifier = Modifier.size(24.dp),
            tint = MaterialTheme.colorScheme.primary
        )

        Spacer(modifier = Modifier.width(16.dp))

        Text(
            text = title,
            style = MaterialTheme.typography.bodyLarge,
            modifier = Modifier.weight(1f)
        )

        if (value != null) {
            Text(
                text = value,
                style = MaterialTheme.typography.bodyMedium,
                color = MaterialTheme.colorScheme.onSurfaceVariant
            )
        }

        if (onClick != null) {
            Icon(
                Icons.Default.ChevronRight,
                contentDescription = null,
                tint = MaterialTheme.colorScheme.onSurfaceVariant
            )
        }
    }
}
