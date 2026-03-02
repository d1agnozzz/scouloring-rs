use std::{collections::HashMap, fs, path::Path};

use hex_color::HexColor;
use serde::{Deserialize, Serialize};

use crate::color::Color;

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
    pub colors: Vec<Color>,
}

impl From<PaletteDto> for Palette {
    fn from(val: PaletteDto) -> Self {
        Palette {
            name: val.name,
            colors: val
                .colors
                .iter()
                .map(|str| HexColor::parse(str).unwrap().into())
                .collect(),
        }
    }
}
