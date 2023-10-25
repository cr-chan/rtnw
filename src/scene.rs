use std::sync::Arc;


use crate::{
    bvh::BvhNode,
    camera::Camera,
    color::Color,
    hittable::HittableList,
    material::{Dielectric, Lambertian, Metal},
    ray::Point3,
    rtweekend::{random_double, random_double_range},
    sphere::Sphere,
    vec3::Vec3, texture::{CheckerTexture, ImageTexture},
};

pub fn random_sphere() -> Vec<(i32, i32, i32)>{
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
        Lambertian::new(Arc::new(CheckerTexture::new_color(0.8, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)))),
    ));

    world.add(Sphere::new(
        Vec3 {
            e: [0.0, 10.0, 0.0],
        },
        10.0,
        Lambertian::new(Arc::new(CheckerTexture::new_color(0.8, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)))),
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