use crate::conversion::{PI, TWO_PI};
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
/// Sphere object.
#[derive(Clone, Default)]
pub struct Sphere {
    /// Center point of sphere.
    pub center: Point3,
    /// Radius of sphere.
    pub radius: f64,
    /// Material of sphere.
    pub material: std::sync::Arc<Material>,
}

impl Sphere {
    /// Create a new sphere.
    pub fn new(center: Point3, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            material: std::sync::Arc::new(material),
        }
    }
}

/// Utitilfy function for calculating the texture coordinates of a sphere.
pub fn get_sphere_uv(p: &Vec3, u: &mut f64, v: &mut f64) {
    let phi = p.z().atan2(p.x());
    let theta = p.y().asin();
    *u = 1.0 - (phi + PI) * TWO_PI.recip();
    *v = (theta + core::f64::consts::FRAC_PI_2) * PI.recip();
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(&r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
                get_sphere_uv(
                    &((rec.p - self.center) * self.radius.recip()),
                    &mut rec.u,
                    &mut rec.v,
                );
                rec.set_face_normal(r, &outward_normal);
                rec.material = self.material.clone();
                return true;
            }
            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
                get_sphere_uv(
                    &((rec.p - self.center) * self.radius.recip()),
                    &mut rec.u,
                    &mut rec.v,
                );
                rec.set_face_normal(r, &outward_normal);
                rec.material = self.material.clone();
                return true;
            }
        }

        false
    }

    fn bounding_box(&self, _t0: f64, _t1: f64, output_box: &mut crate::aabb::Aabb) -> bool {
        *output_box = crate::aabb::Aabb {
            min: self.center - Vec3::new_with(self.radius),
            max: self.center + Vec3::new_with(self.radius),
        };
        true
    }
    fn pdf_value(&self, o: &Point3, v: &Vec3) -> f64 {
        let mut rec = HitRecord::default();
        if !self.hit(&Ray::new(*o, *v, 0.0), 0.001, f64::INFINITY, &mut rec) {
            return 0.0;
        }

        let cos_theta_max =
            (1.0 - self.radius * self.radius * (self.center - *o).length_squared().recip()).sqrt();

        (TWO_PI * (1.0 - cos_theta_max)).recip()
    }
    fn random(&self, rng: &mut rand::prelude::ThreadRng, origin: &Vec3) -> Vec3 {
        let direction = self.center - *origin;
        let distance_squared = direction.length_squared();
        let uvw = crate::onb::Onb::build_from_w(&direction);
        uvw.local(&crate::pdf::random_to_sphere(
            rng,
            self.radius,
            distance_squared,
        ))
    }
}

/// Moving sphere object, used for motion blur.
#[derive(Clone, Default)]
pub struct MovingSphere {
    /// Initial center point of sphere.
    pub center0: Point3,
    /// Final center point of sphere.
    pub center1: Point3,
    /// Initial time interval of sphere.
    pub time0: f64,
    /// Final time interval of sphere.
    pub time1: f64,
    /// Radius of sphere.
    pub radius: f64,
    /// Material of sphere.
    pub material: std::sync::Arc<Material>,
}

impl MovingSphere {
    /// Create a new moving sphere.
    pub fn new(
        center0: Point3,
        center1: Point3,
        time0: f64,
        time1: f64,
        radius: f64,
        material: std::sync::Arc<Material>,
    ) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }

    /// Find the center of a sphere at a point in time.
    pub fn center(&self, time: f64) -> Point3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.origin() - self.center(r.time());
        let a = r.direction().length_squared();
        let half_b = oc.dot(&r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal: Vec3 = (rec.p - self.center(r.time())) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                rec.material = self.material.clone();
                return true;
            }
            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal: Vec3 = (rec.p - self.center(r.time())) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                rec.material = self.material.clone();
                return true;
            }
        }

        false
    }

    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut crate::aabb::Aabb) -> bool {
        let box0 = crate::aabb::Aabb {
            min: self.center(t0) - Vec3::new_with(self.radius),
            max: self.center(t0) + Vec3::new_with(self.radius),
        };
        let box1 = crate::aabb::Aabb {
            min: self.center(t1) - Vec3::new_with(self.radius),
            max: self.center(t1) + Vec3::new_with(self.radius),
        };
        *output_box = crate::aabb::Aabb::surrounding_box(&box0, &box1);
        true
    }
}
