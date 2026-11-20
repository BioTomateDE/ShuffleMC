//! Filesystem utility functions.

use crate::cli_panic;
use rand::seq::SliceRandom;
use std::path::Path;
use std::path::PathBuf;

pub fn listdir_files(dir_path: &Path, extension: &str) -> impl Iterator<Item = PathBuf> {
    listdir_all(dir_path).filter(move |path| path.is_file() && path.extension().is_some_and(|ext| ext == extension))
}

pub fn listdir_all(dir_path: &Path) -> impl Iterator<Item = PathBuf> {
    let read_dir: std::fs::ReadDir = std::fs::read_dir(dir_path)
        .unwrap_or_else(|e| cli_panic!("Could not list directory `{}`: {e}", dir_path.display()));
    read_dir.filter_map(|entry| {
        let entry = entry.unwrap_or_else(|e| cli_panic!("Could not read entry in `{}`: {e}", dir_path.display()));
        // Skip hidden files/dirs that start with a dot
        if entry.file_name().as_encoded_bytes().starts_with(b".") {
            return None;
        }
        Some(entry.path())
    })
}

pub fn rename_file(old_path: &Path, new_path: &Path) {
    if cfg!(debug_assertions) && new_path.exists() {
        cli_panic!(
            "[DEBUG] Could not rename file `{}` to `{}`: target file already exists",
            old_path.display(),
            new_path.display(),
        );
    }

    // Unfortunately, this function can replace existing files.
    if let Err(e) = std::fs::rename(old_path, new_path) {
        cli_panic!(
            "Could not rename file `{}` to `{}`: {e}",
            old_path.display(),
            new_path.display(),
        );
    }
}

pub fn read_json_file<T: serde::de::DeserializeOwned>(path: &Path) -> T {
    let string: String = std::fs::read_to_string(path)
        .unwrap_or_else(|e| cli_panic!("Could not read JSON file `{}`: {e}", path.display()));
    serde_json::from_str(&string).unwrap_or_else(|e| cli_panic!("Invalid JSON for file `{}`: {e}", path.display()))
}

pub fn write_json_file<T: serde::Serialize>(path: &Path, value: &T) {
    let string: String = serde_json::to_string_pretty(&value)
        .unwrap_or_else(|e| cli_panic!("Could not serialize JSON for {}: {e}", std::any::type_name::<T>()));
    if let Err(e) = std::fs::write(path, string) {
        cli_panic!("Could not write JSON file `{}`: {e}", path.display());
    }
}

// This is an optimized file name shuffling algorithm that avoids as many temporary renames as possible.
// The saved filesystem operations far outweigh the slight extra CPU work.
pub fn shuffle_files<P: AsRef<Path>>(files: &[P], rng: &mut crate::MyRng) {
    let mut order: Box<[usize]> = (0..files.len()).into_iter().collect();
    order.shuffle(rng);

    let mut visited: Box<[bool]> = std::iter::repeat_n(false, files.len()).collect();

    for start in 0..files.len() {
        if visited[start] || order[start] == start {
            visited[start] = true;
            continue;
        }

        // Evacuate the cycle's start once, then walk the whole cycle in one pass.
        let tmp = tempfile(files[start].as_ref());
        rename_file(files[start].as_ref(), &tmp);
        visited[start] = true;

        let mut path = vec![start];
        let mut cur = start;
        loop {
            let next = order[cur];
            if next == start {
                break;
            }
            path.push(next);
            visited[next] = true;
            cur = next;
        }

        let k = path.len();
        rename_file(files[path[k - 1]].as_ref(), files[path[0]].as_ref());
        for i in (1..k).rev() {
            if i == 1 {
                rename_file(&tmp, files[path[1]].as_ref());
            } else {
                rename_file(files[path[i - 1]].as_ref(), files[path[i]].as_ref());
            }
        }
    }
}

// This function also shuffles files but always needs 2N renames (where N is the file count).
// It performs almost twice as many renames as the above function and takes twice as long.
fn _shuffle_files_simplified(source_files: &[PathBuf], rng: &mut crate::MyRng) {
    let mut dest_files = source_files.to_vec();
    dest_files.shuffle(rng);

    for path in source_files {
        rename_file(path, &tempfile(path));
    }

    for (src, dest) in source_files.iter().zip(dest_files) {
        rename_file(&tempfile(src), &dest);
    }
}

#[inline]
fn tempfile(path: &Path) -> PathBuf {
    // It is assumed that this file does not already exist (still checked if debug_assertions are enabled).
    // This could only happen if multiple instances of this program are running at the same time.
    path.with_added_extension("tmp")
}

#[inline]
pub fn shuffle_json_files(directory: &Path, rng: &mut crate::MyRng) {
    let file_paths: Box<[PathBuf]> = listdir_files(directory, "json").collect();
    shuffle_files(&file_paths, rng);
}
