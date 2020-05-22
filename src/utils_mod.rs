//! utils_mod.rs

use unwrap::unwrap;

/// return the position after the delimiter
pub fn find_pos_after_delimiter(
    md_text_content: &str,
    pos: usize,
    delimiter: &str,
) -> Option<usize> {
    if let Some(pos_start_data) = find_from(md_text_content, pos, delimiter) {
        let pos_start_data = pos_start_data + delimiter.len();
        return Some(pos_start_data);
    }
    // return
    None
}

/// return the position before the delimiter
pub fn find_pos_before_delimiter(
    md_text_content: &str,
    pos: usize,
    delimiter: &str,
) -> Option<usize> {
    if let Some(pos_end_data) = find_from(md_text_content, pos, delimiter) {
        return Some(pos_end_data);
    }
    // return
    None
}

#[allow(clippy::integer_arithmetic)]
/// find from_pos
pub fn find_from(text: &str, from_pos: usize, find: &str) -> Option<usize> {
    let slice01 = text.get(from_pos..).unwrap();
    let option_location = slice01.find(find);
    if let Some(location) = option_location {
        // return Option with usize
        Some(from_pos + location)
    } else {
        // return
        None
    }
}
use std::{fs, io, path::Path};
/// traverse dir (sub-dir) with exclude dir
/// the find_file and the exclude dir strings must start with /
pub fn traverse_dir_with_exclude_dir(
    dir: &Path,
    find_file: &str,
    exclude_dirs: &Vec<String>,
) -> io::Result<Vec<String>> {
    // if the parameter is /*.rs, I can eliminate /*
    let find_file = &find_file.replace("/*", "");

    let mut v = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let str_path = unwrap!(path.to_str());
            if path.is_dir() {
                let mut is_excluded = false;
                for excl in exclude_dirs {
                    if str_path.ends_with(excl) {
                        is_excluded = true;
                        break;
                    }
                }
                if !is_excluded {
                    let mut sub_v = traverse_dir_with_exclude_dir(&path, find_file, exclude_dirs)?;
                    v.append(&mut sub_v);
                }
            } else {
                if str_path.ends_with(find_file) {
                    v.push(str_path.to_string());
                }
            }
        }
    }
    Ok(v)
}

/// parse semver ex. 12.99.88alpha
pub fn parse_semver(text: &str) -> (usize, usize, usize) {
    let pos = 0;
    let (major, pos) = parse_next_number(&text, pos);
    // jump over dot
    let pos = pos + 1;
    let (minor, pos) = parse_next_number(&text, pos);
    // jump over dot
    let pos = pos + 1;
    let (patch, _pos) = parse_next_number(&text, pos);
    // return
    (major, minor, patch)
}

/// parse next characters until is numeric or end
fn parse_next_number(text: &str, pos: usize) -> (usize, usize) {
    let mut pos = pos;
    let mut number = "".to_string();
    let mut one_char = text[pos..pos + 1].chars().next().unwrap();
    while one_char.is_numeric() {
        number.push(one_char);
        pos += 1;
        if pos > text.len() - 1 {
            break;
        }
        one_char = text[pos..pos + 1].chars().next().unwrap();
    }
    let number: usize = unwrap!(number.parse());
    // return
    (number, pos)
}
/// similar to ternary operator
pub fn conditional_usize(expr: bool, result_if_true: usize, result_if_false: usize) -> usize {
    if expr {
        result_if_true
    } else {
        result_if_false
    }
}

/// url encode
pub fn url_encode(data: &str) -> String {
    let mut escaped = String::new();
    for b in data.as_bytes().iter() {
        match *b as char {
            // Accepted characters
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => escaped.push(*b as char),

            // Everything else is percent-encoded
            b => escaped.push_str(format!("%{:02X}", b as u32).as_str()),
        };
    }
    return escaped;
}