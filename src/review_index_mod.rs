//! review_index_mod
/// iterating in the original file format is not performant
/// it is better to read the files once and make an index of all
/// and then mostly use this index from memory.
/// this index is created every time the web app is initialized
/// or manually when the new and updated files are fetched
//use crate::duration_mod;
use crate::review_mod::*;
use crate::utils_mod::*;

//use chrono::Local;
use std::fs;
use unwrap::unwrap;

/// one item in the index
#[derive(Clone, Debug)]
pub struct ReviewIndexItem {
    pub crate_name: String,
    pub version: String,
    pub version_for_sorting: String,
    pub author: String,
    pub author_url: String,
    pub file_path: String,
    pub rating_strong: usize,
    pub rating_positive: usize,
    pub rating_neutral: usize,
    pub rating_negative: usize,
    pub rating_none: usize,
    pub alternatives: usize,
    pub issues: usize,
    pub advisories: usize,
}
/// only one field with a generic name vec
pub struct ReviewIndex {
    pub vec: Vec<ReviewIndexItem>,
}

impl ReviewIndex {
    /// prepares the data
    /// todo: this could be cached
    pub fn new() -> ReviewIndex {
        let mut review_index = ReviewIndex { vec: vec![] };
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
            // for filename_result in unwrap!(glob("/reviews/*.crev")) {
            // read crev file
            let crev_text = unwrap!(fs::read_to_string(filename_crev));
            for part1 in crev_text.split("----- END CREV PROOF -----") {
                let start_delimiter = "----- BEGIN CREV PROOF -----";
                if let Some(start_pos) = part1.find(start_delimiter) {
                    let start_pos = start_pos + start_delimiter.len() + 1;
                    if let Some(end_pos) = part1.find("----- SIGN CREV PROOF -----") {
                        let proof_string = &part1[start_pos..end_pos];
                        Self::push_review_index(proof_string, &mut review_index, filename_crev);
                    }
                }
            }
            // older review has different delimiter. Everything else is the same.
            for part1 in crev_text.split("-----END CREV PACKAGE REVIEW-----") {
                let start_delimiter = "-----BEGIN CREV PACKAGE REVIEW-----";
                if let Some(start_pos) = part1.find(start_delimiter) {
                    let start_pos = start_pos + start_delimiter.len() + 1;
                    if let Some(end_pos) =
                        part1.find("-----BEGIN CREV PACKAGE REVIEW SIGNATURE-----")
                    {
                        let proof_string = &part1[start_pos..end_pos];
                        Self::push_review_index(proof_string, &mut review_index, filename_crev);
                    }
                }
            }
        }
        //return
        review_index
    }

    /// mutates review_index
    fn push_review_index(proof_string: &str, review_index: &mut ReviewIndex, file_path: &str) {
        // deserialize one proof
        let proof: crate::review_mod::Review = unwrap!(serde_yaml::from_str(proof_string));
        // use only some of the data for the index
        let review_index_item = ReviewIndexItem {
            crate_name: proof.package.name.to_string(),
            version: proof.package.version.to_string(),
            version_for_sorting: proof.version_for_sorting(),
            author: proof.get_author(),
            author_url: proof.from.url.to_string(),
            file_path: file_path.to_string(),
            rating_strong: conditional_usize(proof.get_rating() == Rating::Strong, 1, 0),
            rating_positive: conditional_usize(proof.get_rating() == Rating::Positive, 1, 0),
            rating_neutral: conditional_usize(proof.get_rating() == Rating::Neutral, 1, 0),
            rating_negative: conditional_usize(proof.get_rating() == Rating::Negative, 1, 0),
            rating_none: conditional_usize(proof.get_rating() == Rating::None, 1, 0),

            alternatives: {
                if let Some(alternatives) = proof.alternatives {
                    alternatives.len()
                } else {
                    0
                }
            },
            issues: {
                if let Some(issues) = proof.issues {
                    issues.len()
                } else {
                    0
                }
            },
            advisories: {
                if let Some(advisories) = proof.advisories {
                    advisories.len()
                } else {
                    0
                }
            },
        };
        review_index.vec.push(review_index_item);
    }
}
