mod block;
mod fs;
mod json;
mod lang;
mod util;

use clap::Parser;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::Xoshiro256PlusPlus;
use std::path::Path;
use std::path::PathBuf;

pub type MyRng = Xoshiro256PlusPlus;

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Cli {
    /// Path to resource pack
    path: PathBuf,

    /// Do not replace resource pack description
    #[arg(long)]
    keep_meta: bool,

    /// Optional seed for random number generation
    #[arg(long)]
    seed: Option<u64>,
}

fn join_dir(base: &Path, path: &str) -> PathBuf {
    let joined: PathBuf = base.join(path);
    if !joined.is_dir() {
        cli_panic!("Incomplete resource pack: `{path}` directory missing");
    }
    joined
}

fn main() {
    let cli = Cli::parse();
    if !cli.path.is_dir() {
        cli_panic!("Resource pack directory does not exist: {}", cli.path.display());
    }

    let meta_file = cli.path.join("pack.mcmeta");
    if !meta_file.is_file() {
        cli_panic!("Directory is not a resource pack: pack.mcmeta file missing");
    }

    // Get all the directories first. If any dir is missing, the program aborts before making any changes.
    let assets_dir: PathBuf = join_dir(&cli.path, "assets/minecraft");
    let lang_dir: PathBuf = join_dir(&assets_dir, "lang");
    let particles_dir: PathBuf = join_dir(&assets_dir, "particles");
    let textures_dir: PathBuf = join_dir(&assets_dir, "textures");
    let item_dir: PathBuf = join_dir(&textures_dir, "item");
    let block_dir: PathBuf = join_dir(&textures_dir, "block");

    let seed: u64 = cli.seed.unwrap_or_else(|| rand::rng().next_u64());
    let rng = &mut Xoshiro256PlusPlus::seed_from_u64(seed);

    lang::shuffle_languages(&lang_dir, rng);
    block::shuffle_blocks(&block_dir, rng);
    fs::shuffle_json_files(&particles_dir, rng);
    fs::shuffle_json_files(&item_dir, rng);

    if !cli.keep_meta {
        let mut meta: json::packmeta::PackMeta = fs::read_json_file(&meta_file);
        meta.pack.description = util::rainbow_text("Resource Randomizer");
        meta.pack.description += "§r\nMade by §u§dBioTomateDE";
        fs::write_json_file(&meta_file, &meta);
    }

    println!("Resources shuffled with seed {seed}!");
}
