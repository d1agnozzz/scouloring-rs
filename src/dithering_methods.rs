use image::{ImageBuffer, Pixel, Rgba};

use crate::{
    color::{Color, ColorSpace},
    palette::{FastPalette, Palette},
};

use rayon::{
    iter::{ParallelBridge, ParallelIterator},
    slice::ParallelSlice,
};

type Image = ImageBuffer<Rgba<u8>, Vec<u8>>;

pub fn no_dither(img: &Image, colors: &[Color<u8>]) -> Image {
    let mut res = image::ImageBuffer::new(img.width(), img.height());

    for (ix, iy, pix) in img.enumerate_pixels() {
        // TODO: separate into palette method
        let palette_distance = colors
            .iter()
            .map(|col| col.distace_to(&Color(*pix), ColorSpace::RGB))
            .collect::<Vec<f32>>();

        let closest_color = colors[palette_distance
            .into_iter()
            .enumerate()
            .min_by(|(_, x), (_, y)| x.total_cmp(y))
            .unwrap()
            .0]
            .0;

        res.put_pixel(ix, iy, closest_color.to_rgba());
    }
    res
}

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

    ImageBuffer::<Rgba<u8>, _>::from_vec(
        img.width(),
        img.height(),
        quantized_pixels.into_iter().flat_map(|p| p.0).collect(),
    )
    .expect("could not create image buffer")
}

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

    ImageBuffer::<Rgba<u8>, _>::from_vec(
        img.width(),
        img.height(),
        quantized_pixels.into_iter().flat_map(|p| p.0).collect(),
    )
    .expect("could not create image buffer")
}

pub fn noise(img: &Image, colors: &[Color<u8>], noise_scale: i8, mono_noise: bool) -> Image {
    let mut res = ImageBuffer::new(img.width(), img.height());
    for (ix, iy, pix) in img.enumerate_pixels() {
        let mut noised = *pix;
        let mut rnd = rand::random_range(-noise_scale..=noise_scale);

        for (i, s) in noised.0.into_iter().enumerate() {
            if !mono_noise {
                rnd = rand::random_range(-noise_scale..=noise_scale);
            }
            noised.0[i] = s.saturating_add_signed(rnd);
        }

        // TODO: separate into palette method
        let palette_distance = colors
            .iter()
            .map(|col| col.distace_to(&Color(noised), ColorSpace::RGB))
            .collect::<Vec<f32>>();

        let closest_color = colors[palette_distance
            .into_iter()
            .enumerate()
            .min_by(|(_, x), (_, y)| x.total_cmp(y))
            .unwrap()
            .0]
            .0;

        res.put_pixel(ix, iy, closest_color.to_rgba());
    }
    res
}

pub fn error_diffusion(img: &Image, palette: &FastPalette, color_space: ColorSpace) -> Image {
    let mut res = ImageBuffer::new(img.width(), img.height());

    for y in 0..img.height() {
        let mut error_pix = Color::<i64>(Rgba([0, 0, 0, 0]));
        for x in 0..img.width() {
            let pix = *img.get_pixel(x, y);
            let with_error = Color(pix).add_i64(&error_pix);
            // TODO: separate into palette method
            // let closest_color_id = colors
            //     .iter()
            //     .map(|color| {
            //         if perpceptive {
            //             color.distace_to(&with_error.to_u8_clamped(), ColorSpace::OKLab)
            //         } else {
            //             color.distace_to(&with_error.to_u8_clamped(), ColorSpace::RGB)
            //         }
            //     })
            //     .enumerate()
            //     .min_by(|(_, x), (_, y)| x.cmp(y))
            //     .unwrap()
            //     .0;
            let closest_color = palette
                .colors
                .nearest(&with_error.to_u8_clamped())
                .expect("kd tree error")
                .item;
            // let closest_color = palette.closest_color(&with_error.to_u8_clamped(), color_space);

            error_pix = with_error.sub_u8(closest_color);
            // error_pix = Color(pix)
            //     .add_i64(&Color::<i64>(Rgba([0, 0, 0, 0])))
            //     .sub_u8(closest_color);

            res.put_pixel(x, y, closest_color.0);
        }
    }
    let res = image::DynamicImage::from(res);

    res.into()
}
