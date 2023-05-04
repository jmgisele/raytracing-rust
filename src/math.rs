use nalgebra::Vector3;
use rand::Rng;
use std::f64::consts::PI;

pub fn deg_to_rad(deg: f64) -> f64 {
    deg * PI / 180.
}

pub fn random_range(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}

pub fn rand() -> f64 {
    random_range(0., 1.)
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    match x {
        x if x < min => min,
        x if x > max => max,
        _ => x,
    }
}

pub fn random_vec() -> Vector3<f64> {
    Vector3::new(rand(), rand(), rand())
}

pub fn random_range_vec(min: f64, max: f64) -> Vector3<f64> {
    Vector3::new(
        random_range(min, max),
        random_range(min, max),
        random_range(min, max),
    )
}

fn random_unit_sphere() -> Vector3<f64> {
    loop {
        let p: Vector3<f64> = random_range_vec(-1., 1.);
        if p.magnitude_squared() >= 1. {
            continue;
        } else {
            return p;
        }
    }
}

pub fn random_unit_vec() -> Vector3<f64> {
    random_unit_sphere().normalize()
}

pub fn near_zero(vec: &Vector3<f64>) -> bool {
    let small = 1e-8;
    vec.x < small && vec.y < small && vec.z < small
}

pub fn reflect(vec: &Vector3<f64>, normal: &Vector3<f64>) -> Vector3<f64> {
    vec - 2. * normal * vec.dot(normal)
}
