use crate::{dithering::ordered::threshold_pattern::ThresholdPattern, BayerMatrix};
use image::{ImageBuffer, Luma};

pub struct ThresholdMatrix {
    size: (u32, u32),
    data: Vec<f32>,
}

impl From<&BayerMatrix> for ThresholdMatrix {
    fn from(value: &BayerMatrix) -> Self {
        ThresholdMatrix {
            size: value.size,
            data: value
                .data
                .iter()
                .map(|v| *v as f32 / (value.size.0 * value.size.1) as f32)
                .collect(),
        }
    }
}

impl ThresholdMatrix {
    pub fn to_image(&self) -> Option<ImageBuffer<Luma<u8>, Vec<u8>>> {
        let to8bit = self
            .data
            .iter()
            .map(|v| (v * 255.0) as u8)
            .collect::<Vec<u8>>();

        ImageBuffer::<Luma<u8>, _>::from_vec(self.size.0, self.size.1, to8bit)
    }

    pub fn normalized(self) -> Self {
        ThresholdMatrix {
            size: self.size,
            data: self.data.iter().map(|v| v - 0.5).collect(),
        }
    }

    pub fn print(&self) {
        for i in 0..self.size.0 as usize {
            for j in 0..self.size.1 as usize {
                print!("{:4}", self.data[i * self.size.1 as usize + j])
            }
            println!();
        }
    }
}

impl ThresholdPattern for ThresholdMatrix {
    fn at_wrapping(&self, x: u32, y: u32) -> f32 {
        let wrapped_x = x % self.size.0;
        let wrapped_y = y % self.size.1;
        self.data[(wrapped_x * self.size.0 + wrapped_y) as usize]
    }
}
