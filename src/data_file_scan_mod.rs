//! data_file_scan_mod

use crate::proof_mod::*;
use crate::utils_mod::*;

use unwrap::unwrap;
use std::fs;

pub struct ReviewPk {
    pub crate_name: String,
    pub author_url: String,
    pub version: String,
}
pub struct OneFileReviewsPk {
    pub file_path: String,
    pub reviews_pk: Vec<ReviewPk>,
}

/// find one or more reviews from one file
/// the review PK crate_name, author_url, version
fn get_vector_of_proof_by_review_pk(path_name: &str, review_pks: Vec<ReviewPk>) -> Vec<Proof> {
    // first fill a vector with proofs, because I need to filter and sort them
    let mut proofs = vec![];
    for review_pk in &review_pks {
        // original cache crev folder: /home/luciano/.cache/crev/remotes
        // on the google vm bestia02: /home/luciano_bestia/.cache/crev/remotes
        // local webfolder example "crev/cache/crev/remotes"
        let path = unwrap!(dirs::home_dir());
        let path = path.join(".cache/crev/remotes");
        let path = path.join(path_name);
        // eprintln!("path: {}", path.display());
        // read crev file
        let crev_text = unwrap!(fs::read_to_string(path));
        for part1 in crev_text.split("----- END CREV PROOF -----") {
            let start_delimiter = "----- BEGIN CREV PROOF -----";
            if let Some(start_pos) = part1.find(start_delimiter) {
                let start_pos = start_pos + start_delimiter.len() + 1;
                if let Some(end_pos) = part1.find("----- SIGN CREV PROOF -----") {
                    let proof_string = &part1[start_pos..end_pos];
                    push_proof(proof_string, &mut proofs, &review_pk);
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
                    push_proof(proof_string, &mut proofs, &review_pk);
                }
            }
        }
    }
    // return
    proofs
}

fn push_proof(proof_string: &str, proofs: &mut Vec<Proof>, review_pk: &ReviewPk) {
    let mut proof: Proof = unwrap!(serde_yaml::from_str(proof_string));
    // filter: only the one equal to review_pk
    if proof.package.name == review_pk.crate_name
        && proof.from.url == review_pk.author_url
        && proof.package.version == review_pk.version
    {
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
