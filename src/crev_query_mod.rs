//! crev_query_mod

use crate::all_summary_mod::AllSummaries;
use crate::html_template_mod::*;
use crate::proof_mod::*;
use crate::*;
use chrono::Local;
use dirs;
use std::{fs, io, path::Path};
use unwrap::unwrap;

pub struct CrevQueryData {
    pub all_summaries: AllSummaries,
    pub proofs: Vec<Proof>,
}
/// crev query returns html
pub fn html_for_crev_query(templates_folder_name: &str, crate_name: &str) -> String {
    eprintln!(
        "{}: crate_name: {}",
        &Local::now().format("%Y-%m-%d %H:%M:%S"),
        crate_name
    );

    //first fill a vector with proofs, because I need to filter and sort them
    let proofs = proofs_crev_query(crate_name);
    let all_summaries = all_summary_mod::calculate_all_summary_for_proofs(crate_name, &proofs);
    // put all data needed for this template in one place
    let crev_query_data = CrevQueryData {
        proofs,
        all_summaries,
    };

    // now I have the data and I render the html from the template
    // the folders hierarchy for templates is similar like the routes
    // so to retain the same relative folders like css
    let template_file_name = format!("{}query/crev_query_template.html", templates_folder_name);
    let html_template_raw = template_raw_from_file(&template_file_name);
    let nodes =
        unwrap!(crev_query_data.render_template_raw_to_nodes(&html_template_raw, HtmlOrSvg::Html,));
    //because this is the root template it must return one ElementNode
    let mut html = "".to_string();
    match &nodes[0].node_enum{
        NodeEnum::Element(temp_element_node)=>{
            html = unwrap!(element_node_to_string(temp_element_node));
        }
        _=>eprintln!("Error: crev_query_data.render_template_raw_to_nodes does not return one ElementNode.{}","")
    }
    //return
    html
}

/// crev query returns html
fn proofs_crev_query(crate_name: &str) -> Vec<Proof> {
    //first fill a vector with proofs, because I need to filter and sort them
    let mut proofs = vec![];
    // original cache crev folder: /home/luciano/.cache/crev/remotes
    // on the google vm bestia02: /home/luciano_bestia/.cache/crev/remotes
    // local webfolder example "crev/cache/crev/remotes"
    let path = unwrap!(dirs::home_dir());
    let path = path.join(".cache/crev/remotes");
    eprintln!("path: {}", path.display());
    let mut count_files = 0;
    for filename_crev in &unwrap!(traverse_dir_with_exclude_dir(
        &path,
        "/*.crev",
        // avoid big folders and other folders with *.crev
        &vec!["/.git".to_string(), "/trust".to_string()]
    )) {
        count_files += 1;
        // eprintln!("filename_crev: {}", filename_crev);
        // for filename_result in unwrap!(glob("/proofs/*.crev")) {
        // read crev file
        let crev_text = unwrap!(fs::read_to_string(filename_crev));
        for part1 in crev_text.split("----- END CREV PROOF -----") {
            let start_delimiter = "----- BEGIN CREV PROOF -----";
            if let Some(start_pos) = part1.find(start_delimiter) {
                let start_pos = start_pos + start_delimiter.len() + 1;
                if let Some(end_pos) = part1.find("----- SIGN CREV PROOF -----") {
                    let proof_string = &part1[start_pos..end_pos];
                    push_proof(proof_string, &mut proofs, &crate_name, filename_crev);
                }
            }
        }
        // older review has different delimiter. Everything else is the same.
        for part1 in crev_text.split("-----END CREV PACKAGE REVIEW-----") {
            let start_delimiter = "-----BEGIN CREV PACKAGE REVIEW-----";
            if let Some(start_pos) = part1.find(start_delimiter) {
                let start_pos = start_pos + start_delimiter.len() + 1;
                if let Some(end_pos) = part1.find("-----BEGIN CREV PACKAGE REVIEW SIGNATURE-----") {
                    let proof_string = &part1[start_pos..end_pos];
                    push_proof(proof_string, &mut proofs, &crate_name, filename_crev);
                }
            }
        }
    }
    eprintln!("files queried: {}", count_files);
    //sort first by version desc, but semver version and then by date
    proofs.sort_by(|a, b| {
        b.package
            .version_for_sorting
            .cmp(&a.package.version_for_sorting)
    });
    //return
    proofs
}

/// parse semver ex. 12.99.88alpha
fn parse_semver(text: &str) -> (usize, usize, usize) {
    let pos = 0;
    let (major, pos) = parse_next_number(&text, pos);
    //jump over dot
    let pos = pos + 1;
    let (minor, pos) = parse_next_number(&text, pos);
    //jump over dot
    let pos = pos + 1;
    let (patch, _pos) = parse_next_number(&text, pos);
    //return
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
    //return
    (number, pos)
}

/// traverse dir (sub-dir) with exclude dir
/// the find_file and the exclude dir strings must start with /
fn traverse_dir_with_exclude_dir(
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

fn push_proof(proof_string: &str, proofs: &mut Vec<Proof>, crate_name: &str, _filename_crev: &str) {
    let mut proof: Proof = unwrap!(serde_yaml::from_str(proof_string));
    //filter: only one crate_name
    if &proof.package.name == crate_name {
        // proofs without review are not important
        //version for sorting
        let (major, minor, patch) = parse_semver(&proof.package.version);
        proof.package.version_for_sorting = Some(format!(
            "{:09}.{:09}.{:09}-{}",
            major,
            minor,
            patch,
            get_author(&proof)
        ));
        proofs.push(proof);
    }
}
