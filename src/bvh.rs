use std::cmp::Ordering;

use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray, list::HittableList,
};

enum BvhNode {
    Branch { left: Box<Bvh>, right: Box<Bvh> },
    Leaf(Box<dyn Hittable>),
}

pub struct Bvh {
    tree: BvhNode,
    bbox: Aabb,
}

impl Bvh {
    pub fn new(list: HittableList) -> Self {
        let len = list.objects.len();
        Self::build(list.objects, 0, len)
    }

    fn build(src_objects: Vec<Box<dyn Hittable>>, start: usize, end: usize) -> Self {
        let mut bbox = Aabb::EMPTY;
        src_objects[start..end].iter().for_each(|object| {
            bbox = Aabb::new_from_boxes(bbox, object.bounding_box().unwrap());
        });

        let axis = bbox.longest_axis();

        let comparator = match axis {
            0 => Self::box_compare_x,
            1 => Self::box_compare_y,
            _ => Self::box_compare_z,
        };

        let mut object = src_objects;

        let object_span = end - start;

        match object_span {
            1 => {
                let leaf = object.pop();
                Self {
                    tree: BvhNode::Leaf(leaf.unwrap()),
                    bbox,
                }
            }

            _ => {
                object[start..end].sort_by(comparator);
                let mid = start + object_span / 2;
                let left = Box::new(Self::build(
                    object.drain(object_span / 2..).collect(),
                    start,
                    mid,
                ));
                let right = Box::new(Self::build(object, start, mid));
                Self {
                    tree: BvhNode::Branch { left, right },
                    bbox,
                }
            }
        }
    }

    fn box_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>, axis_index: usize) -> Ordering {
        a.bounding_box()
            .unwrap()
            .axis(axis_index)
            .min
            .partial_cmp(&b.bounding_box().unwrap().axis(axis_index).min)
            .unwrap()
    }

    fn box_compare_x(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 0)
    }

    fn box_compare_y(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 1)
    }

    fn box_compare_z(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl Hittable for Bvh {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        if !self.bbox.hit(r, ray_t) {
            return None;
        }

        match &self.tree {
            BvhNode::Leaf(object) => object.hit(r, ray_t),
            BvhNode::Branch { left, right } => {
                let hit_left = left.hit(r, ray_t);
                let hit_right = right.hit(r, ray_t);

                match (hit_left, hit_right) {
                    (Some(left_hit), Some(right_hit)) => {
                        if left_hit.t < right_hit.t {
                            Some(left_hit)
                        } else {
                            Some(right_hit)
                        }
                    },

                    (Some(left_hit), None) => Some(left_hit),

                    (None, Some(right_hit)) => Some(right_hit),

                    (None, None) => None,
                }
            }
        }
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(self.bbox)
    }
}
