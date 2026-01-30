// Round to 2 decimal places for currency
pub fn round_currency(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}
