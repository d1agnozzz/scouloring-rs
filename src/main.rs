use std::path;

use image::ImageReader;
use scouloring_rs::{
    color::ColorSpace,
    dithering_methods::*,
    palette::{load_all_palettes, FastPalette},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let palettes = load_all_palettes("./palettes").unwrap();
    let filename = path::Path::new("skull.png");
    let img = ImageReader::open(path::Path::new("./").join(filename))?.decode()?;
    // img.convert_color_space(
    //     image::metadata::Cicp::SRGB_LINEAR,
    //     ConvertColorOptions::default(),
    //     image::ColorType::Rgba8,
    // );
    let palette = &palettes["websafe"];
    let palette = &FastPalette::from(palette);
    let color_space = ColorSpace::RGB;

    let mut results = Vec::new();

    let start = std::time::Instant::now();
    results.push(no_dither_par(&img.to_rgba8(), palette, color_space, 4096));
    let duration = start.elapsed();
    println!("par: {duration:?}");
    let start = std::time::Instant::now();
    results.push(noise_par(
        &img.to_rgba8(),
        palette,
        32,
        true,
        color_space,
        4096,
    ));
    results.push(noise_par(
        &img.to_rgba8(),
        palette,
        32,
        false,
        color_space,
        4096,
    ));
    let duration = start.elapsed();
    println!("par: {duration:?}");
    let start = std::time::Instant::now();
    results.push(error_diffusion(&img.to_rgba8(), palette, color_space));
    let duration = start.elapsed();
    println!("par: {duration:?}");

    // results.push(noise(&img.to_rgba8(), &palette.colors, 127, true));
    // results.push(error_diffusion(&img.to_rgba8(), &palette.colors, false));
    // results.push(error_diffusion(&img.to_rgba8(), &palette.colors, true));

    for (id, res) in results.iter().enumerate() {
        let _ = res.save(format!(
            "./_{}{}.png",
            filename.file_stem().unwrap().to_str().unwrap(),
            id,
        ));
    }

    Ok(())
}
