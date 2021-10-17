mod color;
mod vec;

use color::Color;

fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    for j in (0..=ny - 1).rev() {
        for i in 0..nx {
            let c = Color::new(i as f32 / nx as f32, j as f32 / ny as f32, 0.2f32);
            let ir = (255.99 * c.r) as i32;
            let ig = (255.99 * c.g) as i32;
            let ib = (255.99 * c.b) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
