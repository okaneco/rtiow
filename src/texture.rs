//! Texture objects.

use crate::vec3::{Color, Point3};
use std::sync::Arc;

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
