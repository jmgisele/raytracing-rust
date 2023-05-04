use nalgebra::Vector3;

use crate::structs::{Point, Ray};

pub const ASPECT_RATIO: f64 = 16. / 9.;

pub struct Camera {
    origin: Point,
    lower_left: Point,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
}

impl Default for Camera {
    fn default() -> Self {
        const VIEWPORT_HEIGHT: f64 = 2.;
        const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
        const FOCAL_LENGTH: f64 = 1.;

        let origin = Point(Vector3::new(0., 0., 0.));
        let horizontal: Vector3<f64> = Vector3::new(VIEWPORT_WIDTH, 0., 0.);
        let vertical: Vector3<f64> = Vector3::new(0., VIEWPORT_HEIGHT, 0.);
        let lower_left = Point(
            origin.0 - (horizontal / 2.) - (vertical / 2.) - Vector3::new(0., 0., FOCAL_LENGTH),
        );

        Camera {
            origin,
            lower_left,
            horizontal,
            vertical,
        }
    }
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            dir: self.lower_left + u * self.horizontal + v * self.vertical - self.origin,
        }
    }
}
