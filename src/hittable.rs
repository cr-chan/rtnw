use crate::{aabb::Aabb, interval::*, material::Material, ray::*, vec3::Vec3, rtweekend::{degrees_to_radians, INFINITY}};

pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: &'a dyn Material,
    pub u: f64,
    pub v: f64,
}

impl HitRecord<'_> {
    pub fn set_face_normal(ray: &Ray, outward_normal: Vec3) -> (Vec3, bool) {
        let front_face = Vec3::dot(ray.direction(), outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        (normal, front_face)
    }
}

pub trait Hittable: Sync {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> Option<Aabb>;
}

pub struct Translate {
    object: Box<dyn Hittable>,
    offset: Vec3,
    bbox: Aabb,
}

impl Translate {
    pub fn new(p: Box<dyn Hittable>, displacement: Vec3) -> Self {
        let bbox = p.bounding_box();
        
        Self {
            object: p,
            offset: displacement,
            bbox: bbox.unwrap() + displacement,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let offset_r = Ray::new_with_time(r.origin() - self.offset, r.direction(), r.time());

        if let Some(mut rec) = self.object.hit(&offset_r, ray_t) {
            rec.p += self.offset;
            Some(HitRecord {
                p: rec.p,
                normal: rec.normal,
                t: rec.t,
                u: rec.u,
                v: rec.v,
                front_face: rec.front_face,
                mat: rec.mat,
            })
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(self.bbox)
    }
}

pub struct RotateY {
    object: Box<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl RotateY {
    pub fn new(p: Box<dyn Hittable>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = p.bounding_box();

        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(-INFINITY, -INFINITY, -INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.unwrap().x.max + (1.0 - j as f64) * bbox.unwrap().x.min;
                    let y = j as f64 * bbox.unwrap().y.max + (1.0 - j as f64) * bbox.unwrap().y.min;
                    let z = k as f64 * bbox.unwrap().z.max + (1.0 - k as f64) * bbox.unwrap().z.min;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        Self {
            object: p,
            sin_theta,
            cos_theta,
            bbox: Aabb::new_from_points(min, max),
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut origin = r.origin();
        let mut direction = r.direction();

        origin[0] = self.cos_theta * r.origin()[0] - self.sin_theta * r.origin()[2];
        origin[2] = self.sin_theta * r.origin()[0] + self.cos_theta * r.origin()[2];

        direction[0] = self.cos_theta * r.direction()[0] - self.sin_theta * r.direction()[2];
        direction[2] = self.sin_theta * r.direction()[0] + self.cos_theta * r.direction()[2];

        let rotate_r = Ray::new_with_time(origin, direction, r.time());

        if let Some(mut rec) = self.object.hit(&rotate_r, ray_t) {
            rec.p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
            rec.p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

            rec.normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
            rec.normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

            Some(HitRecord {
                p: rec.p,
                normal: rec.normal,
                t: rec.t,
                u: rec.u,
                v: rec.v,
                front_face: rec.front_face,
                mat: rec.mat,
            })
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Option<Aabb>{
        Some(self.bbox)
    }
}



