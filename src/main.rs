use nalgebra::Vector3;

fn main() {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    // Render
    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");

    for j in (0..IMAGE_WIDTH).rev() {
        eprintln!("Scanlines remaining: {j}");
        for i in 0..IMAGE_HEIGHT {
            let color = Color(Vector3::new(
                i as f64 / IMAGE_WIDTH as f64,
                j as f64 / IMAGE_HEIGHT as f64,
                0.25,
            ));

            write_color(color);
        }
    }
    eprintln!("Done!");
}

struct Color(Vector3<f64>);
struct Point(Vector3<f64>);

fn write_color(color: Color) {
    let r = 255.999 * color.0.x;
    let g = 255.999 * color.0.y;
    let b = 255.999 * color.0.z;

    println!("{r} {g} {b} \n");
}
