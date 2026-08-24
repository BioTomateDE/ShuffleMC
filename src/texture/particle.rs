use crate::util::listdir_files;
use std::path::{Path, PathBuf};


pub fn shuffle_particles(particle_dir: &Path, rng: &mut crate::MyRng) {
    let source_files: Vec<PathBuf> = listdir_files(particle_dir, "json").collect();
    super::shuffle_files(source_files, rng);
}
