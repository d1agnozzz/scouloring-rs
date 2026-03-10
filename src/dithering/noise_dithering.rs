use image::Rgba;

use crate::color::Color;
use crate::{color::ColorSpace, dithering::Image, palette::FastPalette};
use rayon::iter::ParallelIterator;
use rayon::slice::ParallelSlice;

pub fn noise_par(
    img: &Image,
    palette: &FastPalette,
    noise_scale: i8,
    mono_noise: bool,
    color_space: ColorSpace,
    chunk_size: usize,
) -> Image {
    let pixels: Vec<Rgba<u8>> = img.pixels().copied().collect();

    let quantized_pixels = pixels
        .par_chunks(chunk_size)
        .flat_map_iter(|chunk| {
            chunk.iter().map(|pixel| {
                let mut noised = *pixel;
                let mut rnd = rand::random_range(-noise_scale..=noise_scale);
                for (i, s) in noised.0.into_iter().enumerate() {
                    if !mono_noise {
                        rnd = rand::random_range(-noise_scale..=noise_scale);
                    }
                    noised.0[i] = s.saturating_add_signed(rnd);
                }

                palette.colors.nearest(&Color(noised)).unwrap().item.0
            })
        })
        .collect::<Vec<Rgba<u8>>>();

    Image::from_vec(
        img.width(),
        img.height(),
        quantized_pixels.into_iter().flat_map(|p| p.0).collect(),
    )
    .expect("could not create image buffer")
}
