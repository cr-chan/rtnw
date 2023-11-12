#![allow(dead_code)]

use std::{io::Write, time::Instant};

mod aabb;
mod bvh;
mod camera;
mod color;
mod constant_medium;
mod hittable;
mod interval;
mod list;
mod material;
mod perlin;
mod quads;
mod ray;
mod rtimage;
mod rtweekend;
mod scene;
mod sphere;
mod texture;
mod vec3;

fn main() {
    let start_time = Instant::now();

    let image = scene::final_scene();

    for p in 0..image.len() {
        let stdout = std::io::stdout();
        let mut lock = stdout.lock();

        writeln!(lock, "{} {} {}", image[p].0, image[p].1, image[p].2).unwrap();
    }

    eprintln!("Done!");

    let end_time = Instant::now();

    let elapsed_time = end_time.duration_since(start_time);

    eprintln!("Time: {:?}", elapsed_time);
}
