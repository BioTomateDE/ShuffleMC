use crate::util::listdir_files;
use std::path::{Path, PathBuf};

// Items are easy to shuffle; you can just swap PNGs.
pub fn shuffle_items(item_dir: &Path, rng: &mut crate::MyRng) {
    let source_files: Vec<PathBuf> = listdir_files(item_dir, "png").collect();
    super::shuffle_files(source_files, rng);
}
