use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Sphere<'a> {
    pub center: Point3,
    pub radius: f64,
    pub material: Box<dyn Material + 'a>,
}

impl<'a> Sphere<'a> {
    pub fn new(center: Point3, radius: f64, mat: impl Material + 'a) -> Sphere<'a> {
        Sphere {
            center,
            radius,
            material: Box::new(mat),
        }
    }
}

impl<'a> Hittable for Sphere<'a> {
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

        let mut rec = HitRecord::new(p, normal, t, &*self.material);
        rec.set_normal(ray);
        Some(rec)
    }
}
