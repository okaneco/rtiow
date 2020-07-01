//! Various utility functions for faster conversion between float and integers.

/// The numeric constant PI.
pub const PI: f64 = core::f64::consts::PI;
/// The numeric constant PI multiplied by 2.
pub const TWO_PI: f64 = 2.0 * core::f64::consts::PI;

// C23 = 2^23, in f32
// C52 = 2^52, in f64
const C23: u32 = 0x4b00_0000;
const C52: u64 = 0x4330_0000_0000_0000;

/// Trait for fast conversion from float to u8.
pub trait IntoU8 {
    /// Convert and clamp float input to u8.
    fn into_u8(self) -> u8;
}

impl IntoU8 for f32 {
    #[inline]
    fn into_u8(self) -> u8 {
        let max = u8::MAX as f32;
        let scaled = (self * max).min(max);
        let f = scaled + f32::from_bits(C23);
        (f.to_bits().saturating_sub(C23)) as u8
    }
}

impl IntoU8 for f64 {
    #[inline]
    fn into_u8(self) -> u8 {
        let max = u8::MAX as f64;
        let scaled = (self * max).min(max);
        let f = scaled + f64::from_bits(C52);
        (f.to_bits().saturating_sub(C52)) as u8
    }
}

impl crate::vec3::Color {
    /// Convert a float RGB color into u8 with gamma correction.
    pub fn into_u8_color(self, samples: f64) -> crate::vec3::ColorU8 {
        let scale = samples.recip();

        crate::vec3::ColorU8(
            crate::conversion::IntoU8::into_u8((self.0 * scale).sqrt()),
            crate::conversion::IntoU8::into_u8((self.1 * scale).sqrt()),
            crate::conversion::IntoU8::into_u8((self.2 * scale).sqrt()),
        )
    }
}

/// Trait for fast conversion from uint to f64.
pub trait IntoF64 {
    /// Convert unsigned integer to f64.
    fn into_f64(self) -> f64;
}

impl IntoF64 for u8 {
    fn into_f64(self) -> f64 {
        let comp_u = self as u64 + C52;
        let comp_f = f64::from_bits(comp_u) - f64::from_bits(C52);
        let max_u = core::u8::MAX as u64 + C52;
        let max_f = (f64::from_bits(max_u) - f64::from_bits(C52)).recip();
        comp_f * max_f
    }
}
