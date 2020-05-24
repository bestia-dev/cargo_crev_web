//! review_index_mod

use crate::review_mod::*;
use crate::utils_mod::*;
/// iterating in the original file format is not performant
/// it is better to read the files once and make an index of all
/// and then mostly use this index from memory.
/// this index is created every time the web app is initialized
/// or manually when the new and updated files are fetched
use crate::*;
use std::fs;
use unwrap::unwrap;

/// one item in the index
#[derive(Clone, Debug)]
pub struct ReviewIndexItem {
    pub crate_name: String,
    pub version: String,
    pub version_for_sorting: String,
    pub author_name: String,
    pub author_url: String,
    pub author_id: String,
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
    pub fn new() -> Self {
        let ns_start = ns_start("ReviewIndex");
        let mut review_index = ReviewIndex { vec: vec![] };
        // original cache crev folder: /home/luciano/.cache/crev/remotes
        // on the google vm bestia02: /home/luciano_bestia/.cache/crev/remotes
        // local webfolder example "../sample_data/cache/crev/remotes"
        let path = unwrap!(dirs::home_dir());
        let path = path.join(".cache/crev/remotes");
        // eprintln!("path: {}", path.display());
        // let mut count_files = 0;
        for filename_crev in &unwrap!(traverse_dir_with_exclude_dir(
            &path,
            "/*.crev",
            // avoid big folders and other folders with *.crev
            &vec![s!("/.git"), s!("/trust")]
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
                        let review_string = &part1[start_pos..end_pos];
                        Self::push_review_index(review_string, &mut review_index, filename_crev);
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
                        let review_string = &part1[start_pos..end_pos];
                        Self::push_review_index(review_string, &mut review_index, filename_crev);
                    }
                }
            }
        }
        // sort by file_path
        review_index
            .vec
            .sort_by(|a, b| b.file_path.cmp(&a.file_path));
        ns_print("ReviewIndex.new()", ns_start);
        //return
        review_index
    }

    /// mutates review_index
    fn push_review_index(review_string: &str, review_index: &mut ReviewIndex, file_path: &str) {
        // deserialize one review
        let review: crate::review_mod::Review = unwrap!(serde_yaml::from_str(review_string));
        // use only some of the data for the index
        let review_index_item = ReviewIndexItem {
            crate_name: s!(&review.package.name),
            version: s!(&review.package.version),
            version_for_sorting: review.version_for_sorting(),
            author_name: review.get_author_name(),
            author_url: s!(&review.from.url),
            author_id: s!(&review.from.id),
            file_path: file_path.to_string(),
            rating_strong: conditional_usize(review.get_rating() == Rating::Strong, 1, 0),
            rating_positive: conditional_usize(review.get_rating() == Rating::Positive, 1, 0),
            rating_neutral: conditional_usize(review.get_rating() == Rating::Neutral, 1, 0),
            rating_negative: conditional_usize(review.get_rating() == Rating::Negative, 1, 0),
            rating_none: conditional_usize(review.get_rating() == Rating::None, 1, 0),

            alternatives: {
                if let Some(alternatives) = review.alternatives {
                    alternatives.len()
                } else {
                    0
                }
            },
            issues: {
                if let Some(issues) = review.issues {
                    issues.len()
                } else {
                    0
                }
            },
            advisories: {
                if let Some(advisories) = review.advisories {
                    advisories.len()
                } else {
                    0
                }
            },
        };
        review_index.vec.push(review_index_item);
    }
}
