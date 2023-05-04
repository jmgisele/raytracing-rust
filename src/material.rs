use nalgebra::Vector3;

use crate::{
    data::Color,
    math::{near_zero, random_unit_vec, reflect},
    world::{Intersection, Ray},
};

#[derive(Copy, Clone)]
pub enum MaterialType {
    Lambertian(Lambertian),
    Metal(Metal),
}

impl MaterialType {
    pub fn scatter(&self, ray_in: &Ray, intersection: &Intersection) -> Option<(Color, Ray)> {
        match *self {
            MaterialType::Lambertian(lam) => lam.scatter(ray_in, intersection),
            MaterialType::Metal(metal) => metal.scatter(ray_in, intersection),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

#[derive(Copy, Clone)]
pub struct Metal {
    pub albedo: Color,
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, intersection: &Intersection) -> Option<(Color, Ray)>;
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

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, intersection: &Intersection) -> Option<(Color, Ray)> {
        let reflect_dir: Vector3<f64> = reflect(&ray_in.dir.normalize(), &intersection.normal);

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
