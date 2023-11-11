extern crate rayon;

use rayon::prelude::*;

use crate::{
    color::Color,
    hittable::Hittable,
    interval::Interval,
    ray::{Point3, Ray},
    rtweekend::{self, degrees_to_radians, random_double},
    vec3::Vec3,
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    pub background: Color,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    image_height: i32,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            image_height: 0,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            lookfrom: Point3::new(0.0, 0.0, -1.0),
            lookat: Point3::new(0.0, 0.0, 0.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            u: Vec3::default(),
            v: Vec3::default(),
            w: Vec3::default(),
            center: Point3::default(),
            pixel00_loc: Point3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            defocus_disk_u: Vec3::default(),
            defocus_disk_v: Vec3::default(),
            background: Color::default(),
        }
    }
}

impl Camera {
    pub fn render(&mut self, world: &dyn Hittable) -> Vec<(i32, i32, i32)> {
        self.initialize();
        println!("P3\n{} {}\n255", self.image_width, self.image_height);
        eprintln!("\rProcessing...");
        let collection = (0..self.image_height)
            .into_par_iter()
            .map(|j| {
                eprintln!("\rScanlines remaining: {}", self.image_height - j);
                (0..self.image_width)
                    .into_par_iter()
                    .map(|i| {
                        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                        for _ in 0..self.samples_per_pixel {
                            let r = self.get_ray(i, j);
                            pixel_color += self.ray_color(&r, self.max_depth, world);
                        }
                        let (x, y, z) = pixel_color.color(self.samples_per_pixel);
                        (x, y, z)
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        collection.into_iter().flatten().collect::<Vec<_>>()
    }

    fn ray_color(&self, r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(rec) = world.hit(r, &Interval::new(0.001, rtweekend::INFINITY)) {
            let color_from_emission = rec.mat.emitted(rec.u, rec.v, rec.p);
            if let Some((scatterd, attenuation)) = rec.mat.scatter(r, &rec) {
                let color_from_scatter = attenuation * self.ray_color(&scatterd, depth - 1, world);
                color_from_emission + color_from_scatter
            } else {
                color_from_emission
            }
        } else {
            self.background
        }
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.center = self.lookfrom;

        let theta = degrees_to_radians(self.vfov);
        let h = theta / 2.0;
        let h = h.tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        self.w = Vec3::unit_vector(self.lookfrom - self.lookat);
        self.u = Vec3::unit_vector(Vec3::cross(self.vup, self.w));
        self.v = Vec3::cross(self.w, self.u);

        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;

        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius = self.focus_dist * degrees_to_radians(self.defocus_angle / 2.0).tan();

        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_center =
            self.pixel00_loc + i as f64 * self.pixel_delta_u + j as f64 * self.pixel_delta_v;

        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };

        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random_double();

        Ray::new_with_time(ray_origin, ray_direction, ray_time)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + rtweekend::random_double();
        let py = -0.5 + rtweekend::random_double();
        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();

        self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }
}
