use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - ray.origin;

        let a = Vec3::length_squared(ray.direction);
        let h = Vec3::dot(ray.direction, oc);
        let c = Vec3::length_squared(oc) - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if root <= ray_t.min || ray_t.max <= root {
            root = (h + sqrtd) / a;
            if !ray_t.surronds(root) {
                return None;
            }
        }

        let t = root;
        let p = ray.at(t);
        let normal = (p - self.center) / self.radius;

        let mut rec = HitRecord::new(p, normal, t);
        rec.set_normal(ray);
        Some(rec)
    }
}
