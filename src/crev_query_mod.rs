//use glob::glob;
use crate::proof_html_template_impl_mod;
use crate::proof_mod::*;
use dirs;
//use serde_derive::{Deserialize, Serialize};
use std::{fs, io, path::Path};
use unwrap::unwrap;

/// crev query returns html
pub fn crev_query(crate_name: String) -> String {
    println!("crate_name: {}", crate_name);
    let mut html = String::with_capacity(4000);
    //first fill a vector with proofs, because I need to filter and sort them
    let mut proofs = vec![];
    // original cache crev folder: /home/luciano/.cache/crev/remotes
    // on the google vm bestia02: /home/luciano_bestia/.cache/crev/remotes
    // local webfolder example "crev/cache/crev/remotes"
    let path = unwrap!(dirs::home_dir());
    let path = path.join(".cache/crev/remotes");
    println!("path: {}", path.display());
    let mut count_files = 0;
    for filename_crev in &unwrap!(traverse_dir_with_exclude_dir(
        &path,
        "/*.crev",
        // avoid big folders and other folders with *.crev
        &vec!["/.git".to_string(), "/trust".to_string()]
    )) {
        count_files += 1;
        //println!("filename_crev: {}", filename_crev);
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
    println!("files queried: {}", count_files);
    //sort first by version desc, but semver version and then by date
    proofs.sort_by(|a, b| {
        b.package
            .version_for_sorting
            .cmp(&a.package.version_for_sorting)
    });
    //how to repeat same template ? Now is too late.
    // i should extract the sub-template before.
    // the subtemplates can be visible for graphic designer or invisible (not too usefull)

    for proof in &proofs {
        proof_html_template_impl_mod::push_review_to_html(&mut html, proof);
    }
    //println!("html: {}", &html);
    let html_file = unwrap!(fs::read_to_string("crev/template_without_body.html"));
    //println!("html_file: {}", html_file);
    let html_file = html_file.replace("<!-- content -->", &html);
    //return
    html_file
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
        proof.package.version_for_sorting = Some(format!("{:09}.{:09}.{:09}", major, minor, patch));
        proofs.push(proof);
    }
}

struct VersionSummary {
    version: String,
    version_for_sorting: String,
    review_number: usize,
    rating_strong: usize,
    rating_positive: usize,
    rating_neutral: usize,
    rating_negative: usize,
    alternatives: usize,
    issues: usize,
    advisories: usize,
    thoroughness: usize,
    understanding: usize,
}

impl VersionSummary {
    pub fn new() -> Self {
        VersionSummary {
            version: "".to_string(),
            version_for_sorting: "".to_string(),
            review_number: 0,
            rating_strong: 0,
            rating_positive: 0,
            rating_neutral: 0,
            rating_negative: 0,
            alternatives: 0,
            issues: 0,
            advisories: 0,
            thoroughness: 0,
            understanding: 0,
        }
    }
}

pub fn proof_summary(proofs: &mut Vec<Proof>) {
    // summary for all the version. Does it have any meaning?
    // The same author can repeat it
    // for every version. So if I group by author could be any good.
    // Then we have reviews for every version separate
    let mut review_number = 0;
    let mut rating_strong = 0;
    let mut rating_positive = 0;
    let mut rating_neutral = 0;
    let mut rating_negative = 0;
    let mut alternatives = 0;
    let mut issues = 0;
    let mut advisories = 0;
    let mut thoroughness = 0;
    let mut understanding = 0;

    let mut versions_summaries: Vec<VersionSummary> = Vec::new();

    for proof in proofs {
        //find version in vector or create new
        let mut version: Option<&mut VersionSummary> = None;
        for version_summary in &mut versions_summaries {
            if version_summary.version == proof.package.version {
                version = Some(version_summary);
                break;
            }
        }
        if version.is_none() {
            //new element
            let mut version_to_push = VersionSummary::new();
            version_to_push.version = proof.package.version.to_string();
            version_to_push.version_for_sorting =
                unwrap!(proof.package.version_for_sorting.clone()).to_string();
            versions_summaries.push(version_to_push);
            version = Some(unwrap!(versions_summaries.last_mut()));
        }
        // Here Option is not needed any more.
        let mut ver = unwrap!(version);
        review_number += 1;
        ver.review_number += 1;

        if let Some(review) = &proof.review {
            if review.rating == Rating::Strong {
                rating_strong += 1;
                ver.rating_strong += 1;
            }
        }
    }
}
