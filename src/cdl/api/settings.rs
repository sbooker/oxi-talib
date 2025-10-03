/// Holds configuration parameters for the pattern recognition algorithms.
///
/// These settings define how properties of candles, such as body size or
/// shadow length, are interpreted.
///
/// Use `Settings::default()` to get standard values, then modify fields as needed.
/// The created struct should be passed to [`crate::cdl::engines::talib::engine::configure`].
#[derive(Debug, Clone)]
pub struct Settings {
    /// The lookback period for calculating average candle component sizes.
    pub period: i32,
    /// Factor for a candle body to be considered "long".
    pub body_long_factor: f64,
    /// Factor for a candle body to be considered "very long".
    pub body_very_long_factor: f64,
    /// Factor for a candle body to be considered "short".
    pub body_short_factor: f64,
    /// Factor for a candle body to be considered a "doji".
    pub body_doji_factor: f64,
    /// Factor for a candle shadow to be considered "long".
    pub shadow_long_factor: f64,
    /// Factor for a candle shadow to be considered "very long".
    pub shadow_very_long_factor: f64,
    /// Factor for a candle shadow to be considered "short".
    pub shadow_short_factor: f64,
    /// Factor for a candle shadow to be considered "very short".
    pub shadow_very_short_factor: f64,
    /// Factor for two prices to be considered "near" each other.
    pub near_factor: f64,
    /// Factor for two prices to be considered "far" from each other.
    pub far_factor: f64,
    /// Factor for two prices to be considered "equal".
    pub equal_factor: f64,
    /// Penetration factor for `MorningStar` and `EveningStar` patterns. Default is 0.3.
    pub star_penetration_factor: f64,
    /// Penetration factor for `DarkCloudCover` and `PiercingLine` patterns. Default is 0.5.
    pub piercing_penetration_factor: f64,
}

impl Default for Settings {
    /// Creates `Settings` with balanced default values.
    ///
    /// These values are tuned for general use and may differ from the original
    /// defaults of the TA-Lib C library.
    fn default() -> Self {
        Self {
            period: 10,
            body_doji_factor: 0.1,
            shadow_short_factor: 0.1,
            equal_factor: 0.25,
            body_long_factor: 1.0,
            body_very_long_factor: 3.0,
            body_short_factor: 1.0,
            shadow_long_factor: 1.0,
            shadow_very_long_factor: 2.0,
            shadow_very_short_factor: 0.05,
            near_factor: 0.2,
            far_factor: 0.6,
            star_penetration_factor: 0.3,
            piercing_penetration_factor: 0.5,
        }
    }
}
