use nalgebra::Vector3;
use std::ops::{Add, Mul, Sub};

pub struct Color(pub Vector3<f64>);

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

pub struct Ray {
    pub origin: Point,
    pub dir: Vector3<f64>,
}

impl Ray {
    pub fn at(&self, &t: &f64) -> Point {
        return Point(self.origin + t * self.dir);
    }
}

pub trait Hittable {
    fn is_hit(&self, ray: &Ray, t_min: f64, t_max: f64, intersection: &mut Intersection) -> bool;

    fn root_in_bounds(root: &f64, t_min: f64, t_max: f64) -> bool {
        !(*root < t_min || *root > t_max)
    }
}

#[derive(Copy, Clone)]
pub struct Intersection {
    pub point: Point,
    pub normal: Vector3<f64>,
    pub t: f64,
    pub front_face: bool,
}

impl Intersection {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector3<f64>) {
        self.front_face = ray.dir.dot(outward_normal) < 0.;
        self.normal = match self.front_face {
            true => *outward_normal,
            false => -*outward_normal,
        };
    }
}

impl Default for Intersection {
    fn default() -> Self {
        Intersection {
            point: Point(Vector3::zeros()),
            normal: Vector3::zeros(),
            t: 0.,
            front_face: false,
        }
    }
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn is_hit(&self, ray: &Ray, t_min: f64, t_max: f64, intersection: &mut Intersection) -> bool {
        let vec_oc = ray.origin - self.center;

        let a = ray.dir.magnitude_squared();
        let b = vec_oc.dot(&ray.dir);
        let c = vec_oc.magnitude_squared() - (self.radius * self.radius);
        let discriminant = b * b - a * c;

        if discriminant < 0. {
            return false;
        }
        let roots = [
            (-b - discriminant.sqrt()) / a,
            (-b + discriminant.sqrt()) / a,
        ];

        for root in roots {
            if Self::root_in_bounds(&root, t_min, t_max) {
                intersection.t = root;
                intersection.point = ray.at(&intersection.t);
                let outward_normal = (intersection.point - self.center) / self.radius;
                intersection.set_face_normal(ray, &outward_normal);

                return true;
            }
        }

        return false;
    }
}

pub struct ObjectList<T> {
    pub objects: Vec<T>,
}

impl ObjectList<Sphere> {
    pub fn get_hits(
        &self,
        ray: &Ray,
        t_min: f64,
        t_max: f64,
        intersection: &mut Intersection,
    ) -> bool {
        let mut temp_intersect = Intersection::default();
        let mut hit_anything = false;
        let mut closest = t_max;

        for object in self.objects.iter() {
            if object.is_hit(ray, t_min, closest, &mut temp_intersect) {
                hit_anything = true;
                closest = temp_intersect.t;
                *intersection = temp_intersect;
            }
        }

        hit_anything
    }
}
