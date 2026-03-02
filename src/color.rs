use hex_color::HexColor;
use image::Rgba;

// pub type Color = Rgba<u8>;
#[derive(Debug)]
pub struct Color(pub Rgba<u8>);

pub trait ColorOps {
    fn r(&self) -> u8;
    fn g(&self) -> u8;
    fn b(&self) -> u8;
    fn a(&self) -> u8;
    fn to_hex(&self) -> String;
    fn distance_to(&self, other: &Self) -> u32;
}

impl From<HexColor> for Color {
    fn from(value: HexColor) -> Self {
        Color(Rgba([value.r, value.g, value.b, value.a]))
    }
}

impl ColorOps for Color {
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

    fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r(), self.g(), self.b())
    }

    fn distance_to(&self, other: &Self) -> u32 {
        ((self
            .0
             .0
            .iter()
            .zip(other.0 .0.iter())
            .map(|(a, b)| {
                let diff = a.abs_diff(*b);
                diff as u32 * diff as u32
            })
            .sum::<u32>()) as f64)
            .sqrt() as u32
    }
}
