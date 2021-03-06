use crate::{aabb::*, hit_record::*, hittable::*, ray::*, vec3::*};
use rand::seq::SliceRandom;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl<'a> HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }
}

impl<'a> Hittable for HittableList {
    fn hit(&self, ray: &Ray, taemin: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            let mut temp_rec = HitRecord::default();
            if object.hit(&ray, taemin, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *hit_record = temp_rec;
            }
        }

        hit_anything
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }

        let mut temp_box = AABB::default();

        for object in &self.objects {
            if let Some(bbox) = object.bounding_box(t0, t1) {
                temp_box = temp_box.surrounding_box(bbox);
            } else {
                return None;
            }
        }

        Some(temp_box)
    }

    fn pdf_value(&self, point: &Point, vector: &Vec3) -> f32 {
        let weight = 1.0 / self.objects.len() as f32;
        let mut sum = 0.0;

        for object in &self.objects {
            sum += weight * object.pdf_value(point, vector);
        }

        sum
    }

    fn random(&self, point: &Point) -> Vec3 {
        self.objects
            .choose(&mut rand::thread_rng())
            .unwrap()
            .random(point)
    }
}
