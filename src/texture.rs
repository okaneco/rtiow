//! Texture objects.

use std::sync::Arc;

use crate::perlin::NoiseType;
use crate::vec3::{Color, Point3};

/// A trait for procedural or lookup textures.
pub trait Texture {
    /// Calculate the value of a texture based on the surface coordinates.
    fn value(&self, u: f64, v: f64, p: &Point3) -> crate::vec3::Color;
}

/// Texture with one color.
#[derive(Clone, Copy, Debug, Default)]
pub struct SolidColor {
    /// Color of the texture.
    color: Color,
}

impl SolidColor {
    /// Create a new solid color texture.
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Self {
            color: Color::new(a, b, c),
        }
    }

    /// Create a new solid color texture with the same value for all fields.
    pub fn new_with(a: f64) -> Self {
        Self {
            color: Color::new(a, a, a),
        }
    }

    /// Create a new solid color texture from a color.
    pub fn from_color(color: Color) -> Self {
        Self { color }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.color
    }
}

/// Texture with one color.
#[derive(Clone)]
pub struct Checker {
    /// Odd pattern.
    pub odd: Arc<dyn Texture + Send + Sync>,
    /// Even pattern.
    pub even: Arc<dyn Texture + Send + Sync>,
}

impl Checker {
    /// Create new checker texture.
    pub fn new(odd: Arc<dyn Texture + Send + Sync>, even: Arc<dyn Texture + Send + Sync>) -> Self {
        Self { odd, even }
    }
}

impl Texture for Checker {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

#[derive(Clone, Debug)]
/// Perlin noise texture.
pub struct Noise {
    /// Perlin noise generator.
    pub noise: crate::perlin::Perlin,
    /// Base color of the texture.
    pub albedo: Color,
    /// Type of noise to generate.
    pub noise_type: NoiseType,
    /// Scale of noise frequency.
    pub scale: f64,
    /// Depth of recursion for turbulence calculation.
    pub turb_depth: u32,
    /// Phase of `Marble` texture.
    pub phase: f64,
}

impl Noise {
    /// Create a new Perlin noise texture.
    pub fn new(
        red: f64,
        green: f64,
        blue: f64,
        noise_type: NoiseType,
        scale: f64,
        turb_depth: u32,
        phase: f64,
    ) -> Self {
        Self {
            noise: crate::perlin::Perlin::new(),
            albedo: Color::new(red, green, blue),
            noise_type,
            scale,
            turb_depth,
            phase,
        }
    }

    /// Create a new Perlin noise texture with the same value for all color
    /// fields.
    pub fn new_with(
        color: f64,
        noise_type: NoiseType,
        scale: f64,
        turb_depth: u32,
        phase: f64,
    ) -> Self {
        Self {
            noise: crate::perlin::Perlin::new(),
            albedo: Color::new_with(color),
            noise_type,
            scale,
            turb_depth,
            phase,
        }
    }

    /// Create a new Perlin noise texture from another color.
    pub fn from_color(
        albedo: Color,
        noise_type: NoiseType,
        scale: f64,
        turb_depth: u32,
        phase: f64,
    ) -> Self {
        Self {
            noise: crate::perlin::Perlin::new(),
            albedo,
            noise_type,
            scale,
            turb_depth,
            phase,
        }
    }
}

impl core::default::Default for Noise {
    fn default() -> Self {
        Self {
            noise: crate::perlin::Perlin::new(),
            albedo: Color::new_with(1.0),
            noise_type: NoiseType::default(),
            scale: 4.0,
            turb_depth: 7,
            phase: 10.0,
        }
    }
}

impl Texture for Noise {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> crate::vec3::Color {
        match self.noise_type {
            NoiseType::Square | NoiseType::Trilinear => {
                self.albedo * self.noise.noise(&(self.scale * *p), self.noise_type)
            }
            NoiseType::Smooth => {
                self.albedo * 0.5 * (1.0 + self.noise.noise(&(self.scale * *p), self.noise_type))
            }
            NoiseType::Marble => {
                self.albedo
                    * 0.5
                    * (1.0
                        + (self.scale * p.z()
                            + self.phase * self.noise.turb(p, self.turb_depth, self.noise_type))
                        .sin())
            }
            NoiseType::Net => self.albedo * self.noise.turb(p, self.turb_depth, self.noise_type),
        }
    }
}

/// Texture that holds image data.
#[derive(Clone, Debug, Default)]
pub struct ImageTexture {
    /// Buffer of pixels.
    data: Vec<u8>,
    /// Width of image.
    width: u32,
    /// Height of image.
    height: u32,
    /// Number of bytes per pixel.
    bytes_per_pixel: u32,
    /// Number of bytes per line of image.
    bytes_per_scanline: u32,
}

impl ImageTexture {
    /// Create new `ImageTexture` from file.
    pub fn new<P: AsRef<std::path::Path>>(filename: P) -> Result<Self, Box<dyn std::error::Error>> {
        let bytes_per_pixel = 3;
        let img = image::open(filename)?.to_rgb();
        let (width, height) = img.dimensions();
        let data = img.into_raw();

        Ok(Self {
            data,
            width,
            height,
            bytes_per_pixel,
            bytes_per_scanline: bytes_per_pixel * width,
        })
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point3) -> Color {
        if self.data.is_empty() {
            return Color::new(1.0, 0.0, 1.0);
        }

        let u = u.max(0.0).min(1.0);
        let v = 1.0 - v.max(0.0).min(1.0);

        let mut i = (u * f64::from(self.width)) as u32;
        let mut j = (v * f64::from(self.height)) as u32;

        if i >= self.width {
            i = self.width - 1;
        }
        if j >= self.height {
            j = self.height - 1;
        }

        let pixel = (j * self.bytes_per_scanline + i * self.bytes_per_pixel) as usize;

        Color::new(
            crate::conversion::IntoF64::into_f64(self.data[pixel]),
            crate::conversion::IntoF64::into_f64(self.data[pixel + 1]),
            crate::conversion::IntoF64::into_f64(self.data[pixel + 2]),
        )
    }
}
