mod cellular_automata;
pub mod utils;

use cellular_automata::{
    BorderHandling,
    RowStartPosition,
};
use image::DynamicImage;

fn main() {
    println!("Creating a system with rule 110 and running for 100 generations...");

    let mut cas =
        cellular_automata::ElementaryCellularAutomata::new(110, 1600, RowStartPosition::Right, BorderHandling::Dead);

    (0..2560).for_each(|_| cas.update());

    let image_buffer = cas.as_image_buffer();
    let mut image = DynamicImage::ImageRgb8(image_buffer);

    image = image.rotate270();
    image = image.flipv();

    image.save("ca.png").expect("Failed to save the image.");
}
