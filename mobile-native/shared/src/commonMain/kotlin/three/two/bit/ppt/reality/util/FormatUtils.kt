package three.two.bit.ppt.reality.util

import three.two.bit.ppt.reality.listing.Address

/**
 * Shared formatting utilities.
 *
 * Epic 48 - Code Review Fix: Extract duplicated formatting functions
 */
object FormatUtils {
    /**
     * Format a price amount with currency symbol.
     *
     * @param price The price in minor units (e.g., cents)
     * @param currency The ISO currency code (e.g., EUR, USD, GBP)
     * @return Formatted price string with currency symbol
     */
    fun formatPrice(price: Long, currency: String): String {
        val formatted = formatPriceAmount(price)
        val symbol = currencySymbol(currency)
        return symbol?.let { "$it$formatted" } ?: "$formatted $currency"
    }

    /**
     * Format a price amount for display.
     * - Values >= 1M are shown as "X.XXM"
     * - Values >= 1K are shown with thousands separator
     * - Values < 1K are shown as-is
     */
    private fun formatPriceAmount(price: Long): String {
        return when {
            price >= 1_000_000 -> String.format("%.2fM", price / 1_000_000.0)
            price >= 1_000 -> String.format("%,d", price)
            else -> price.toString()
        }
    }

    /** Get the symbol for a currency code. */
    private fun currencySymbol(currency: String): String? {
        return when (currency) {
            "EUR" -> "\u20AC" // Euro sign
            "USD" -> "$"
            "GBP" -> "\u00A3" // Pound sign
            else -> null
        }
    }

    /**
     * Build a location string from an address.
     *
     * @param address The address to format
     * @param includeStreet Whether to include street in the output
     * @param includePostalCode Whether to include postal code in the output
     * @return Formatted location string with parts joined by ", "
     */
    fun buildLocationString(
        address: Address,
        includeStreet: Boolean = false,
        includePostalCode: Boolean = false
    ): String {
        val parts = mutableListOf<String>()

        if (includeStreet) {
            address.street?.let { parts.add(it) }
        }

        if (includePostalCode) {
            address.postalCode?.let { parts.add(it) }
        }

        // Always include district, city, region
        address.district?.let { parts.add(it) }
        address.city?.let { parts.add(it) }
        address.region?.let { parts.add(it) }

        return parts.joinToString(", ")
    }

    /** Build a simple location string (district, city, region only). */
    fun buildSimpleLocationString(address: Address): String {
        return buildLocationString(address, includeStreet = false, includePostalCode = false)
    }

    /** Build a detailed location string (including street and postal code). */
    fun buildDetailedLocationString(address: Address): String {
        return buildLocationString(address, includeStreet = true, includePostalCode = true)
    }
}
