use hex_color::HexColor;
use image::Rgba;

// pub type Color = Rgba<u8>;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color<T>(pub Rgba<T>);

impl kd_tree::KdPoint for Color<u8> {
    type Scalar = i32;

    type Dim = typenum::U3;

    fn at(&self, i: usize) -> Self::Scalar {
        self.0 .0[i] as i32
    }
}

#[derive(Clone, Copy)]
pub enum ColorSpace {
    RGB,
    OKLab,
}

impl From<HexColor> for Color<u8> {
    fn from(value: HexColor) -> Self {
        Color(Rgba([value.r, value.g, value.b, value.a]))
    }
}

impl From<Rgba<u8>> for Color<u8> {
    fn from(value: Rgba<u8>) -> Self {
        Color(value)
    }
}

impl Color<u8> {
    fn r(&self) -> u8 {
        self.0[0]
    }

    fn g(&self) -> u8 {
        self.0[1]
    }

    fn b(&self) -> u8 {
        self.0[2]
    }
    fn a(&self) -> u8 {
        self.0[3]
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r(), self.g(), self.b())
    }
    fn to_oklab(&self) -> oklab::Oklab {
        oklab::srgb_to_oklab(oklab::Rgb {
            r: self.r(),
            g: self.g(),
            b: self.b(),
        })
    }
    pub fn distace_to(&self, other: &Self, color_space: ColorSpace) -> f32 {
        match color_space {
            ColorSpace::RGB => self.rgb_distance(other),
            ColorSpace::OKLab => self.oklab_distance(other),
        }
    }

    fn oklab_distance(&self, other: &Self) -> f32 {
        let self_oklab = self.to_oklab();
        let other_oklab = other.to_oklab();

        ((self_oklab.l - other_oklab.l).powi(2)
            + (self_oklab.a - other_oklab.a).powi(2)
            + (self_oklab.b - other_oklab.b).powi(2))
        .sqrt()
    }

    fn rgb_distance(&self, other: &Self) -> f32 {
        ((self
            .0
             .0
            .iter()
            .zip(other.0 .0.iter())
            .map(|(a, b)| {
                let diff = a.abs_diff(*b);
                diff as u32 * diff as u32
            })
            .sum::<u32>()) as f32)
            .sqrt()
    }
    fn sub(&self, other: &Self) -> Color<i16> {
        Color::<i16>(Rgba(
            self.0
                 .0
                .into_iter()
                .zip(other.0 .0)
                .map(|(a, b)| a as i16 - b as i16)
                .collect::<Vec<i16>>()
                .try_into()
                .unwrap(),
        ))
    }
    pub fn add_i64(&self, other: &Color<i64>) -> Color<i64> {
        Color::<i64>(Rgba(
            self.0
                 .0
                .into_iter()
                .zip(other.0 .0)
                .map(|(a, b)| a as i64 + b)
                .collect::<Vec<i64>>()
                .try_into()
                .unwrap(),
        ))
    }

    fn add(&self, other: &Self) -> Color<i16> {
        Color::<i16>(Rgba(
            self.0
                 .0
                .into_iter()
                .zip(other.0 .0)
                .map(|(a, b)| a as i16 + b as i16)
                .collect::<Vec<i16>>()
                .try_into()
                .unwrap(),
        ))
    }
}

impl Color<i64> {
    pub fn to_u8_clamped(&self) -> Color<u8> {
        Color(Rgba([
            self.0[0].clamp(0, 255) as u8,
            self.0[1].clamp(0, 255) as u8,
            self.0[2].clamp(0, 255) as u8,
            self.0[3].clamp(0, 255) as u8,
        ]))
    }

    pub fn sub_u8(&self, other: &Color<u8>) -> Self {
        Color::<i64>(Rgba(
            self.0
                 .0
                .into_iter()
                .zip(other.0 .0)
                .map(|(a, b)| a - b as i64)
                .collect::<Vec<i64>>()
                .try_into()
                .unwrap(),
        ))
    }
}
