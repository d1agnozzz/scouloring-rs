use image::{ImageReader, Pixel, Rgb, Rgba, RgbaImage};
use scouloring_rs::{
    color::{Color, ColorOps},
    palette::load_all_palettes,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let palettes = load_all_palettes("./palettes").unwrap();
    println!("{:?}", palettes);
    let img = ImageReader::open("./test3.png")?.decode()?.into_rgba8();
    let mut res = RgbaImage::new(img.width(), img.height());
    let palette = &palettes["rgbcmyk"].colors;

    for (ix, iy, pix) in img.enumerate_pixels() {
        let src = Rgba([pix.0[0], pix.0[1], pix.0[2], pix.0[3]]);
        let mut noised = src;
        let monorand = rand::random_range(-96..=96);

        for (i, s) in noised.0.into_iter().enumerate() {
            noised.0[i] = s.saturating_add_signed(monorand);
        }

        let palette_distance = palette
            .iter()
            .map(|col| {
                // let mut dist = 0;
                // for (i, ch) in col.0 .0.into_iter().enumerate() {
                //     if i > 2 {
                //         continue;
                //     }
                //     if ch > src.0[i] {
                //         dist += ((ch - src.0[i]) as usize).pow(2)
                //     } else {
                //         dist += ((src.0[i] - ch) as usize).pow(2)
                //     }
                // }
                col.distance_to(&Color(noised))
                // dist
            })
            .collect::<Vec<u32>>();

        let closest_color = palette[palette_distance
            .into_iter()
            .enumerate()
            .min_by_key(|(_idx, val)| *val)
            .unwrap()
            .0]
            .0;

        res.put_pixel(ix, iy, closest_color.to_rgba());
    }

    res.save("./out4.png");

    Ok(())
}
