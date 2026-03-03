use std::path;

use image::{ConvertColorOptions, ImageReader, Pixel, Rgb, Rgba, RgbaImage};
use rayon::result;
use scouloring_rs::{color::Color, dithering_methods::*, palette::load_all_palettes};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let palettes = load_all_palettes("./palettes").unwrap();
    let filename = path::Path::new("astronaut.png");
    let mut img = ImageReader::open(path::Path::new("./").join(filename))?.decode()?;
    // img.convert_color_space(
    //     image::metadata::Cicp::SRGB_LINEAR,
    //     ConvertColorOptions::default(),
    //     image::ColorType::Rgba8,
    // );
    let palette = &palettes["websafe"];

    let mut results = Vec::new();

    results.push(no_dither(&img.to_rgba8(), &palette.colors));
    results.push(noise(&img.to_rgba8(), &palette.colors, 127, true));
    results.push(error_diffusion(&img.to_rgba8(), &palette.colors));

    for (id, res) in results.iter().enumerate() {
        let _ = res.save(format!(
            "./_{}{}.png",
            filename.file_stem().unwrap().to_str().unwrap(),
            id,
        ));
    }

    Ok(())
}
