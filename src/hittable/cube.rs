use crate::{
    hittable::{rectangle::*, *},
    hittable_list::*,
    material::*,
};

pub struct Cube {
    box_min: Point,
    box_max: Point,
    sides: HittableList,
}

impl Cube {
    pub fn new(box_min: Point, box_max: Point, material: Material) -> Self {
        let mut sides = HittableList::new();

        sides.add(Box::new(XYRect {
            x0: box_min.x,
            x1: box_max.x,
            y0: box_min.y,
            y1: box_max.y,
            k: box_min.z,
            material: material.clone(),
        }));
        sides.add(Box::new(XYRect {
            x0: box_min.x,
            x1: box_max.x,
            y0: box_min.y,
            y1: box_max.y,
            k: box_max.z,
            material: material.clone(),
        }));

        sides.add(Box::new(XZRect {
            x0: box_min.x,
            x1: box_max.x,
            z0: box_min.z,
            z1: box_max.z,
            k: box_min.y,
            material: material.clone(),
        }));
        sides.add(Box::new(XZRect {
            x0: box_min.x,
            x1: box_max.x,
            z0: box_min.z,
            z1: box_max.z,
            k: box_max.y,
            material: material.clone(),
        }));

        sides.add(Box::new(YZRect {
            y0: box_min.y,
            y1: box_max.y,
            z0: box_min.z,
            z1: box_max.z,
            k: box_min.x,
            material: material.clone(),
        }));
        sides.add(Box::new(YZRect {
            y0: box_min.y,
            y1: box_max.y,
            z0: box_min.z,
            z1: box_max.z,
            k: box_max.x,
            material,
        }));

        Self {
            box_min,
            box_max,
            sides,
        }
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, taemin: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        self.sides.hit(ray, taemin, t_max, hit_record)
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB {
            min: self.box_min,
            max: self.box_max,
        })
    }
}
