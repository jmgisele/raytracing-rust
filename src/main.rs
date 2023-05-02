use nalgebra::Vector3;
use structs::{Color, Point, Ray};

pub mod structs;

fn main() {
    // image
    const IMAGE_WIDTH: u32 = 400;
    const ASPECT_RATIO: f64 = 16. / 9.;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    // camera
    const VIEWPORT_HEIGHT: f64 = 2.;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.;

    let origin = Point(Vector3::new(0., 0., 0.));
    let horizontal: Vector3<f64> = Vector3::new(VIEWPORT_WIDTH, 0., 0.);
    let vertical: Vector3<f64> = Vector3::new(0., VIEWPORT_HEIGHT, 0.);
    let lower_left_corner: Vector3<f64> =
        origin.0 - (horizontal / 2.) - (vertical / 2.) - Vector3::new(0., 0., FOCAL_LENGTH);

    // render
    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {j}");
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;

            let ray = Ray {
                origin: origin,
                dir: lower_left_corner + u * horizontal + v * vertical - origin,
            };

            let color = ray_color(&ray);

            write_color(color);
        }
    }
    eprintln!("Done!");
}

fn hit_sphere(center: Point, radius: f64, ray: &Ray) -> bool {
    let vec_oc = ray.origin - center;

    let a = ray.dir.magnitude_squared();
    let b = 2. * vec_oc.dot(&ray.dir);
    let c = vec_oc.magnitude_squared() - (radius * radius);
    let discriminant = b * b - 4. * a * c;

    discriminant > 0.
}

fn ray_color(ray: &Ray) -> Color {
    let white = Color(Vector3::new(1., 1., 1.));
    let red = Color(Vector3::new(1., 0., 0.));
    let blue = Color(Vector3::new(0.5, 0.7, 1.));

    if (hit_sphere(Point(Vector3::new(0., 0., -1.)), 0.5, ray)) {
        return red;
    }
    let unit_dir: Vector3<f64> = ray.dir.normalize();

    let t = 0.5 * unit_dir.y + 1.;

    (1. - t) * white + t * blue
}

fn write_color(color: Color) {
    let r = 255.999 * color.0.x;
    let g = 255.999 * color.0.y;
    let b = 255.999 * color.0.z;

    println!("{r} {g} {b} \n");
}
