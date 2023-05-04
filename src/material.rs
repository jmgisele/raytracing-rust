use nalgebra::Vector3;

use crate::{
    math::{near_zero, random_unit_vec, reflect},
    structs::{Color, Intersection, Ray},
};

#[derive(Copy, Clone)]
pub enum MaterialType {
    Lambertian(Lambertian),
    Metal(Metal),
}

impl MaterialType {
    pub fn scatter(
        &self,
        ray_in: &Ray,
        intersection: &Intersection,
        attenuation: &mut Color,
        ray_scattered: &mut Ray,
    ) -> bool {
        match *self {
            MaterialType::Lambertian(lam) => {
                lam.scatter(ray_in, intersection, attenuation, ray_scattered)
            }
            MaterialType::Metal(metal) => {
                metal.scatter(ray_in, intersection, attenuation, ray_scattered)
            }
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
    fn scatter(
        &self,
        ray_in: &Ray,
        intersection: &Intersection,
        attenuation: &mut Color,
        ray_scattered: &mut Ray,
    ) -> bool;
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray_in: &Ray,
        intersection: &Intersection,
        attenuation: &mut Color,
        ray_scattered: &mut Ray,
    ) -> bool {
        let mut scatter_dir: Vector3<f64> = intersection.normal + random_unit_vec();

        if near_zero(&scatter_dir) {
            scatter_dir = intersection.normal;
        }

        *ray_scattered = Ray {
            origin: intersection.point,
            dir: scatter_dir,
        };
        *attenuation = self.albedo;

        true
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        intersection: &Intersection,
        attenuation: &mut Color,
        ray_scattered: &mut Ray,
    ) -> bool {
        let reflect_dir: Vector3<f64> = reflect(&ray_in.dir.normalize(), &intersection.normal);

        *ray_scattered = Ray {
            origin: intersection.point,
            dir: reflect_dir,
        };
        *attenuation = self.albedo;

        ray_scattered.dir.dot(&intersection.normal) > 0.
    }
}
