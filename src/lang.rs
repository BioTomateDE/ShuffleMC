//! Languages.

use crate::{
    cli_panic,
    util::{listdir_files, read_json_file, write_json_file},
};
use rand::seq::SliceRandom;
use std::{collections::HashMap, path::Path};

pub fn shuffle_languages(lang_dir: &Path, rng: &mut crate::MyRng) {
    for path in listdir_files(lang_dir, "json") {
        let name = path.file_stem().expect("read_dir never yields paths ending with `..`");
        let Some(name) = name.to_str() else {
            // Skip filenames with invalid UTF-8; they're not valid language json files anyway.
            continue;
        };
        if !is_language_code(name) {
            continue;
        }
        println!("Shuffling language {name}");
        let mut langmap: HashMap<String, String> = read_json_file(&path);
        shuffle_language(&mut langmap, rng);
        write_json_file(&path, &langmap);
    }
}

fn shuffle_language(langmap: &mut HashMap<String, String>, rng: &mut crate::MyRng) {
    // Setting this value too high will hog up memory.
    // Vanilla only uses like 5 placeholders at max (more is useless anyway unless you mod the game).
    const PLACEHOLDER_LIMIT: usize = 30;

    // Group language strings (keys) by placeholder count.
    let mut groups: [Vec<String>; PLACEHOLDER_LIMIT] = core::array::from_fn(|_| Vec::new());

    // You could also group by text length and/or whether it contains newlines.
    // Newlines are usually displayed as a box with "LF" written inside in-game.

    for (key, string) in langmap.iter() {
        if key.starts_with("translation.test") {
            // these contain invalid placeholders on purpose
            continue;
        }
        let placeholder_count = get_placeholder_count(string);
        let Some(langvec) = groups.get_mut(placeholder_count) else {
            cli_panic!(
                "Placeholder count {placeholder_count} exceeds my limit {PLACEHOLDER_LIMIT}"
            );
        };
        langvec.push(key.clone());
    }

    for keys in &mut groups {
        let mut values: Vec<String> = keys.iter().map(|key| langmap[key].clone()).collect();
        values.shuffle(rng);
        for (key, value) in keys.iter().zip(values) {
            *langmap.get_mut(key).unwrap() = value;
        }
    }
}

/// Gets the string formatting placeholder count.
///
/// This is either how many `%s` are in the string or the `%N$s` with the highest number N.
/// Should a string contain both `%s` and these explicit positional placeholders (i.e. `%4$s`),
/// then the higher count of the two is selected. The counts are independent.
fn get_placeholder_count(mut string: &str) -> usize {
    let mut implicit_count = 0;
    let mut explicit_count = 0;

    while let Some(idx) = string.find('%') {
        string = &string[idx + 1..];

        // "%%" denotes a literal "%"
        if let Some(rest) = string.strip_prefix('%') {
            string = rest;
            continue;
        }

        // "%s" is an "implicit" placeholder
        if let Some(rest) = string.strip_prefix('s') {
            string = rest; // only skips the 's', technically optional
            implicit_count += 1;
            continue;
        }

        // Neither '%' nor 's'; must be a positional placeholder like '%4$s'.
        let number_end = string.find(|c: char| !c.is_ascii_digit()).unwrap_or(string.len());
        if number_end == 0 {
            cli_panic!("Expected number after '%', found {string:?}");
        }
        let number_str = &string[..number_end];
        let Some(number) = number_str.parse::<usize>().ok().filter(|&n| i32::try_from(n).is_ok())
        else {
            cli_panic!("String formatting positional placeholder {number_str} is too high")
        };
        string = &string[number_end..];
        explicit_count = explicit_count.max(number);
        if let Some(rest) = string.strip_prefix("$s") {
            string = rest;
        } else {
            cli_panic!("Expected '$s' after positional placeholder, got {string:?}");
        }
    }

    implicit_count.max(explicit_count)
}

fn is_language_code(string: &str) -> bool {
    let bytes = string.as_bytes();
    bytes.len() == 5
        && bytes[0..2].iter().all(u8::is_ascii_lowercase)
        && bytes[2] == b'_'
        && bytes[3..5].iter().all(u8::is_ascii_lowercase)
}
