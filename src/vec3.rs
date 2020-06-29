//! Vector class from which the `Color` and `Point3` tuples are derived.

use rand::Rng;

/// General purpose Vector3 struct, basis for `Color` and `Point3` struct types.
#[derive(Copy, Clone, Debug, Default)]
pub struct Vec3(pub f64, pub f64, pub f64);

/// Struct for 8-bit color used in image output.
#[derive(Copy, Clone, Debug, Default)]
pub struct ColorU8(pub u8, pub u8, pub u8);

/// Color struct holding (R, G, B).
pub type Color = crate::vec3::Vec3;
/// Point struct holding (x, y, z).
pub type Point3 = crate::vec3::Vec3;

const TWO_PI: f64 = 2.0 * core::f64::consts::PI;

impl Vec3 {
    /// Create a new `Vec3`.
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Self(a, b, c)
    }

    /// Create a new `Vec3` with the same value for all fields.
    pub fn new_with(a: f64) -> Self {
        Self(a, a, a)
    }

    /// Return the first element of the tuple.
    pub fn x(&self) -> f64 {
        self.0
    }

    /// Return the second element of the tuple.
    pub fn y(&self) -> f64 {
        self.1
    }

    /// Return the third element of the tuple.
    pub fn z(&self) -> f64 {
        self.2
    }

    /// Return the length of the vector.
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Return the squared length of the vector.
    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    /// Return the dot product of the vector and another `v`.
    pub fn dot(&self, v: &Self) -> f64 {
        self.0 * v.0 + self.1 * v.1 + self.2 * v.2
    }

    /// Return the cross product of the vector and another `v`.
    pub fn cross(&self, v: &Self) -> Self {
        Vec3(
            self.1 * v.2 - self.2 * v.1,
            self.2 * v.0 - self.0 * v.2,
            self.0 * v.1 - self.1 * v.0,
        )
    }

    /// Return the unit vector.
    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    /// Generate a Vec3 with range `[0.0, 1.0)` for each element.
    #[inline]
    pub fn random<R: Rng>(rng: &mut R) -> Self {
        Vec3(rng.gen(), rng.gen(), rng.gen())
    }

    /// Generate a Vec3 with range `[min, max)` for each element.
    #[inline]
    pub fn random_range<R: Rng>(rng: &mut R, min: f64, max: f64) -> Self {
        Vec3(
            rng.gen_range(min, max),
            rng.gen_range(min, max),
            rng.gen_range(min, max),
        )
    }

    /// Create a random unit vector.
    #[inline]
    pub fn random_unit_vector<R: Rng>(rng: &mut R) -> Self {
        let a = rng.gen_range(0.0, TWO_PI);
        let z = rng.gen_range(-1.0, 1.0);
        let r = f64::sqrt(1.0 - z * z);
        Vec3(r * a.cos(), r * a.sin(), z)
    }

    /// Sample a circle uniformly.
    #[inline]
    pub fn random_in_unit_circle<R: Rng>(rng: &mut R) -> Self {
        let r = rng.gen::<f64>().sqrt();
        let theta = rng.gen::<f64>() * TWO_PI;
        Self::new(r * theta.cos(), r * theta.sin(), 0.0)
    }

    /// Create a random vector in a unit sphere.
    #[inline]
    pub fn random_in_unit_sphere<R: Rng>(rng: &mut R) -> Self {
        loop {
            let p = Self::random_range(rng, -1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    /// Create a random vector in a hemisphere.
    #[inline]
    pub fn random_in_hemisphere<R: Rng>(rng: &mut R, normal: &Self) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere(rng);
        if in_unit_sphere.dot(&normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    /// Calculate the reflection of a vector and normal `n`.
    #[inline]
    pub fn reflect(v: &Self, n: &Self) -> Self {
        *v - (v.dot(&n) * 2.0) * *n
    }

    /// Calculate the reflection of a vector and normal `n` and etas
    /// `etai_over_etat`.
    pub fn refract(uv: &Self, n: &Self, etai_over_etat: f64) -> Self {
        let cos_theta = (-*uv).dot(n);
        let r_out_parallel = etai_over_etat * (*uv + cos_theta * *n);
        let r_out_perp = -(1.0 - r_out_parallel.length_squared()).sqrt() * *n;
        r_out_parallel + r_out_perp
    }
}

impl core::ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl core::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl core::ops::Add<Vec3> for f64 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3(self + rhs.0, self + rhs.1, self + rhs.2)
    }
}

impl core::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

impl core::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl core::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.0 -= other.0;
        self.1 -= other.1;
        self.2 -= other.2;
    }
}

impl core::ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Vec3(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl core::ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        self.0 *= other.0;
        self.1 *= other.1;
        self.2 *= other.2;
    }
}

impl core::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl core::ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl core::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl core::ops::Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3(
            self.recip() * rhs.0,
            self.recip() * rhs.1,
            self.recip() * rhs.2,
        )
    }
}

impl core::ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Vec3(
            self.0 * rhs.recip(),
            self.1 * rhs.recip(),
            self.2 * rhs.recip(),
        )
    }
}

impl core::ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.0 *= rhs.recip();
        self.1 *= rhs.recip();
        self.2 *= rhs.recip();
    }
}

impl core::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}

impl core::iter::Sum for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self(0.0, 0.0, 0.0), |a, b| a + b)
    }
}

impl From<ColorU8> for std::vec::Vec<u8> {
    fn from(other: ColorU8) -> Self {
        [other.0, other.1, other.2].to_vec()
    }
}
