use std::sync::Arc;

use crate::{
    bvh::BvhNode,
    camera::Camera,
    color::Color,
    constant_medium::ConstantMedium,
    hittable::{HittableList, RotateY, Translate},
    material::{Dielectric, DiffuseLight, Isotropic, Lambertian, Metal},
    quad::Quad,
    ray::Point3,
    rtweekend::{random_double, random_double_range},
    sphere::Sphere,
    texture::{CheckerTexture, NoiseTexture},
    vec3::Vec3,
};

pub fn random_sphere() -> Vec<(i32, i32, i32)> {
    let mut world = HittableList::default();
    let mut world1 = HittableList::default();

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

    world1.add(Sphere::new(Point3 { e: [0.0, 1.0, 0.0] }, 1.0, material1));

    let material2 = Lambertian::new_color(Color { e: [0.4, 0.2, 0.1] });

    world1.add(Sphere::new(
        Point3 {
            e: [-4.0, 1.0, 0.0],
        },
        1.0,
        material2,
    ));

    let material3 = Metal::new(Color { e: [0.7, 0.6, 0.5] }, 0.0);

    world1.add(Sphere::new(Vec3 { e: [4.0, 1.0, 0.0] }, 1.0, material3));

    world = HittableList::new(Arc::new(BvhNode::new(&mut world1)));

    let mut camera = Camera::default();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 1200;
    camera.samples_per_pixel = 10;
    camera.max_depth = 50;
    camera.background = Color::new(0.7, 0.8, 1.0);
    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(13.0, 2.0, 3.0);
    camera.lookat = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);
    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;

    camera.render(&world)
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
    camera.background = Color::new(0.7, 0.8, 1.0);
    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(13.0, 2.0, 3.0);
    camera.lookat = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);
    camera.defocus_angle = 0.0;

    camera.render(&world)
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
    camera.background = Color::new(0.7, 0.8, 1.0);
    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(13.0, 2.0, 3.0);
    camera.lookat = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);
    camera.defocus_angle = 0.0;

    

    camera.render(&world)
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
    camera.background = Color::new(0.7, 0.8, 1.0);
    camera.vfov = 80.0;
    camera.lookfrom = Point3::new(0.0, 0.0, 9.0);
    camera.lookat = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);
    camera.defocus_angle = 0.0;

    

    camera.render(&world)
}

pub fn simple_light() -> Vec<(i32, i32, i32)> {
    let mut world = HittableList::default();

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

    let difflight = DiffuseLight::new_color(Color::new(4.0, 4.0, 4.0));

    world.add(Quad::new(
        Point3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        difflight.clone(),
    ));

    world.add(Sphere::new(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        difflight.clone(),
    ));

    let mut camera = Camera::default();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 1200;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.background = Color::new(0.0, 0.0, 0.0);
    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(26.0, 3.0, 6.0);
    camera.lookat = Point3::new(0.0, 2.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);
    camera.defocus_angle = 0.0;

    

    camera.render(&world)
}

pub fn cornell_box() -> Vec<(i32, i32, i32)> {
    let mut world = HittableList::default();

    let red = Lambertian::new_color(Color::new(0.65, 0.05, 0.05));
    let white = Lambertian::new_color(Color::new(0.73, 0.73, 0.73));
    let green = Lambertian::new_color(Color::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new_color(Color::new(7.0, 7.0, 7.0));

    let box1 = Arc::new(Quad::boxes(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Translate::new(box1, Vec3::new(265.0, 0.0, 295.0));

    world.add(box1);

    let box2 = Arc::new(Quad::boxes(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    ));
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Translate::new(box2, Vec3::new(130.0, 0.0, 65.0));

    world.add(box2);

    world.add(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    ));

    world.add(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    ));

    world.add(Quad::new(
        Point3::new(113.0, 554.0, 127.0),
        Vec3::new(330.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 305.0),
        light,
    ));

    world.add(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    ));

    world.add(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    ));

    world.add(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white,
    ));

    let mut camera = Camera::default();

    camera.aspect_ratio = 1.0;
    camera.image_width = 600;
    camera.samples_per_pixel = 200;
    camera.max_depth = 50;
    camera.background = Color::new(0.0, 0.0, 0.0);
    camera.vfov = 40.0;
    camera.lookfrom = Point3::new(278.0, 278.0, -800.0);
    camera.lookat = Point3::new(278.0, 278.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);
    camera.defocus_angle = 0.0;

    

    camera.render(&world)
}

pub fn cornell_smoke() -> Vec<(i32, i32, i32)> {
    let mut world = HittableList::default();

    let red = Lambertian::new_color(Color::new(0.65, 0.05, 0.05));
    let white = Lambertian::new_color(Color::new(0.73, 0.73, 0.73));
    let green = Lambertian::new_color(Color::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new_color(Color::new(7.0, 7.0, 7.0));

    let box1 = Arc::new(Quad::boxes(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Translate::new(box1, Vec3::new(265.0, 0.0, 295.0));

    let box1 = Arc::new(box1);

    let box2 = Arc::new(Quad::boxes(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    ));
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Translate::new(box2, Vec3::new(130.0, 0.0, 65.0));

    let box2 = Arc::new(box2);

    world.add(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    ));

    world.add(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    ));

    world.add(Quad::new(
        Point3::new(113.0, 554.0, 127.0),
        Vec3::new(330.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 305.0),
        light,
    ));

    world.add(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    ));

    world.add(Quad::new(
        Point3::new(0.0, 555.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    ));

    world.add(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white,
    ));

    world.add(ConstantMedium::new(
        box1,
        0.01,
        Isotropic::new(Color::new(0.0, 0.0, 0.0)),
    ));

    world.add(ConstantMedium::new(
        box2,
        0.01,
        Isotropic::new(Color::new(1.0, 1.0, 1.0)),
    ));

    let mut camera = Camera::default();

    camera.aspect_ratio = 1.0;
    camera.image_width = 600;
    camera.samples_per_pixel = 200;
    camera.max_depth = 50;
    camera.background = Color::new(0.0, 0.0, 0.0);
    camera.vfov = 40.0;
    camera.lookfrom = Point3::new(278.0, 278.0, -800.0);
    camera.lookat = Point3::new(278.0, 278.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);
    camera.defocus_angle = 0.0;

    

    camera.render(&world)
}

pub fn final_scene() -> Vec<(i32, i32, i32)> {
    let mut boxes1 = HittableList::default();

    let ground = Lambertian::new_color(Color::new(0.48, 0.83, 0.53));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Quad::boxes(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            ));
        }
    }

    let mut world = HittableList::default();

    world.add(BvhNode::new(&mut boxes1));

    let light = DiffuseLight::new_color(Color::new(9.0, 9.0, 9.0));

    world.add(Quad::new(
        Point3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 305.0),
        light,
    ));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);

    let sphere_material = Lambertian::new_color(Color::new(0.7, 0.3, 0.1));

    world.add(Sphere::new_moving(center1, center2, 50.0, sphere_material));

    world.add(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        70.0,
        Dielectric::new(1.5),
    ));

    world.add(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Metal::new(Color::new(0.8, 0.8, 0.9), 1.0),
    ));

    let mut boundary = Sphere::new(Point3::new(360.0, 150.0, 145.0), 70.0, Dielectric::new(1.5));

    world.add(boundary.clone());

    world.add(ConstantMedium::new(
        Arc::new(boundary),
        0.2,
        Isotropic::new(Color::new(0.2, 0.4, 0.9)),
    ));

    boundary = Sphere::new(Point3::new(0.0, 0.0, 0.0), 5000.0, Dielectric::new(1.5));

    world.add(ConstantMedium::new(
        Arc::new(boundary),
        0.0001,
        Isotropic::new(Color::new(1.0, 1.0, 1.0)),
    ));

    let pertext = NoiseTexture::new(0.1);

    world.add(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Lambertian::new(Arc::new(pertext)),
    ));

    let mut boxes2 = HittableList::default();
    let white = Lambertian::new_color(Color::new(0.73, 0.73, 0.73));

    let ns = 1000;

    for _ in 0..ns {
        boxes2.add(Sphere::new(
            Point3::random_range(0.0, 165.0),
            10.0,
            white.clone(),
        ));
    }

    world.add(Translate::new(
        Arc::new(RotateY::new(Arc::new(BvhNode::new(&mut boxes2)), 15.0)),
        Vec3::new(450.0, 200.0, 400.0),
    ));

    let mut camera = Camera::default();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 1200;
    camera.samples_per_pixel = 100;
    camera.max_depth = 40;
    camera.background = Color::new(0.0, 0.0, 0.0);
    camera.vfov = 40.0;
    camera.lookfrom = Point3::new(478.0, 278.0, -600.0);
    camera.lookat = Point3::new(278.0, 278.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);
    camera.defocus_angle = 0.0;

    

    camera.render(&world)
}
