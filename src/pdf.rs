//! Probability distribution module.

use std::sync::Arc;

use rand::Rng;

use crate::hittable::Hittable;
use crate::onb::Onb;
use crate::vec3::{Point3, Vec3};

/// Trait for implementing PDFs.
pub trait Pdf {
    /// Return the value from a PDF.
    fn value(&self, direction: &Vec3) -> f64;
    /// Generate the direction from a PDF.
    fn generate(&self, rng: &mut rand::rngs::ThreadRng) -> Vec3;
}

/// Cosine probability distribution struct holding an orthonormal basis.
#[derive(Clone, Default)]
pub struct CosPdf {
    /// Orthonormal basis of cosine PDF.
    pub uvw: Onb,
}

impl CosPdf {
    /// Create a new cosine PDF.
    pub fn new(w: &Vec3) -> Self {
        Self {
            uvw: Onb::build_from_w(&w),
        }
    }

    /// Return a `Vec3` random cosine direction.
    pub fn random_cosine_direction<R: rand::Rng>(rng: &mut R) -> Vec3 {
        let r1 = rng.gen::<f64>();
        let r2 = rng.gen::<f64>();
        let z = (1.0 - r2).sqrt();

        let phi = crate::conversion::TWO_PI * r1;
        let x = phi.cos() * r2.sqrt();
        let y = phi.sin() * r2.sqrt();

        crate::vec3::Vec3::new(x, y, z)
    }
}

impl Pdf for CosPdf {
    fn value(&self, direction: &Vec3) -> f64 {
        let cosine = direction.unit_vector().dot(&self.uvw.w());
        if cosine <= 0.0 {
            0.0
        } else {
            cosine * core::f64::consts::FRAC_1_PI
        }
    }

    fn generate(&self, rng: &mut rand::rngs::ThreadRng) -> Vec3 {
        self.uvw.local(&CosPdf::random_cosine_direction(rng))
    }
}

/// Sampling that directs light towards a hittable object.
#[derive(Clone)]
pub struct HittablePdf {
    /// Origin of object.
    pub origin: Point3,
    /// Pointer to the hittable object.
    pub pointer: Arc<dyn Hittable + Send + Sync>,
}

impl HittablePdf {
    /// Create a new `HittablePDF` object.
    pub fn new(origin: &Point3, pointer: Arc<dyn Hittable + Send + Sync>) -> Self {
        Self {
            origin: *origin,
            pointer,
        }
    }
}

impl Pdf for HittablePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        self.pointer.pdf_value(&self.origin, direction)
    }

    fn generate(&self, rng: &mut rand::rngs::ThreadRng) -> Vec3 {
        self.pointer.random(rng, &self.origin)
    }
}

/// Struct for mixing the densities of PDFs.
#[derive(Clone)]
pub struct MixturePdf {
    /// Vec of probability density functions.
    pub p: Vec<Arc<dyn Pdf>>,
}

impl MixturePdf {
    /// Create a new `MixturePdf`.
    pub fn new(p: Vec<Arc<dyn Pdf>>) -> Self {
        Self { p }
    }
}

impl Pdf for MixturePdf {
    fn value(&self, direction: &Vec3) -> std::primitive::f64 {
        if self.p.is_empty() {
            return 0.0;
        }

        // Mix all the Pdfs together evenly
        let factor = (self.p.len() as f64).recip();
        self.p
            .iter()
            .fold(0.0, |acc, x| acc + (factor * x.value(direction)))
    }

    fn generate(&self, rng: &mut rand::prelude::ThreadRng) -> Vec3 {
        self.p
            .get(rng.gen_range(0, self.p.len()))
            .map_or_else(|| Vec3::default(), |pdf| pdf.generate(rng))
    }
}
