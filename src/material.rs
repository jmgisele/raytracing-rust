use nalgebra::Vector3;

use crate::{
    data::Color,
    math::{near_zero, rand, random_unit_sphere, random_unit_vec, reflect, refract},
    world::{Intersection, Ray},
    WHITE,
};

#[derive(Copy, Clone)]
pub enum MaterialType {
    Lambertian(Lambertian),
    Metal(Metal),
    FuzzyMetal(FuzzyMetal),
    Dielectric(Dielectric),
}

impl MaterialType {
    pub fn scatter(&self, ray_in: &Ray, intersection: &Intersection) -> Option<(Color, Ray)> {
        match *self {
            MaterialType::Lambertian(lam) => lam.scatter(ray_in, intersection),
            MaterialType::Metal(metal) => metal.scatter(ray_in, intersection),
            MaterialType::FuzzyMetal(metal) => metal.scatter(ray_in, intersection),
            MaterialType::Dielectric(metal) => metal.scatter(ray_in, intersection),
        }
    }
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, intersection: &Intersection) -> Option<(Color, Ray)>;
}

#[derive(Copy, Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, intersection: &Intersection) -> Option<(Color, Ray)> {
        let mut scatter_dir: Vector3<f64> = intersection.normal + random_unit_vec();

        if near_zero(&scatter_dir) {
            scatter_dir = intersection.normal;
        }

        let attenuation = self.albedo;
        let ray_scattered = Ray {
            origin: intersection.point,
            dir: scatter_dir,
        };

        Some((attenuation, ray_scattered))
    }
}

#[derive(Copy, Clone)]
pub struct Metal {
    pub albedo: Color,
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, intersection: &Intersection) -> Option<(Color, Ray)> {
        let reflect_dir: Vector3<f64> = reflect(&ray_in.dir, &intersection.normal);

        let attenuation = self.albedo;
        let ray_scattered = Ray {
            origin: intersection.point,
            dir: reflect_dir,
        };

        let dir = ray_scattered.dir.dot(&intersection.normal);

        match dir > 0. {
            true => Some((attenuation, ray_scattered)),
            false => None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct FuzzyMetal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for FuzzyMetal {
    fn scatter(&self, ray_in: &Ray, intersection: &Intersection) -> Option<(Color, Ray)> {
        let reflect_dir: Vector3<f64> = reflect(&ray_in.dir, &intersection.normal);

        let attenuation = self.albedo;
        let ray_scattered = Ray {
            origin: intersection.point,
            dir: reflect_dir + self.fuzz * random_unit_sphere(),
        };

        let dir = ray_scattered.dir.dot(&intersection.normal);

        match dir > 0. {
            true => Some((attenuation, ray_scattered)),
            false => None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Dielectric {
    pub ref_index: f64,
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, intersection: &Intersection) -> Option<(Color, Ray)> {
        let attenuation = WHITE;
        let refraction_ratio = match intersection.front_face {
            true => 1.0 / self.ref_index,
            false => self.ref_index,
        };

        let cos_theta = f64::min((-ray_in.dir.normalize()).dot(&intersection.normal), 1.0);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let reflects: bool = refraction_ratio * sin_theta > 1.
            || Dielectric::reflectance(cos_theta, refraction_ratio) > rand();
        let direction = match reflects {
            true => reflect(&ray_in.dir, &intersection.normal),
            false => refract(&ray_in.dir, &intersection.normal, refraction_ratio),
        };

        let scattered_ray = Ray {
            origin: intersection.point,
            dir: direction,
        };

        Some((attenuation, scattered_ray))
    }
}

impl Dielectric {
    fn reflectance(cos: f64, reflection_index: f64) -> f64 {
        let mut r0 = (1. - reflection_index) / (1. + reflection_index);
        r0 = r0 * r0;
        r0 + (1. - r0) * (1. - cos).powf(5.)
    }
}
