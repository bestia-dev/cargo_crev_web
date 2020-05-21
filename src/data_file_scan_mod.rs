//! data_file_scan_mod

use crate::review_mod::*;
use crate::utils_mod::*;

use std::fs;
use unwrap::unwrap;

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
fn get_vec_of_review_by_review_pk(path_name: &str, review_pks: Vec<ReviewPk>) -> Vec<Review> {
    // first fill a vector with reviews, because I need to filter and sort them
    let mut reviews = vec![];
    for review_pk in &review_pks {
        // original cache crev folder: /home/luciano/.cache/crev/remotes
        // on the google vm bestia02: /home/luciano_bestia/.cache/crev/remotes
        // local webfolder example "../sample_data/cache/crev/remotes"
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
                    let review_string = &part1[start_pos..end_pos];
                    push_review(review_string, &mut reviews, &review_pk);
                }
            }
        }
        // older review has different delimiter. Everything else is the same.
        for part1 in crev_text.split("-----END CREV PACKAGE REVIEW-----") {
            let start_delimiter = "-----BEGIN CREV PACKAGE REVIEW-----";
            if let Some(start_pos) = part1.find(start_delimiter) {
                let start_pos = start_pos + start_delimiter.len() + 1;
                if let Some(end_pos) = part1.find("-----BEGIN CREV PACKAGE REVIEW SIGNATURE-----") {
                    let review_string = &part1[start_pos..end_pos];
                    push_review(review_string, &mut reviews, &review_pk);
                }
            }
        }
    }
    // return
    reviews
}

fn push_review(review_string: &str, reviews: &mut Vec<Review>, review_pk: &ReviewPk) {
    let mut review: Review = unwrap!(serde_yaml::from_str(review_string));
    // filter: only the one equal to review_pk
    if review.package.name == review_pk.crate_name
        && review.from.url == review_pk.author_url
        && review.package.version == review_pk.version
    {
        // reviews without review are not important
        // version for sorting
        let (major, minor, patch) = parse_semver(&review.package.version);
        review.package.version_for_sorting = Some(review.version_for_sorting());
        Some(format!(
            "{:09}.{:09}.{:09}-{}",
            major,
            minor,
            patch,
            review.get_author()
        ));
        reviews.push(review);
    }
}
