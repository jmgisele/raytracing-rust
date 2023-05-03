use nalgebra::Vector3;

use crate::structs::{Point, Ray};

pub const ASPECT_RATIO: f64 = 16. / 9.;

pub struct Camera {
    aspect_ratio: f64,
    viewport_height: f64,
    viewport_width: f64,
    focal_length: f64,
    origin: Point,
    lower_left: Point,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio: f64 = ASPECT_RATIO;

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
            aspect_ratio,
            viewport_height: VIEWPORT_HEIGHT,
            viewport_width: VIEWPORT_WIDTH,
            focal_length: FOCAL_LENGTH,
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
