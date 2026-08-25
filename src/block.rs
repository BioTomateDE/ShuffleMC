use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use png::{BitDepth, ColorType};

use crate::{
    cli_panic, fs::{listdir_files, read_json_file, shuffle_files}, json::texture::TextureProperties,
};

type PngReader = png::Reader<BufReader<File>>;

pub fn shuffle_blocks(block_dir: &Path, rng: &mut crate::MyRng) {
    let mut opaque_blocks: Vec<PathBuf> = Vec::new();
    let mut transparent_blocks: Vec<PathBuf> = Vec::new();

    for path in listdir_files(block_dir, "png") {
        // Example: "fire_0.png.mcmeta"
        let mcmeta_path = path.with_added_extension("mcmeta");
        if mcmeta_path.exists() {
            let meta: TextureProperties = read_json_file(&mcmeta_path);
            // Only shuffle non-animated blocks.
            if meta.animation.is_some() {
                continue;
            }
        }

        let png = open_png(&path);
        let info = png.info();
        // This breaks for resource packs with a different resolution!
        // If there is a better way to detect these weird textures, let me know.
        // Currently affected (32x32): dried_ghast_hydration_*_tentacles.png, *_shelf.png
        if info.width != 16 || info.height != 16 {
            continue;
        }

        if png_has_transparency(png) {
            transparent_blocks.push(path);
        } else {
            opaque_blocks.push(path);
        }
    }

    shuffle_files(&opaque_blocks, rng);
    shuffle_files(&transparent_blocks, rng);
}

fn open_png(path: &Path) -> PngReader {
    let file = File::open(path)
        .unwrap_or_else(|e| cli_panic!("Could not open file `{}`: {e}", path.display()));
    let decoder = png::Decoder::new(BufReader::new(file));
    decoder.read_info().unwrap_or_else(|e| {
        cli_panic!("Could not read PNG metadata for file `{}`: {e}", path.display())
    })
}

// ai slop warning
fn png_has_transparency(mut reader: PngReader) -> bool {
    let info = reader.info();

    match info.color_type {
        ColorType::Grayscale | ColorType::Rgb => {
            return info.trns.is_some();
        }
        ColorType::Indexed => {
            return info.trns.as_ref().is_some_and(|trns| trns.iter().any(|&a| a != 255));
        }
        ColorType::GrayscaleAlpha | ColorType::Rgba => {}
    }

    let samples = match info.color_type {
        ColorType::GrayscaleAlpha => 2,
        ColorType::Rgba => 4,
        _ => unreachable!("for other color types, the function already returned above"),
    };
    let bit_depth = info.bit_depth;

    loop {
        let row = reader.next_row().unwrap_or_else(|e| cli_panic!("Could not read PNG row: {e}"));
        let Some(row) = row else { return false };
        let data = row.data();
        match bit_depth {
            BitDepth::Eight => {
                for px in data.chunks_exact(samples) {
                    if px[samples - 1] != 255 {
                        return true;
                    }
                }
            }
            BitDepth::Sixteen => {
                for px in data.chunks_exact(samples * 2) {
                    let hi = px[px.len() - 2];
                    let lo = px[px.len() - 1];
                    if u16::from_be_bytes([hi, lo]) != 0xFFFF {
                        return true;
                    }
                }
            }
            _ => unreachable!("gray+alpha/rgba are always 8 or 16 bit"),
        }
    }
}
