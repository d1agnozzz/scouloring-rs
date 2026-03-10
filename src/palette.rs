use std::{collections::HashMap, fs, path::Path};

use hex_color::HexColor;
use serde::{Deserialize, Serialize};

use crate::color::{self, Color};

#[derive(Debug, Serialize, Deserialize)]
struct PaletteDto {
    name: String,
    colors: Vec<String>,
}

pub fn load_all_palettes<P: AsRef<Path>>(
    dir: P,
) -> Result<HashMap<String, Palette>, Box<dyn std::error::Error>> {
    let mut palettes = HashMap::new();

    let entries = fs::read_dir(dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let key = path.file_stem().unwrap().to_str().unwrap().to_owned();

        if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
            let content = fs::read_to_string(&path)?;
            match serde_json::from_str::<PaletteDto>(&content) {
                Ok(palette) => {
                    palettes.insert(key, palette.into());
                }
                Err(e) => eprintln!("Failed to parse {}: {}", path.display(), e),
            }
        }
    }

    Ok(palettes)
}

#[derive(Debug)]
pub struct Palette {
    pub name: String,
    pub colors: Vec<Color<u8>>,
}

pub struct FastPalette {
    pub name: String,
    pub colors: kd_tree::KdTree<Color<u8>>,
}

impl From<&Palette> for FastPalette {
    fn from(p: &Palette) -> Self {
        FastPalette {
            name: p.name.clone(),
            colors: kd_tree::KdTree::build_by(p.colors.clone(), |item1, item2, k| {
                item1.0 .0[k].cmp(&item2.0 .0[k])
            }),
        }
    }
}

impl From<PaletteDto> for Palette {
    fn from(dto: PaletteDto) -> Self {
        Palette {
            name: dto.name,
            colors: dto
                .colors
                .iter()
                .map(|str| HexColor::parse(str).unwrap().into())
                .collect(),
        }
    }
}
impl Palette {
    pub fn closest_color_id(&self, color: &Color<u8>, color_space: color::ColorSpace) -> usize {
        self.colors
            .iter()
            .map(|c| c.distace_to(&color, color_space))
            .enumerate()
            .min_by(|(_, a), (_, b)| a.total_cmp(b))
            .unwrap()
            .0
    }

    pub fn closest_color(&self, color: &Color<u8>, color_space: color::ColorSpace) -> Color<u8> {
        self.colors[self.closest_color_id(color, color_space)]
    }
}
