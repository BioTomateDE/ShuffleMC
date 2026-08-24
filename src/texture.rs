use crate::{cli_panic, util::rename_file};
use rand::seq::SliceRandom;
use std::path::PathBuf;

pub mod block;
pub mod particle;
pub mod item;

fn shuffle_files(mut source_files: Vec<PathBuf>, rng: &mut crate::MyRng) {
    let mut dest_files = source_files.clone();
    dest_files.shuffle(rng);

    for path in &mut source_files {
        let new_path = path.with_added_extension("bak");
        if new_path.exists() {
            cli_panic!("Temp file somehow already exists: {}", new_path.display());
        }
        rename_file(path, &new_path);
        *path = new_path;
    }

    for (src, dest) in source_files.iter().zip(&dest_files) {
        rename_file(src, dest);
    }
}
