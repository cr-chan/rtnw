use crate::{color::Color, hittable::HitRecord, ray::Ray, rtweekend::random_double, vec3::Vec3, texture::Texture};

pub trait Material: Sync {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
    ) -> Option<(Ray, Color)>;

    fn emitted(&self, _u: f64, _v: f64, _p: Vec3) -> Color {
        Color::default()
    }
}

#[derive(Clone, Copy, Default)]
pub struct Lambertian<T: Texture> {
    albedo: T,
}

impl<T: Texture> Lambertian<T> {
    pub fn new(a: T) -> Self {
        Self { albedo: a }
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
    ) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new_with_time(rec.p, scatter_direction, r_in.time());
        let attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        Some((scattered, attenuation))
    }
}

#[derive(Clone, Copy)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(a: Color, f: f64) -> Self {
        Self {
            albedo: a,
            fuzz: if f < 1.0 { f } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
    ) -> Option<(Ray, Color)> {
        let reflected = Vec3::reflect(Vec3::unit_vector(r_in.direction()), rec.normal);
        let scattered = Ray::new_with_time(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere(), r_in.time());
        let attenuation = self.albedo;
        if Vec3::dot(scattered.direction(), rec.normal) > 0.0 {
            return Some((scattered, attenuation));
        }
        None
    }
}

#[derive(Clone, Copy)]
pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            ir: index_of_refraction,
        }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;

        let a = 1.0 - cosine;

        r0 + (1.0 - r0) * a.powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
    ) -> Option<(Ray, Color)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = Vec3::unit_vector(r_in.direction());

        let cos_theta = -Vec3::dot(rec.normal, unit_direction).min(1.0);
        let sin_theta = (1.0 - cos_theta.powf(2.0)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction =
            if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random_double() {
                Vec3::reflect(unit_direction, rec.normal)
            } else {
                Vec3::refract(unit_direction, rec.normal, refraction_ratio)
            };

        let scattered = Ray::new_with_time(rec.p, direction, r_in.time());

        Some((scattered, attenuation))
    }
}

#[derive(Clone, Copy)]
pub struct DiffuseLight<T: Texture> {
    emit: T,
}

impl<T: Texture> DiffuseLight<T> {
    pub fn new(a: T) -> Self {
        Self { emit: a }
    }
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
    ) -> Option<(Ray, Color)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Color {
        self.emit.value(u, v, p)
    }
}

pub struct Isotropic {
    albedo: Box<dyn Texture>,
}

impl Isotropic {
    pub fn new(a: Box<dyn Texture>) -> Self {
        Self { albedo: a }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let scattered = Ray::new_with_time(rec.p, Vec3::random_unit_vector(), r_in.time());
        let attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        Some((scattered, attenuation))
    }
}