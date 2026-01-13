//! Tests for multi-currency repository (Epic 145).

#[cfg(test)]
mod tests {
    use super::super::multi_currency::MultiCurrencyRepository;
    use crate::models::multi_currency::{
        CreateCurrencyConfig, CreateExchangeRate, ExchangeRateSource, SupportedCurrency,
    };

    // Note: These tests require a database connection and should be run
    // in an integration test environment with proper setup/teardown.

    #[test]
    fn test_supported_currency_display() {
        assert_eq!(SupportedCurrency::EUR.to_string(), "EUR");
        assert_eq!(SupportedCurrency::CZK.to_string(), "CZK");
        assert_eq!(SupportedCurrency::CHF.to_string(), "CHF");
        assert_eq!(SupportedCurrency::GBP.to_string(), "GBP");
        assert_eq!(SupportedCurrency::PLN.to_string(), "PLN");
    }

    #[test]
    fn test_default_currency() {
        let default = SupportedCurrency::default();
        assert_eq!(default, SupportedCurrency::EUR);
    }

    #[test]
    fn test_default_exchange_rate_source() {
        let default = ExchangeRateSource::default();
        assert_eq!(default, ExchangeRateSource::Ecb);
    }

    #[test]
    fn test_create_currency_config_defaults() {
        let json = r#"{"base_currency": "EUR"}"#;
        let config: CreateCurrencyConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.base_currency, SupportedCurrency::EUR);
        assert!(config.show_original_amount);
        assert_eq!(config.decimal_places, 2);
        assert!(config.auto_update_rates);
        assert_eq!(config.update_frequency_hours, 24);
        assert_eq!(config.rounding_mode, "half_up");
    }
}
