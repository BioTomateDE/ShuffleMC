use crate::{
    cli_panic,
    util::{listdir_files, rename_file},
};
use rand::seq::SliceRandom;
use std::path::{Path, PathBuf};

// Items are easy to shuffle; you can just swap PNGs.
pub fn shuffle_items(item_dir: &Path, rng: &mut crate::MyRng) {
    let mut source_files: Vec<PathBuf> = listdir_files(item_dir, "png").collect();
    let mut dest_files = source_files.clone();
    dest_files.shuffle(rng);
    
    for path in &mut source_files {
        let new_path = path.with_added_extension("bak");
        if new_path.exists() {
            cli_panic!("Temp file somehow already exists: {}", new_path.display())
        }
        rename_file(path, &new_path);
        *path = new_path;
    }
    
    for (src, dest) in source_files.iter().zip(&dest_files) {
        rename_file(src, dest);
    }
}
