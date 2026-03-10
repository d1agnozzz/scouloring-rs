use image::RgbImage;
use std::thread::current;

use crate::dithering::ordered::theshold_matrix::ThresholdMatrix;

#[derive(Debug, PartialEq)]
pub struct BayerMatrix {
    pub size: (u32, u32),
    pub(crate) data: Vec<u32>,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum BayerError {
    #[error("Invalid matrix power: {0}. Power must be at least 1")]
    InvalidSize(usize),
}

impl BayerMatrix {
    /// Takes `(x_level, y_level)` as parameters and generates Bayer matrix of size `(2^( x_level +
    /// 1), 2^( y_level + 1))`
    ///
    /// This is an adaptation of the algorithm from [Joel Yliluoma's work](https://bisqwit.iki.fi/story/howto/dither/jy/#Appendix%202ThresholdMatrix).
    pub fn new((x_level, y_level): (usize, usize)) -> Result<Self, BayerError> {
        let (x_power, y_power) = (x_level + 1, y_level + 1);
        let xdim = 1 << x_power;
        let ydim = 1 << y_power;
        let mut data = vec![0; xdim * ydim];
        for y in 0..ydim {
            for x in 0..xdim {
                let mut v = 0;
                let mut offset = 0;
                let mut xmask = x_power;
                let mut ymask = y_power;
                if x_power == 0 || (x_power > y_power && y_power != 0) {
                    let xc = x ^ ((y << x_power) >> y_power);
                    let yc = y;
                    let mut bit = 0;
                    while bit < x_power + y_power {
                        ymask = ymask.wrapping_sub(1);
                        v |= ((yc >> ymask) & 1) << bit;
                        bit += 1;
                        offset += x_power;
                        while offset >= y_power {
                            xmask = xmask.wrapping_sub(1);
                            v |= ((xc >> xmask) & 1) << bit;
                            bit += 1;
                            offset -= y_power;
                        }
                    }
                } else {
                    let xc = x;
                    let yc = y ^ ((x << y_power) >> x_power);
                    let mut bit = 0;
                    while bit < x_power + y_power {
                        xmask = xmask.wrapping_sub(1);
                        v |= ((xc >> xmask) & 1) << bit;
                        bit += 1;
                        offset += y_power;
                        while offset >= x_power {
                            ymask = ymask.wrapping_sub(1);
                            v |= ((yc >> ymask) & 1) << bit;
                            bit += 1;
                            offset -= x_power;
                        }
                    }
                }
                data[x * ydim + y] = v as u32;
            }
        }
        Ok(BayerMatrix {
            size: (xdim as u32, ydim as u32),
            data,
        })
    }

    pub fn to_thresholds(&self) -> ThresholdMatrix {
        self.into()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bad_argument() {
        let result = BayerMatrix::new((0, 0));
        assert_eq!(result, Err(BayerError::InvalidSize(0)));
    }

    mod regular {
        use super::*;
        #[test]
        fn test_2x2() {
            let matrix = BayerMatrix::new((1, 1)).unwrap();
            assert_eq!(matrix.data.as_slice(), [0, 2, 3, 1]);
        }

        #[test]
        fn text_4x4() {
            let matrix = BayerMatrix::new((2, 2)).unwrap();
            assert_eq!(
                matrix.data.as_slice(),
                [0, 8, 2, 10, 12, 4, 14, 6, 3, 11, 1, 9, 15, 7, 13, 5]
            )
        }

        #[test]
        fn test_8x8() {
            let matrix = BayerMatrix::new((3, 3)).unwrap();
            assert_eq!(
                matrix.data.as_slice(),
                [
                    0, 32, 8, 40, 2, 34, 10, 42, 48, 16, 56, 24, 50, 18, 58, 26, 12, 44, 4, 36, 14,
                    46, 6, 38, 60, 28, 52, 20, 62, 30, 54, 22, 3, 35, 11, 43, 1, 33, 9, 41, 51, 19,
                    59, 27, 49, 17, 57, 25, 15, 47, 7, 39, 13, 45, 5, 37, 63, 31, 55, 23, 61, 29,
                    53, 21
                ]
            )
        }
    }

    mod irregular {
        use super::*;

        #[test]
        fn test_4x2() {
            let matrix = BayerMatrix::new((2, 1)).unwrap();
            assert_eq!(matrix.data.as_slice(), [0, 3, 4, 7, 2, 1, 6, 5])
        }
        #[test]
        fn test_2x4() {
            let matrix = BayerMatrix::new((1, 2)).unwrap();
            assert_eq!(matrix.data.as_slice(), [0, 4, 2, 6, 3, 7, 1, 5])
        }
        #[test]
        fn test_8x2() {
            let matrix = BayerMatrix::new((3, 1)).unwrap();
            assert_eq!(
                matrix.data.as_slice(),
                [0, 3, 8, 11, 4, 7, 12, 15, 2, 1, 10, 9, 6, 5, 14, 13]
            )
        }
        #[test]
        fn test_8x4() {
            let matrix = BayerMatrix::new((3, 2)).unwrap();
            assert_eq!(
                matrix.data.as_slice(),
                [
                    0, 12, 3, 15, 16, 28, 19, 31, 8, 4, 11, 7, 24, 20, 27, 23, 2, 14, 1, 13, 18,
                    30, 17, 29, 10, 6, 9, 5, 26, 22, 25, 21
                ]
            )
        }
    }
}
