use nalgebra::Vector3;
use rand::Rng;
use std::f64::consts::PI;
// use std::f64::min;

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

pub fn random_unit_sphere() -> Vector3<f64> {
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
    let normalized_vec = vec.normalize();
    normalized_vec - 2. * normal * normalized_vec.dot(normal)
}

pub fn refract(vec: &Vector3<f64>, normal: &Vector3<f64>, etai_over_etat: f64) -> Vector3<f64> {
    let uv = &vec.normalize();
    let cos_theta = f64::min((-uv).dot(&normal), 1.0);

    let r_out_perp: Vector3<f64> = etai_over_etat * (uv + cos_theta * normal);
    let r_out_parallel = -((1.0 - r_out_perp.magnitude_squared()).abs()).sqrt() * normal;

    r_out_perp + r_out_parallel
}

// pub fn refract(vec: &Vector3<f64>, normal: &Vector3<f64>, ni_nt: f64) -> Option<Vector3<f64>> {
//     let mut refracted = Vector3::default();

//     let uv: Vector3<f64> = vec.normalize();
//     let dt: Vector3<f64> = vec.dot(&normal);
//     let discriminant: f64 = 1. - ni_nt * ni_nt * (1. - dt * dt);

//     match discriminant {
//         discriminant if discriminant > 0. => {
//             refracted = ni_nt * (uv - normal * dt) - normal * discriminant.sqrt();
//             Some(refracted)
//         }
//         _ => None,
//     }
// }
