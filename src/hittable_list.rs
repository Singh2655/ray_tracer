use crate::{hittable::Hittable, interval::Interval};

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.objects.push(Box::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &crate::ray::Ray, ray_t: Interval) -> Option<crate::hittable::HitRecord> {
        let mut rec = None;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            if let Some(hrec) = object.hit(ray, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = hrec.t;
                rec = Some(hrec);
            }
        }
        rec
    }
}
