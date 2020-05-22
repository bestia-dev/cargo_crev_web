//! data_file_scan_mod

use crate::review_mod::*;
use crate::utils_mod::*;

use std::fs;
use unwrap::unwrap;

#[derive(Clone, Debug)]
pub struct ReviewPk {
    pub crate_name: String,
    pub author_id: String,
    pub version: String,
}
#[derive(Clone, Debug)]
pub struct OneFileReviewsPk {
    pub file_path: String,
    pub reviews_pk: Vec<ReviewPk>,
}
#[derive(Clone, Debug)]
pub struct ManyFileReviewsPk {
    pub vec: Vec<OneFileReviewsPk>,
}
#[derive(Clone, Debug)]
pub struct Stat {
    pub files_count: i64,
    pub all_reviews_count: i64,
    pub reviews_returned: i64,
    pub ns_serde_yaml: i64,
    pub ns_push_review: i64,
    pub ns_file_read: i64,
    pub ns_find_all_proof_new: i64,
    pub ns_find_all_proof_old: i64,
}

pub fn get_vec_of_review(review_pks: ManyFileReviewsPk) -> Vec<Review> {
    println!("review_pks: {:#?}", review_pks);
    let mut reviews = vec![];
    let mut stat = Stat {
        files_count: 0,
        all_reviews_count: 0,
        reviews_returned: 0,
        ns_serde_yaml: 0,
        ns_push_review: 0,
        ns_file_read: 0,
        ns_find_all_proof_new: 0,
        ns_find_all_proof_old: 0,
    };
    for one_file in &review_pks.vec {
        get_vec_of_review_by_review_pk(&mut reviews, &one_file, &mut stat);
    }

    println!("stat: {:#?}", stat);
    // return
    reviews
}
/// find one or more reviews from one file
/// the review PK crate_name, author_id, version
fn get_vec_of_review_by_review_pk(
    reviews: &mut Vec<Review>,
    one_file_review_pk: &OneFileReviewsPk,
    stat: &mut Stat,
) {
    let ns_start = ns_start("");
    let file_path = &one_file_review_pk.file_path;
    // first fill a vector with reviews, because I need to filter and sort them
    // original cache crev folder: /home/luciano/.cache/crev/remotes
    // on the google vm bestia02: /home/luciano_bestia/.cache/crev/remotes
    // local webfolder example "../sample_data/cache/crev/remotes"
    let path = unwrap!(dirs::home_dir());
    let path = path.join(".cache/crev/remotes");
    let path = path.join(file_path);
    // eprintln!("path: {}", path.display());
    // read crev file
    let crev_text = unwrap!(fs::read_to_string(path));
    let ns_new = ns_print("", ns_start);
    stat.ns_file_read += ns_elapsed(ns_start);
    stat.files_count += 1;

    for part1 in crev_text.split("----- END CREV PROOF -----") {
        let start_delimiter = "----- BEGIN CREV PROOF -----";
        if let Some(start_pos) = part1.find(start_delimiter) {
            let start_pos = start_pos + start_delimiter.len() + 1;
            if let Some(end_pos) = part1.find("----- SIGN CREV PROOF -----") {
                let review_string = &part1[start_pos..end_pos];
                push_review(review_string, reviews, &one_file_review_pk.reviews_pk, stat);
            }
        }
    }
    let ns_proof = ns_print("", ns_new);
    stat.ns_find_all_proof_new += ns_elapsed(ns_new);
    // older review has different delimiter. Everything else is the same.
    for part1 in crev_text.split("-----END CREV PACKAGE REVIEW-----") {
        let start_delimiter = "-----BEGIN CREV PACKAGE REVIEW-----";
        if let Some(start_pos) = part1.find(start_delimiter) {
            let start_pos = start_pos + start_delimiter.len() + 1;
            if let Some(end_pos) = part1.find("-----BEGIN CREV PACKAGE REVIEW SIGNATURE-----") {
                let review_string = &part1[start_pos..end_pos];
                push_review(review_string, reviews, &one_file_review_pk.reviews_pk, stat);
            }
        }
    }
    let _ns_review = ns_print("", ns_proof);
    stat.ns_find_all_proof_old += ns_elapsed(ns_proof);
}

fn push_review(
    review_string: &str,
    reviews: &mut Vec<Review>,
    review_pks: &Vec<ReviewPk>,
    stat: &mut Stat,
) {
    let ns_start = ns_start("");
    use serde_derive::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize, Clone)]
    struct ReviewShort {
        pub from: ReviewFrom,
        pub package: ReviewPackage,
    }
    let review_short: ReviewShort = unwrap!(serde_yaml::from_str(review_string));
    stat.all_reviews_count += 1;

    let ns_new = ns_print("", ns_start);
    stat.ns_serde_yaml += ns_elapsed(ns_start);

    // to do if yaml takes long. First yaml only 3 data.
    for review_pk in review_pks {
        // filter: only the one equal to review_pk
        if review_short.package.name == review_pk.crate_name
            && review_short.from.id == review_pk.author_id
            && review_short.package.version == review_pk.version
        {
            stat.reviews_returned += 1;
            // version for sorting
            let mut review: Review = unwrap!(serde_yaml::from_str(review_string));
            let (major, minor, patch) = parse_semver(&review.package.version);
            review.package.version_for_sorting = Some(format!(
                "{:09}.{:09}.{:09}-{}",
                major,
                minor,
                patch,
                review.get_author()
            ));
            reviews.push(review);
            break;
        }
    }
    ns_print("", ns_new);
    stat.ns_push_review += ns_elapsed(ns_new);
}
