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


#[allow(unused_imports)]
use ansi_term::Colour::{Blue, Green, Red, Yellow};
use chrono::prelude::*;

pub fn ns_start(text: &str) -> i64 {
    let now = Utc::now();
    if !text.is_empty() {
        eprintln!(
            "{}: {}",
            Green.paint(Local::now().format("%Y-%m-%d %H:%M:%S").to_string()),
            Green.paint(text)
        );
    }
    now.timestamp_nanos()
}

pub fn ns_print(name: &str, ns_start: i64) -> i64 {
    let now_ns = Utc::now().timestamp_nanos();
    let duration_ns = now_ns - ns_start;
    let duration_ns = duration_ns.to_string();
    let duration_ns = if duration_ns.len() == 4 {
        format!("          {}.{}", &duration_ns[0..1], &duration_ns[1..4])
    } else if duration_ns.len() == 5 {
        format!("         {}.{}", &duration_ns[0..2], &duration_ns[2..5])
    } else if duration_ns.len() == 6 {
        format!("        {}.{}", &duration_ns[0..3], &duration_ns[3..6])
    } else if duration_ns.len() == 7 {
        format!(
            "      {}.{}.{}",
            &duration_ns[0..1],
            &duration_ns[1..4],
            &duration_ns[4..7]
        )
    } else if duration_ns.len() == 8 {
        format!(
            "     {}.{}.{}",
            &duration_ns[0..2],
            &duration_ns[2..5],
            &duration_ns[5..8]
        )
    } else if duration_ns.len() == 9 {
        format!(
            "    {}.{}.{}",
            &duration_ns[0..3],
            &duration_ns[3..6],
            &duration_ns[6..9]
        )
    } else if duration_ns.len() == 10 {
        format!(
            "  {}.{}.{}.{}",
            &duration_ns[0..1],
            &duration_ns[1..4],
            &duration_ns[4..7],
            &duration_ns[7..10]
        )
    } else if duration_ns.len() == 11 {
        format!(
            " {}.{}.{}.{}",
            &duration_ns[0..2],
            &duration_ns[2..5],
            &duration_ns[5..8],
            &duration_ns[8..11]
        )
    } else if duration_ns.len() == 12 {
        format!(
            "{}.{}.{}.{}",
            &duration_ns[0..3],
            &duration_ns[3..6],
            &duration_ns[6..9],
            &duration_ns[9..12]
        )
    } else {
        duration_ns
    };
    eprintln!("{} ns : {}", duration_ns, name);
    // return
    now_ns
}
