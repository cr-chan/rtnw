#![allow(dead_code)]

use std::{io::Write,time::Instant};

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
mod list;
mod scene;
mod rtimage;
mod perlin;
mod quads;
mod constant_medium;

fn main() {
    let start_time = Instant::now();
    
    let image = scene::earth();

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
