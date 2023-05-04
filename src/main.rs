use std::f64::INFINITY;

use math::random_unit_vec;
use nalgebra::Vector3;
use structs::{Color, Intersection, ObjectList, Point, Ray, Sphere};

use crate::{
    camera::{Camera, ASPECT_RATIO},
    math::{clamp, rand},
};

pub mod camera;
pub mod math;
pub mod structs;

fn main() {
    // image
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_RAY_DEPTH: i32 = 50;
    // camera
    let camera = Camera::default();

    // world

    let mut world = ObjectList {
        objects: Vec::new(),
    };
    world.objects.push(Sphere {
        center: Point(Vector3::new(0., 0., -1.)),
        radius: 0.5,
    });

    world.objects.push(Sphere {
        center: Point(Vector3::new(0., -100.5, -1.)),
        radius: 100.,
    });

    // render
    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {j}");
        for i in 0..IMAGE_WIDTH {
            let mut color = Color(Vector3::new(0., 0., 0.));
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rand()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rand()) / (IMAGE_HEIGHT - 1) as f64;

                let ray = camera.get_ray(u, v);

                color.0 += ray_color(&ray, &world, MAX_RAY_DEPTH).0;
            }
            write_color(color, SAMPLES_PER_PIXEL);
        }
    }
    eprintln!("Done!");
}

fn ray_color(ray: &Ray, world: &ObjectList<Sphere>, depth: i32) -> Color {
    let white = Color(Vector3::new(1., 1., 1.));
    let black = Color(Vector3::new(0., 0., 0.));
    let blue = Color(Vector3::new(0.5, 0.7, 1.));

    if depth <= 0 {
        return black;
    };

    let mut intersection = Intersection::default();

    if world.get_hits(ray, 0.001, INFINITY, &mut intersection) {
        let target: Vector3<f64> = intersection.point + intersection.normal + random_unit_vec();
        let refracted_ray: Ray = Ray {
            origin: intersection.point,
            dir: target - intersection.point,
        };

        return 0.5 * ray_color(&refracted_ray, world, depth - 1);
    }

    let unit_dir: Vector3<f64> = ray.dir.normalize();
    let t = 0.5 * (unit_dir.y + 1.);
    (1. - t) * white + t * blue
}

fn write_color(color: Color, samples_per_pix: i32) {
    let mut r = color.0.x;
    let mut g = color.0.y;
    let mut b = color.0.z;

    let scale = 1. / samples_per_pix as f64;
    r = 256. * clamp((r * scale).sqrt(), 0., 0.999);
    g = 256. * clamp((g * scale).sqrt(), 0., 0.999);
    b = 256. * clamp((b * scale).sqrt(), 0., 0.999);

    println!("{r} {g} {b} \n");
}
