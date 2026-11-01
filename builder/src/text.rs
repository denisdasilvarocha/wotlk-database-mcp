//! Shared number rendering for client data.

/// Converts the shortest decimal form of a DBC float without binary f32 noise.
pub(crate) fn real(value: f32) -> f64 {
    value.to_string().parse().unwrap_or_else(|_| f64::from(value))
}
