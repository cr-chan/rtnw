use crate::{ray::Point3, rtweekend::random_int_range, vec3::Vec3};

const POINT_COUNT: usize = 256;

pub struct Perlin {
    ranvec: Vec<Vec3>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Default for Perlin {
    fn default() -> Self {
        let mut ranvec: Vec<Vec3> = Vec::with_capacity(POINT_COUNT);
        for _ in 0..POINT_COUNT {
            ranvec.push(Vec3::unit_vector(Vec3::random_range(-1.0, 1.0)))
        }
        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();

        Self {
            ranvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }
}

impl Perlin {
    pub fn noise(&self, p: Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c = [[[Vec3::default(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[self.perm_x[((i + di as i32) & 255) as usize]
                        as usize
                        ^ self.perm_y[((j + dj as i32) & 255) as usize] as usize
                        ^ self.perm_z[((k + dk as i32) & 255) as usize] as usize];
                }
            }
        }

        Self::trilinear_interp(c, u, v, w)
    }

    pub fn turb(&self, p: Point3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;
        for _ in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }
        accum.abs()
    }

    fn perlin_generate_perm() -> Vec<i32> {
        let mut p: Vec<i32> = Vec::with_capacity(POINT_COUNT);

        for i in 0..POINT_COUNT {
            p.push(i.try_into().unwrap());
        }
        Self::permute(&mut p, POINT_COUNT);

        p
    }

    fn permute(p: &mut Vec<i32>, n: usize) {
        for i in (0..n - 1).rev() {
            let target = random_int_range(0, i as i32);
            p.swap(i, target as usize);
        }
    }

    fn trilinear_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u.powf(2.0) * (3.0 - 2.0 * u);
        let vv = v.powf(2.0) * (3.0 - 2.0 * v);
        let ww = w.powf(2.0) * (3.0 - 2.0 * w);

        c.iter()
            .enumerate()
            .map(|(i, array_2d)| {
                array_2d
                    .iter()
                    .enumerate()
                    .map(|(j, array_1d)| {
                        array_1d
                            .iter()
                            .enumerate()
                            .map(|(k, c_value)| {
                                let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                                (i as f64 * u + (1 - i) as f64 * (1.0 - uu))
                                    * (j as f64 * v + (1 - j) as f64 * (1.0 - vv))
                                    * (k as f64 * w + (1 - k) as f64 * (1.0 - ww))
                                    * Vec3::dot(*c_value, weight_v)
                            })
                            .sum::<f64>()
                    })
                    .sum::<f64>()
            })
            .sum()
    }
}
