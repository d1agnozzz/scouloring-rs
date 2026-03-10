use image::{ImageBuffer, Rgba};
use rayon::iter::ParallelIterator;
use rayon::slice::ParallelSlice;

use crate::{
    color::{Color, ColorSpace},
    dithering::Image,
    palette::FastPalette,
};

pub fn no_dither_par(
    img: &Image,
    palette: &FastPalette,
    color_space: ColorSpace,
    chunk_size: usize,
) -> Image {
    let pixels: Vec<Rgba<u8>> = img.pixels().copied().collect();

    let quantized_pixels = pixels
        .par_chunks(chunk_size)
        .flat_map_iter(|chunk| {
            chunk
                .iter()
                .map(|pixel| palette.colors.nearest(&Color(*pixel)).unwrap().item.0)
        })
        .collect::<Vec<Rgba<u8>>>();

    Image::from_vec(
        img.width(),
        img.height(),
        quantized_pixels.into_iter().flat_map(|p| p.0).collect(),
    )
    .expect("could not create image buffer")
}
