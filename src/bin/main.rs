use std::path;

use image::{ConvertColorOptions, ImageReader};
use scouloring_rs::{
    color::ColorSpace,
    dithering::ordered::{
        pattern_dithering::{self, pattern_par},
        theshold_matrix::ThresholdMatrix,
        threshold_pattern::ThresholdPattern,
    },
    dithering_methods::*,
    palette::{load_all_palettes, FastPalette, Palette},
    BayerMatrix,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let palettes = load_all_palettes("./palettes").unwrap();
    let filename = path::Path::new("test2.png");
    let mut img = ImageReader::open(path::Path::new("./").join(filename))?.decode()?;
    // img.convert_color_space(
    //     image::metadata::Cicp::SRGB_LINEAR,
    //     ConvertColorOptions::default(),
    //     image::ColorType::Rgba8,
    // );
    let img = img.to_rgba8();
    let palette = &palettes["websafe"];
    let palette_names = [
        "catppuccin-frappe",
        "catppuccin-mocha",
        "catppuccin-macchiato",
        "catppuccin-latte",
        "tokyonight-night",
        "tokyonight-storm",
        "tokyonight-day",
        "gruvbox-dark-all",
        "gruvbox-light-all",
        "kanagawa",
        "mono2",
        "websafe",
    ];

    let bayer = BayerMatrix::new((1, 1)).unwrap();
    let threshold_matrix = ThresholdMatrix::from(&bayer).normalized();
    let threshold_pattern: &dyn ThresholdPattern = &threshold_matrix;

    // let mut results = Vec::new();
    for palette_name in palette_names {
        let palette = &FastPalette::from(&palettes[palette_name]);
        let color_space = ColorSpace::RGB;

        let start = std::time::Instant::now();
        no_dither_par(&img, palette, color_space, 4096).save(format!(
            "./out/_{}_{}_quant.png",
            filename.file_stem().unwrap().to_str().unwrap(),
            palette_name
        ));
        noise_par(&img, palette, 32, false, color_space, 4096).save(format!(
            "./out/_{}_{}_noise.png",
            filename.file_stem().unwrap().to_str().unwrap(),
            palette_name
        ));
        pattern_par(&img, palette, threshold_pattern, color_space, 4096).save(format!(
            "./out/_{}_{}_pattern.png",
            filename.file_stem().unwrap().to_str().unwrap(),
            palette_name
        ));
        let duration = start.elapsed();
        println!("{palette_name}: {duration:?}");
    }

    // results.push(noise(&img.to_rgba8(), &palette.colors, 127, true));
    // results.push(error_diffusion(&img.to_rgba8(), &palette.colors, false));
    // results.push(error_diffusion(&img.to_rgba8(), &palette.colors, true));

    // for (id, res) in results.iter().enumerate() {
    //     let _ = res.save(format!(
    //         "./_{}{}.png",
    //         filename.file_stem().unwrap().to_str().unwrap(),
    //         id,
    //     ));
    // }

    Ok(())
}
