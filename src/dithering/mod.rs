pub mod no_dithering;
pub mod noise_dithering;
pub mod ordered;

use image::{ImageBuffer, Rgba};
pub use ordered::BayerMatrix;

pub type Image = ImageBuffer<Rgba<u8>, Vec<u8>>;
