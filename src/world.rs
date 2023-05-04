use nalgebra::Vector3;

use crate::{
    data::{Color, Point},
    material::{Lambertian, MaterialType, Metal},
};

pub struct Ray {
    pub origin: Point,
    pub dir: Vector3<f64>,
}

impl Default for Ray {
    fn default() -> Self {
        Ray {
            origin: Point(Vector3::new(0., 0., 0.)),
            dir: Vector3::new(0., 0., 0.),
        }
    }
}

impl Ray {
    pub fn at(&self, &t: &f64) -> Point {
        return Point(self.origin + t * self.dir);
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection>;

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
    pub material: MaterialType,
}

impl Intersection {
    pub fn set_front_face(&mut self, ray: &Ray, outward_normal: &Vector3<f64>) {
        self.front_face = ray.dir.dot(outward_normal) < 0.;
    }

    pub fn set_normal(&mut self, outward_normal: &Vector3<f64>) {
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
            material: MaterialType::Lambertian(Lambertian {
                albedo: Color::default(),
            }),
        }
    }
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub material: MaterialType,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let vec_oc = ray.origin - self.center;

        let a = ray.dir.magnitude_squared();
        let b = vec_oc.dot(&ray.dir);
        let c = vec_oc.magnitude_squared() - (self.radius * self.radius);
        let discriminant = b * b - a * c;

        if discriminant < 0. {
            return None;
        }

        let roots = [
            (-b - discriminant.sqrt()) / a,
            (-b + discriminant.sqrt()) / a,
        ];

        let mut intersection = Intersection::default();

        for root in roots {
            if Self::root_in_bounds(&root, t_min, t_max) {
                intersection.t = root;
                intersection.point = ray.at(&intersection.t);
                let outward_normal = (intersection.point - self.center) / self.radius;
                intersection.set_front_face(ray, &outward_normal); // i happen first!
                intersection.set_normal(&outward_normal); // depends on front face
                intersection.material = self.material;

                return Some(intersection);
            }
        }

        return None;
    }
}

pub struct ObjectList<T> {
    pub objects: Vec<T>,
}

impl Default for ObjectList<Sphere> {
    fn default() -> Self {
        let mut world = ObjectList {
            objects: Vec::new(),
        };

        world.objects.push(Sphere {
            center: Point(Vector3::new(0., -100.5, -1.)),
            radius: 100.,
            material: MaterialType::Lambertian(Lambertian {
                albedo: Color(Vector3::new(0.8, 0.8, 0.)),
            }),
        });

        //center
        world.objects.push(Sphere {
            center: Point(Vector3::new(0., 0., -1.)),
            radius: 0.5,
            material: MaterialType::Lambertian(Lambertian {
                albedo: Color(Vector3::new(0.7, 0.3, 0.3)),
            }),
        });

        // left
        world.objects.push(Sphere {
            center: Point(Vector3::new(-1., 0., -1.)),
            radius: 0.5,
            material: MaterialType::Metal(Metal {
                albedo: Color(Vector3::new(0.8, 0.8, 0.8)),
            }),
        });

        // right
        world.objects.push(Sphere {
            center: Point(Vector3::new(1., 0., -1.)),
            radius: 0.5,
            material: MaterialType::Metal(Metal {
                albedo: Color(Vector3::new(0.8, 0.6, 0.2)),
            }),
        });

        world
    }
}

impl Hittable for ObjectList<Sphere> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let mut hit_anything = None;
        let mut closest = t_max;

        for object in self.objects.iter() {
            if let Some(intersection) = object.hit(ray, t_min, closest) {
                hit_anything = Some(intersection);
                closest = intersection.t;
            }
        }

        hit_anything
    }
}
