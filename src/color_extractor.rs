use std::{collections::HashSet, io::ErrorKind, path::Path};

use image::{ImageReader, Pixel};
use rayon::iter::ParallelIterator;

use crate::color::Color;

pub fn extract_from_image<P: AsRef<Path>>(
    filename: P,
) -> Result<HashSet<Color<u8>>, std::io::Error> {
    let img = ImageReader::open(filename)?
        .decode()
        .or(Err(std::io::Error::from(ErrorKind::InvalidData)))?;

    Ok(img
        .to_rgb8()
        .par_pixels()
        .map(|m| Color::<u8>(m.to_rgba()))
        .collect::<HashSet<Color<u8>>>())
}
