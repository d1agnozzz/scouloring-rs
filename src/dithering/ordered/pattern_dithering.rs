use std::usize;

use image::Rgba;

use crate::color::Color;
use crate::dithering::ordered::theshold_matrix::ThresholdMatrix;
use crate::dithering::ordered::threshold_pattern::ThresholdPattern;
use crate::{color::ColorSpace, dithering::Image, palette::FastPalette};
use rayon::iter::ParallelIterator;
use rayon::slice::ParallelSlice;

// TODO: implement Thomas Knoll's algo from Photoshop and Joel Yliluoma's algorithm for ordered
// dithering
pub fn pattern_par(
    img: &Image,
    palette: &FastPalette,
    threshold_pattern: &dyn ThresholdPattern,
    color_space: ColorSpace,
    chunk_size: usize,
) -> Image {
    let pixels = img
        .enumerate_pixels()
        .map(|(x, y, p)| (x, y, p.clone()))
        .collect::<Vec<(u32, u32, Rgba<u8>)>>();

    let quantized_pixels = pixels
        .par_chunks(chunk_size)
        .flat_map_iter(|chunk| {
            chunk.iter().map(|(x, y, pixel)| {
                let mut thresholded = *pixel;
                let threshold_u8 = (threshold_pattern.at_wrapping(*x, *y) * 255.0) as i8;
                for (i, s) in thresholded.0.into_iter().enumerate() {
                    thresholded.0[i] = s.saturating_add_signed(threshold_u8);
                }

                palette.colors.nearest(&Color(thresholded)).unwrap().item.0
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
