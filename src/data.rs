use std::ops::{Add, Mul, Sub};

use nalgebra::Vector3;

#[derive(Copy, Clone)]
pub struct Color(pub Vector3<f64>);

impl Default for Color {
    fn default() -> Self {
        Color(Vector3::new(0., 0., 0.))
    }
}

impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, mul: f64) -> Self::Output {
        Color(mul * self.0)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, color: Color) -> Self::Output {
        Color(color.0 * self)
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, color: Color) -> Self::Output {
        Color(self.0 + color.0)
    }
}

#[derive(Copy, Clone)]
pub struct Point(pub Vector3<f64>);

impl Add<Vector3<f64>> for Point {
    type Output = Vector3<f64>;
    fn add(self, vec: Vector3<f64>) -> Self::Output {
        self.0 + vec
    }
}

impl Add<Point> for Vector3<f64> {
    type Output = Vector3<f64>;
    fn add(self, point: Point) -> Self::Output {
        self + point.0
    }
}

impl Add<Point> for Point {
    type Output = Vector3<f64>;
    fn add(self, point: Point) -> Self::Output {
        self.0 + point.0
    }
}

impl Sub<Point> for Vector3<f64> {
    type Output = Vector3<f64>;
    fn sub(self, point: Point) -> Self::Output {
        self - point.0
    }
}

impl Sub<Vector3<f64>> for Point {
    type Output = Vector3<f64>;
    fn sub(self, vec: Vector3<f64>) -> Self::Output {
        self.0 - vec
    }
}

impl Sub<Point> for Point {
    type Output = Vector3<f64>;
    fn sub(self, vec: Point) -> Self::Output {
        self.0 - vec.0
    }
}
