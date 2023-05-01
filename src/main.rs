fn main() {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    // Render
    println!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_WIDTH).rev() {
        for i in 0..IMAGE_HEIGHT {
            let r: f32 = i as f32 / IMAGE_WIDTH as f32;
            let g: f32 = j as f32 / IMAGE_HEIGHT as f32;
            let b: f32 = 0.25;

            let i_r: i32 = (255.999 * r) as i32;
            let i_g: i32 = (255.999 * g) as i32;
            let i_b: i32 = (255.999 * b) as i32;

            println!("{} {} {} \n", i_r, i_g, i_b);
        }
    }
}
