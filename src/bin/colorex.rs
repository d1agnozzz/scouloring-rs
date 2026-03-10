use std::{
    env::args,
    fs::File,
    io::BufWriter,
    path::{Path, PathBuf},
};

use clap::Parser;
use scouloring_rs::{color::Color, color_extractor::extract_from_image, palette::PaletteDto};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    // Filename of the input image
    #[arg(short, long)]
    filenames: String,
}

fn main() {
    let args = Args::parse();

    let inputs = args.filenames.split(" ").to_owned();

    for input in inputs {
        let colors = extract_from_image(input);

        match colors {
            Ok(colors_set) => {
                let filepath = PathBuf::from(input);
                let colors_vec = colors_set
                    .into_iter()
                    .map(|c| c.to_hex())
                    .collect::<Vec<String>>();

                let palette = PaletteDto {
                    name: filepath.file_stem().unwrap().to_str().unwrap().to_owned(),
                    colors: colors_vec,
                };
                let file = File::create(filepath.with_extension("json")).unwrap();
                let mut writer = BufWriter::new(file);
                serde_json::to_writer(&mut writer, &palette).unwrap();
            }
            Err(e) => println!("Could not extract colors: {e}"),
        }
    }
}
