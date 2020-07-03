//! Generate Perlin noise textures.

use crate::vec3::{Point3, Vec3};
use rand::Rng;

/// Type of noise.
///
/// Perlin noise is filtered to get different appearances.
#[derive(Clone, Copy, Debug)]
pub enum NoiseType {
    /// Unfiltered noise.
    Square,
    /// Smooth interpolation with random unit vectors.
    Smooth,
    /// Marble pattern with adjustable phase.
    Marble,
    /// Turbulent pattern that resembles a net.
    Net,
    /// Trilinear interpolation.
    Trilinear,
}
/// Perlin noise generator.
#[derive(Clone, Debug, Default)]
pub struct Perlin {
    point_count: usize,
    ranfloat: Vec<f64>,
    ranvec: Vec<Vec3>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    /// Initialize Perlin noise texture.
    pub fn new() -> Self {
        let point_count = 256;
        let mut rng = rand::thread_rng();

        fn perlin_generate_permute<R: rand::Rng>(rng: &mut R, point_count: usize) -> Vec<i32> {
            let mut p: Vec<i32> = (0..256).map(|i| i).collect();
            for i in (1..point_count).rev() {
                let target = rng.gen_range(0, i);
                p.swap(i, target);
            }

            p
        }

        Self {
            point_count,
            ranfloat: (0..256).map(|_| rng.gen()).collect(),
            ranvec: (0..256)
                .map(|_| Vec3::random_range(&mut rng, -1.0, 1.0).unit_vector())
                .collect(),
            perm_x: perlin_generate_permute(&mut rng, point_count),
            perm_y: perlin_generate_permute(&mut rng, point_count),
            perm_z: perlin_generate_permute(&mut rng, point_count),
        }
    }

    /// Hash the generated noise.
    pub fn noise(&self, p: &crate::vec3::Point3, noise: NoiseType) -> f64 {
        match noise {
            NoiseType::Square => {
                let i = (4.0 * p.x()) as i32 & 255;
                let j = (4.0 * p.y()) as i32 & 255;
                let k = (4.0 * p.z()) as i32 & 255;

                self.ranfloat[(self.perm_x[i as usize]
                    ^ self.perm_y[j as usize]
                    ^ self.perm_z[k as usize]) as usize]
            }
            NoiseType::Trilinear => {
                let i = p.x().floor();
                let j = p.y().floor();
                let k = p.z().floor();

                let mut u = p.x() - i;
                let mut v = p.y() - j;
                let mut w = p.z() - k;

                /* cubic Hermite smoothing */
                u = u * u * (3.0 - 2.0 * u);
                v = v * v * (3.0 - 2.0 * v);
                w = w * w * (3.0 - 2.0 * w);

                let mut c = [[[0.0f64; 2]; 2]; 2];
                for di in 0..2 {
                    for dj in 0..2 {
                        for dk in 0..2 {
                            c[di][dj][dk] = self.ranfloat[(self.perm_x[(i as usize + di) & 255]
                                ^ self.perm_y[((j as usize + dj) & 255)]
                                ^ self.perm_z[((k as usize + dk) & 255)])
                                as usize];
                        }
                    }
                }

                trilinear_interp(c, u, v, w)
            }
            _ => {
                let i = p.x().floor();
                let j = p.y().floor();
                let k = p.z().floor();

                let mut u = p.x() - i;
                let mut v = p.y() - j;
                let mut w = p.z() - k;

                /* cubic Hermite smoothing */
                u = u * u * (3.0 - 2.0 * u);
                v = v * v * (3.0 - 2.0 * v);
                w = w * w * (3.0 - 2.0 * w);

                let mut c = [[[Vec3::default(); 2]; 2]; 2];
                for di in 0..2 {
                    for dj in 0..2 {
                        for dk in 0..2 {
                            c[di][dj][dk] = self.ranvec[(self.perm_x[(i as usize + di) & 255]
                                ^ self.perm_y[((j as usize + dj) & 255)]
                                ^ self.perm_z[((k as usize + dk) & 255)])
                                as usize];
                        }
                    }
                }

                Self::perlin_interp(c, u, v, w)
            }
        }
    }

    #[inline]
    fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut acc = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    acc += (i as f64 * u + (1 - i) as f64 * (1.0 - u))
                        * (j as f64 * v + (1 - j) as f64 * (1.0 - v))
                        * (k as f64 * w + (1 - k) as f64 * (1.0 - w))
                        * (c[i][j][k]).dot(&weight_v);
                }
            }
        }

        acc
    }

    /// Turbulence noise calculation.
    pub fn turb(&self, p: &Point3, depth: u32, noise_type: NoiseType) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * Self::noise(self, &temp_p, noise_type);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }
}

impl core::default::Default for NoiseType {
    fn default() -> Self {
        Self::Square
    }
}

/// Trilinear interpolation.
#[inline]
pub fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let mut acc = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                acc += (i as f64 * u + (1 - i) as f64 * (1.0 - u))
                    * (j as f64 * v + (1 - j) as f64 * (1.0 - v))
                    * (k as f64 * w + (1 - k) as f64 * (1.0 - w))
                    * c[i][j][k];
            }
        }
    }

    acc
}
