//! crev_query_mod

use crate::all_summary_mod::AllSummaries;
use crate::duration_mod;
use crate::html_template_mod::*;
use crate::proof_mod::*;
use crate::utils_mod::*;
use crate::*;
use chrono::Local;
use dirs;
use std::fs;
use unwrap::unwrap;

pub struct CrevQueryData {
    pub all_summaries: AllSummaries,
    pub proofs: Vec<Proof>,
}
/// crev query returns html
pub fn html_for_crev_query(
    templates_folder_name: &str,
    crate_name: &str,
    version: &str,
    kind: &str,
) -> String {
    let start = duration_mod::start_ns();
    eprintln!(
        "{}: crate_name: '{}', version '{}', kind '{}'",
        &Local::now().format("%Y-%m-%d %H:%M:%S"),
        Green.paint(crate_name),
        Green.paint(version),
        Green.paint(kind)
    );

    // first fill a vector with proofs, because I need to filter and sort them
    let mut proofs = proofs_crev_query(crate_name);
    let before_sum_and_filter =
        duration_mod::eprint_duration_ns("  after proofs_crev_query()", start);

    // the summary is always from all proofs. We must filter the proofs later.
    let all_summaries = all_summary_mod::calculate_all_summary_for_proofs(crate_name, &proofs);
    filter_proofs(&mut proofs, version, kind);
    // put all data needed for this template in one place
    let crev_query_data = CrevQueryData {
        proofs,
        all_summaries,
    };
    let before_render = duration_mod::eprint_duration_ns("  sum_and_filter", before_sum_and_filter);
    // now I have the data and I render the html from the template
    // the folders hierarchy for templates is similar like the routes
    // so to retain the same relative folders like css
    let template_file_name = format!("{}query/crev_query_template.html", templates_folder_name);
    let html = crev_query_data.render_from_file(&template_file_name);

    duration_mod::eprint_duration_ns("  render", before_render);
    duration_mod::eprint_duration_ns("html_for_crev_query()", start);
    // return
    html
}

/// crev query returns html
fn proofs_crev_query(crate_name: &str) -> Vec<Proof> {
    // first fill a vector with proofs, because I need to filter and sort them
    let mut proofs = vec![];
    // this part can be cached: last 10 queried crates

    // original cache crev folder: /home/luciano/.cache/crev/remotes
    // on the google vm bestia02: /home/luciano_bestia/.cache/crev/remotes
    // local webfolder example "crev/cache/crev/remotes"
    let path = unwrap!(dirs::home_dir());
    let path = path.join(".cache/crev/remotes");
    // eprintln!("path: {}", path.display());
    // let mut count_files = 0;
    for filename_crev in &unwrap!(traverse_dir_with_exclude_dir(
        &path,
        "/*.crev",
        // avoid big folders and other folders with *.crev
        &vec!["/.git".to_string(), "/trust".to_string()]
    )) {
        //count_files += 1;
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
                    push_proof(proof_string, &mut proofs, &crate_name);
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
                    push_proof(proof_string, &mut proofs, &crate_name);
                }
            }
        }
    }
    // eprintln!("files queried: {}", count_files);
    // sort first by version desc, but semver version and then by date
    proofs.sort_by(|a, b| {
        b.package
            .version_for_sorting
            .cmp(&a.package.version_for_sorting)
    });
    // return
    proofs
}

fn filter_proofs(proofs: &mut Vec<Proof>, version: &str, kind: &str) {
    if !version.is_empty() && version != "crate" {
        proofs.retain(|x| x.package.version == version);
    }
    if !kind.is_empty() && kind != "c" {
        // strong
        if kind == "S" {
            proofs.retain(|x| {
                x.review.is_some() && x.review.as_ref().unwrap().rating == Rating::Strong
            });
        } else if kind == "P" {
            proofs.retain(|x| {
                x.review.is_some() && x.review.as_ref().unwrap().rating == Rating::Positive
            });
        } else if kind == "E" {
            proofs.retain(|x| {
                x.review.is_some() && x.review.as_ref().unwrap().rating == Rating::Neutral
            });
        } else if kind == "N" {
            proofs.retain(|x| {
                x.review.is_some() && x.review.as_ref().unwrap().rating == Rating::Negative
            });
        } else if kind == "v" {
            proofs.retain(|x| x.alternatives.is_some());
        } else if kind == "i" {
            proofs.retain(|x| x.issues.is_some());
        } else if kind == "a" {
            proofs.retain(|x| x.advisories.is_some() || x.advisory.is_some());
        }
    }
}

fn push_proof(proof_string: &str, proofs: &mut Vec<Proof>, crate_name: &str) {
    let mut proof: Proof = unwrap!(serde_yaml::from_str(proof_string));
    // filter: only one crate_name
    if &proof.package.name == crate_name {
        // proofs without review are not important
        // version for sorting
        let (major, minor, patch) = parse_semver(&proof.package.version);
        proof.package.version_for_sorting = Some(proof.version_for_sorting());
        Some(format!(
            "{:09}.{:09}.{:09}-{}",
            major,
            minor,
            patch,
            proof.get_author()
        ));
        proofs.push(proof);
    }
}
