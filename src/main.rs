#![allow(dead_code)]

mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;
mod aabb;
mod bvh;
mod texture;
mod scene;
mod perlin;
mod quad;
mod constant_medium;

use std::{io::Write,time::Instant};

use scene::*;


fn main() {
    let start_time = Instant::now();

    let a: usize = 2;

    let image = match a {
        1 => two_sphere(),
        2 => random_sphere(),
        3 => two_perlin_spheres(),
        4 => quads(),
        5 => simple_light(),
        6 => cornell_smoke(),
        7 => cornell_box(),
        _ => final_scene(),
    };

    for p in 0..image.len(){
        let stdout = std::io::stdout();
        let mut lock = stdout.lock();

        writeln!(lock, "{} {} {}", image[p].0, image[p].1, image[p].2).unwrap();
    }

    eprintln!("Done!");

    let end_time = Instant::now();

    let elapsed_time = end_time.duration_since(start_time);

    eprintln!("Time: {:?}", elapsed_time);
}
