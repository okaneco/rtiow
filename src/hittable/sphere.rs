use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

/// Sphere object.
#[derive(Debug, Copy, Clone, Default)]
pub struct Sphere {
    /// Center point of sphere.
    pub center: Point3,
    /// Radius of sphere.
    pub radius: f64,
    /// Material of sphere.
    pub material: Material,
}

impl Sphere {
    /// Create a new sphere.
    pub fn new(center: Point3, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
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
                rec.set_face_normal(r, &outward_normal);
                rec.material = self.material;
                return true;
            }
            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                rec.material = self.material;
                return true;
            }
        }

        false
    }
}

/// Moving sphere object, used for motion blur.
#[derive(Debug, Copy, Clone, Default)]
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
    pub material: Material,
}

impl MovingSphere {
    /// Create a new moving sphere.
    pub fn new(
        center0: Point3,
        center1: Point3,
        time0: f64,
        time1: f64,
        radius: f64,
        material: Material,
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
                rec.material = self.material;
                return true;
            }
            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal: Vec3 = (rec.p - self.center(r.time())) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                rec.material = self.material;
                return true;
            }
        }

        false
    }
}
