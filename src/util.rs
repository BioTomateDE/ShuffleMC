use std::{
    any::type_name,
    path::{Path, PathBuf},
};

#[macro_export]
macro_rules! cli_panic {
    ($($arg:tt)*) => {
        <$crate::Cli as ::clap::CommandFactory>::command().error(::clap::error::ErrorKind::ValueValidation, format!($($arg)*)).exit()
    };
}

pub fn listdir_files(dir_path: &Path, extension: &str) -> impl Iterator<Item = PathBuf> {
    listdir_all(dir_path)
        .filter(move |path| path.is_file() && path.extension().is_some_and(|ext| ext == extension))
}

pub fn listdir_all(dir_path: &Path) -> impl Iterator<Item = PathBuf> {
    let read_dir: std::fs::ReadDir = std::fs::read_dir(dir_path)
        .unwrap_or_else(|e| cli_panic!("Could not list directory `{}`: {e}", dir_path.display()));
    read_dir.filter_map(|entry| {
        let entry = entry.unwrap_or_else(|e| {
            cli_panic!("Could not read entry in `{}`: {e}", dir_path.display())
        });
        // Skip hidden files/dirs that start with a dot
        if entry.file_name().as_encoded_bytes().starts_with(b".") {
            return None;
        }
        Some(entry.path())
    })
}

pub fn rename_file(old_path: &Path, new_path: &Path) {
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
    serde_json::from_str(&string)
        .unwrap_or_else(|e| cli_panic!("Invalid JSON for file `{}`: {e}", path.display()))
}

pub fn write_json_file<T: serde::Serialize>(path: &Path, value: &T) {
    let string: String = serde_json::to_string_pretty(&value)
        .unwrap_or_else(|e| cli_panic!("Could not serialize JSON for {}: {e}", type_name::<T>()));
    if let Err(e) = std::fs::write(path, string) {
        cli_panic!("Could not write JSON file `{}`: {e}", path.display());
    }
}

pub fn rainbow_text(text: &str) -> String {
    const COLORS: [char; 7] = ['c', '6', 'e', 'a', 'b', '9', 'd'];
    const ESCAPE: char = '§';

    let mut output = String::with_capacity(text.len() * 3);
    let mut idx = 0;

    for ch in text.chars() {
        if ch.is_whitespace() {
            output.push(ch);
            continue;
        }
        let color: char = COLORS[idx];
        idx += 1;
        idx %= COLORS.len();
        output.push(ESCAPE);
        output.push(color);
        output.push(ch);
        if ch == ESCAPE {
            output.push(ESCAPE);
        }
    }

    output
}
