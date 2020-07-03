//! Orthonormal basis module, used for generating random directions relative to
//! the surface normal.

use crate::vec3::Vec3;

/// Orthonormal basis.
#[derive(Clone, Default)]
pub struct Onb {
    /// Three mutually orthogonal unit vectors.
    pub axis: [Vec3; 3],
}

impl Onb {
    /// Create a new ONB from a `Vec3`.
    pub fn build_from_w(n: &Vec3) -> Self {
        let w = n.unit_vector();
        let a = if w.x().abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let v = w.cross(&a).unit_vector();
        let u = w.cross(&v);

        Self { axis: [u, v, w] }
    }

    /// Return the `u` unit vector.
    pub fn u(&self) -> Vec3 {
        self.axis[0]
    }

    /// Return the `v` unit vector.
    pub fn v(&self) -> Vec3 {
        self.axis[1]
    }

    /// Return the `w` unit vector.
    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }

    /// Return a local from a `Vec3`.
    pub fn local(&self, a: &Vec3) -> Vec3 {
        a.x() * self.u() + a.y() * self.v() + a.z() * self.w()
    }

    /// Return a local from three `f64`s.
    pub fn local_from(&self, a: f64, b: f64, c: f64) -> Vec3 {
        a * self.u() + b * self.v() + c * self.w()
    }
}
