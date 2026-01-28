use crate::{
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(p: Point3, normal: Vec3, t: f64, material: &'a dyn Material) -> HitRecord<'a> {
        HitRecord {
            p,
            normal,
            t,
            front_face: true,
            material,
        }
    }
    pub fn set_normal(&mut self, ray: &Ray) {
        self.front_face = Vec3::dot(self.normal, ray.direction) < 0.0;

        if !self.front_face {
            self.normal = -self.normal;
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
}
