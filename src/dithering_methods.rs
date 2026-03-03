use image::{buffer::ConvertBuffer, ConvertColorOptions, ImageBuffer, Pixel, Rgba};

use crate::color::Color;

type Image = ImageBuffer<Rgba<u8>, Vec<u8>>;

pub fn no_dither(img: &Image, colors: &[Color<u8>]) -> Image {
    let mut res = image::ImageBuffer::new(img.width(), img.height());

    for (ix, iy, pix) in img.enumerate_pixels() {
        // TODO: separate into palette method
        let palette_distance = colors
            .iter()
            .map(|col| col.rgb_distance(&Color(*pix)))
            .collect::<Vec<u32>>();

        let closest_color = colors[palette_distance
            .into_iter()
            .enumerate()
            .min_by_key(|(_idx, val)| *val)
            .unwrap()
            .0]
            .0;

        res.put_pixel(ix, iy, closest_color.to_rgba());
    }
    res
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
            .map(|col| col.rgb_distance(&Color(noised)))
            .collect::<Vec<u32>>();

        let closest_color = colors[palette_distance
            .into_iter()
            .enumerate()
            .min_by_key(|(_idx, val)| *val)
            .unwrap()
            .0]
            .0;

        res.put_pixel(ix, iy, closest_color.to_rgba());
    }
    res
}

pub fn error_diffusion(img: &Image, colors: &[Color<u8>]) -> Image {
    let mut res = ImageBuffer::new(img.width(), img.height());

    for x in 0..img.width() {
        let mut error_pix = Color::<i64>(Rgba([0, 0, 0, 0]));
        for y in 0..img.height() {
            let pix = *img.get_pixel(x, y);
            let with_error = Color(pix).add_i64(&error_pix);
            // TODO: separate into palette method
            let pallete_distance = colors
                .iter()
                .map(|col| col.oklab_distance(&with_error.to_u8_clamped()))
                .collect::<Vec<_>>();
            let closest_color = &colors[pallete_distance
                .into_iter()
                .enumerate()
                .min_by(|x, y| x.1.total_cmp(&y.1))
                .unwrap()
                .0];
            error_pix = with_error.sub_u8(&closest_color);
            res.put_pixel(x, y, closest_color.0);
        }
    }
    let mut res = image::DynamicImage::from(res);

    res.into()
}
