use std::sync::Arc;

use crate::{
    bvh::BvhNode,
    camera::Camera,
    color::Color,
    hittable::HittableList,
    material::{Dielectric, Lambertian, Metal},
    quad::Quad,
    ray::Point3,
    rtweekend::{random_double, random_double_range},
    sphere::Sphere,
    texture::{CheckerTexture, ImageTexture, NoiseTexture},
    vec3::Vec3,
};

pub fn random_sphere() -> Vec<(i32, i32, i32)> {
    let mut world = HittableList::default();

    let ground_material = Lambertian::new_color(Color { e: [0.5, 0.5, 0.5] });

    world.add(Sphere::new(
        Vec3 {
            e: [0.0, -1000.0, 0.0],
        },
        1000.0,
        ground_material,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Lambertian::new_color(albedo);
                    let center2 = center + Vec3::new(0.0, random_double_range(0.0, 0.5), 0.0);
                    world.add(Sphere::new_moving(center, center2, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    let center2 = center + Vec3::new(0.0, random_double_range(0.0, 0.5), 0.0);
                    world.add(Sphere::new_moving(center, center2, 0.2, sphere_material));
                } else {
                    let sphere_material = Dielectric::new(1.5);
                    let center2 = center + Vec3::new(0.0, random_double_range(0.0, 0.5), 0.0);
                    world.add(Sphere::new_moving(center, center2, 0.2, sphere_material));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);

    world.add(Sphere::new(Point3 { e: [0.0, 1.0, 0.0] }, 1.0, material1));

    let material2 = Lambertian::new_color(Color { e: [0.4, 0.2, 0.1] });

    world.add(Sphere::new(
        Point3 {
            e: [-4.0, 1.0, 0.0],
        },
        1.0,
        material2,
    ));

    let material3 = Metal::new(Color { e: [0.7, 0.6, 0.5] }, 0.0);

    world.add(Sphere::new(Vec3 { e: [4.0, 1.0, 0.0] }, 1.0, material3));

    world = HittableList::new(Arc::new(BvhNode::new(&mut world)));

    let mut camera = Camera::default();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 1200;
    camera.samples_per_pixel = 10;
    camera.max_depth = 50;
    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(13.0, 2.0, 3.0);
    camera.lookat = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);
    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;

    let image = camera.render(&world);

    image
}

pub fn two_sphere() -> Vec<(i32, i32, i32)> {
    let mut world = HittableList::default();

    // let checker = Arc::new(CheckerTexture::new_color(0.8, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));

    world.add(Sphere::new(
        Vec3 {
            e: [0.0, -10.0, 0.0],
        },
        10.0,
        Lambertian::new(Arc::new(CheckerTexture::new_color(
            0.8,
            Color::new(0.2, 0.3, 0.1),
            Color::new(0.9, 0.9, 0.9),
        ))),
    ));

    world.add(Sphere::new(
        Vec3 {
            e: [0.0, 10.0, 0.0],
        },
        10.0,
        Lambertian::new(Arc::new(CheckerTexture::new_color(
            0.8,
            Color::new(0.2, 0.3, 0.1),
            Color::new(0.9, 0.9, 0.9),
        ))),
    ));

    let mut camera = Camera::default();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 1200;
    camera.samples_per_pixel = 10;
    camera.max_depth = 50;
    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(13.0, 2.0, 3.0);
    camera.lookat = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);
    camera.defocus_angle = 0.0;

    let image = camera.render(&world);

    image
}

pub fn make_earth() -> Vec<(i32, i32, i32)> {
    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Lambertian::new(earth_texture);
    let globe = Arc::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 2.0, earth_surface));

    let mut camera = Camera::default();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 50;
    camera.max_depth = 10;
    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(0.0, 0.0, 12.0);
    camera.lookat = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);
    camera.defocus_angle = 0.0;

    let image = camera.render(&HittableList::new(globe));

    image
}

pub fn two_perlin_spheres() -> Vec<(i32, i32, i32)> {
    let mut world = HittableList::default();

    // let pertext = Arc::new(NoiseTexture::default());

    let pertext = Arc::new(NoiseTexture::new(4.0));

    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(pertext.clone()),
    ));

    world.add(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Lambertian::new(pertext.clone()),
    ));

    let mut camera = Camera::default();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 1200;
    camera.samples_per_pixel = 50;
    camera.max_depth = 50;
    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(13.0, 2.0, 3.0);
    camera.lookat = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);
    camera.defocus_angle = 0.0;

    let image = camera.render(&world);

    image
}

pub fn quads() -> Vec<(i32, i32, i32)> {
    let mut world = HittableList::default();

    let left_red = Lambertian::new_color(Color::new(1.0, 0.2, 0.2));
    let back_green = Lambertian::new_color(Color::new(0.2, 1.0, 0.2));
    let right_blue = Lambertian::new_color(Color::new(0.2, 0.2, 1.0));
    let upper_orange = Lambertian::new_color(Color::new(1.0, 0.5, 0.0));
    let lower_teal = Lambertian::new_color(Color::new(0.2, 0.8, 0.8));

    world.add(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    ));

    world.add(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    ));

    world.add(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    ));

    world.add(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    ));

    world.add(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
    ));

    let mut camera = Camera::default();

    camera.aspect_ratio = 1.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.vfov = 80.0;
    camera.lookfrom = Point3::new(0.0, 0.0, 9.0);
    camera.lookat = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);
    camera.defocus_angle = 0.0;

    let image = camera.render(&world);

    image
    
}
