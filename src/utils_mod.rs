//! utils_mod.rs

use crate::*;

use unwrap::unwrap;

/// return the position after the delimiter or None
/// Does NOT mutate the pos_cursor, because that is for a higher level logic to decide.
pub fn find_pos_after_delimiter(source_str: &str, pos_cursor: usize, delimiter: &str) -> Option<usize> {
    //
    if let Some(pos) = find_from(source_str, pos_cursor, delimiter) {
        let pos = pos + delimiter.len();
        return Some(pos);
    }
    // return
    None
}

/// return the position before the delimiter or None
/// Does NOT mutate the pos_cursor, because that is for a higher level logic to decide.
pub fn find_pos_before_delimiter(source_str: &str, pos_cursor: usize, delimiter: &str) -> Option<usize> {
    if let Some(pos) = find_from(source_str, pos_cursor, delimiter) {
        return Some(pos);
    }
    // return
    None
}

/// find and return the range of the first occurrence between start and end delimiters
/// Success: mutates also the cursor position, so the next find will continue from there
/// Fail: return None if not found and don't mutate pos_cursor
/// I use type Range to avoid references &str and lifetimes. But the programmer can make
/// the error to apply the range to the wrong vector.
pub fn find_range_between_delimiters(source_str: &str, pos_cursor: &mut usize, start_delimiter: &str, end_delimiter: &str) -> Option<std::ops::Range<usize>> {
    if let Some(pos_start) = find_pos_after_delimiter(source_str, *pos_cursor, start_delimiter) {
        // dbg!(&pos_start);
        if let Some(pos_end) = find_pos_before_delimiter(source_str, pos_start, end_delimiter) {
            // dbg!(&pos_end);
            *pos_cursor = pos_end + end_delimiter.len();
            return Some(pos_start..pos_end);
        }
    }
    // return
    None
}

#[allow(clippy::integer_arithmetic)]
/// find str from pos_cursor low level
pub fn find_from(source_str: &str, pos_cursor: usize, find_str: &str) -> Option<usize> {
    let slice01 = source_str.get(pos_cursor..).unwrap();
    let option_pos_found = slice01.find(find_str);
    if let Some(pos_found) = option_pos_found {
        // return Option with usize
        Some(pos_cursor + pos_found)
    } else {
        // return
        None
    }
}
use std::{fs, io, path::Path};
/// traverse dir (sub-dir) with exclude dir
/// the find_file and the exclude dir strings must start with /
pub fn traverse_dir_with_exclude_dir(dir: &Path, find_file: &str, exclude_dirs: &Vec<String>) -> io::Result<Vec<String>> {
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
    let mut number = s!();
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

#[allow(unused_imports)]
use ansi_term::Colour::{Blue, Green, Red, Yellow};
use chrono::prelude::*;

pub fn ns_start(text: &str) -> i64 {
    let now = Utc::now();
    if !text.is_empty() {
        eprintln!("{}: {}", Green.paint(&Local::now().format("%Y-%m-%d %H:%M:%S").to_string()), Green.paint(text));
    }
    now.timestamp_nanos()
}

pub fn ns_elapsed(ns_start: i64) -> i64 {
    let now_ns = Utc::now().timestamp_nanos();
    let duration_ns = now_ns - ns_start;
    // return
    duration_ns
}

pub fn ns_print(name: &str, ns_start: i64) -> i64 {
    let duration_ns = ns_elapsed(ns_start);
    if !name.is_empty() {
        let duration_ns = duration_ns.to_string();
        let duration_ns = if duration_ns.len() == 4 {
            format!("          {}.{}", &duration_ns[0..1], &duration_ns[1..4])
        } else if duration_ns.len() == 5 {
            format!("         {}.{}", &duration_ns[0..2], &duration_ns[2..5])
        } else if duration_ns.len() == 6 {
            format!("        {}.{}", &duration_ns[0..3], &duration_ns[3..6])
        } else if duration_ns.len() == 7 {
            format!("      {}.{}.{}", &duration_ns[0..1], &duration_ns[1..4], &duration_ns[4..7])
        } else if duration_ns.len() == 8 {
            format!("     {}.{}.{}", &duration_ns[0..2], &duration_ns[2..5], &duration_ns[5..8])
        } else if duration_ns.len() == 9 {
            format!("    {}.{}.{}", &duration_ns[0..3], &duration_ns[3..6], &duration_ns[6..9])
        } else if duration_ns.len() == 10 {
            format!("  {}.{}.{}.{}", &duration_ns[0..1], &duration_ns[1..4], &duration_ns[4..7], &duration_ns[7..10])
        } else if duration_ns.len() == 11 {
            format!(" {}.{}.{}.{}", &duration_ns[0..2], &duration_ns[2..5], &duration_ns[5..8], &duration_ns[8..11])
        } else if duration_ns.len() == 12 {
            format!("{}.{}.{}.{}", &duration_ns[0..3], &duration_ns[3..6], &duration_ns[6..9], &duration_ns[9..12])
        } else {
            duration_ns
        };
        eprintln!("{} ns : {}", duration_ns, name);
    }
    // return new now_ns
    Utc::now().timestamp_nanos()
}

pub fn reviewer_name_from_url(url: &str) -> String {
    let reviewer_name = url
        .replace("https://github.com/", "")
        .replace("https://gitlab.com/", "")
        .replace("/crev-proofs", "")
        .replace("https://", "");
    // return
    reviewer_name
}

/// version for sorting
pub fn version_for_sorting(version: &str, reviewer_name: &str) -> String {
    let (major, minor, patch) = parse_semver(version);
    let version_for_sorting = format!("{:09}.{:09}.{:09}-{}", major, minor, patch, reviewer_name,);
    // return
    version_for_sorting
}
