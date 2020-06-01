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
    pub reviews_pk: Option<Vec<ReviewPk>>,
}
#[derive(Clone, Debug)]
pub struct ManyFileReviewsPk {
    pub vec: Vec<OneFileReviewsPk>,
}

pub fn get_vec_of_selected_reviews(review_pks: ManyFileReviewsPk) -> Vec<Review> {
    // dbg!(review_pks);
    let mut reviews = vec![];

    for one_file in &review_pks.vec {
        get_vec_from_one_file(&mut reviews, &one_file);
    }
    // return
    reviews
}


/// find one or more reviews from one file
/// the review PK crate_name, author_id, version
/// if None than push all reviews
fn get_vec_from_one_file(
    reviews: &mut Vec<Review>,
    one_file_review_pk: &OneFileReviewsPk,
) {
    let file_path = &one_file_review_pk.file_path;
    // first fill a vector with reviews, because I need to filter and sort them
    // original cache crev folder: /home/luciano/.cache/crev/remotes
    // on the google vm bestia02: /home/luciano_bestia/.cache/crev/remotes
    // local webfolder example "../sample_data/cache/crev/remotes"
    let path = unwrap!(dirs::home_dir());
    let path = path.join(".cache/crev/remotes");
    let path = path.join(file_path);
    //dbg!(&path);
    // read crev file
    let crev_text = unwrap!(fs::read_to_string(path));

    for part1 in crev_text.split("----- END CREV PROOF -----") {
        let start_delimiter = "----- BEGIN CREV PROOF -----";
        if let Some(start_pos) = part1.find(start_delimiter) {
            let start_pos = start_pos + start_delimiter.len() + 1;
            if let Some(end_pos) = part1.find("----- SIGN CREV PROOF -----") {
                let review_string = &part1[start_pos..end_pos];
                if let Some(reviews_pk) = &one_file_review_pk.reviews_pk{
                    // push this review if it is in selected reviews
                    push_review_if_selected(review_string, reviews, &reviews_pk);
                }else{
                    //push this reviews unconditionally
                    push_this_review(review_string, reviews);
                }
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
                if let Some(reviews_pk) = &one_file_review_pk.reviews_pk{
                    // push this review if it is in selected reviews
                    push_review_if_selected(review_string, reviews, &reviews_pk);
                }else{
                    //push this reviews unconditionally
                    push_this_review(review_string, reviews);
                }
            }
        }
    }
}

fn push_this_review(review_string: &str, reviews: &mut Vec<Review>) {
    // version for sorting
    let mut review: Review = unwrap!(serde_yaml::from_str(review_string));
    let (major, minor, patch) = parse_semver(&review.package.version);
    review.package.version_for_sorting = Some(format!(
        "{:09}.{:09}.{:09}-{}",
        major,
        minor,
        patch,
        review.get_author_name()
    ));
    reviews.push(review);
}

fn push_review_if_selected(review_string: &str, reviews: &mut Vec<Review>, review_pks: &Vec<ReviewPk>) {
    use serde_derive::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone)]
    struct ReviewPackageShort {
        pub name: String,
        pub version: String,
    }
    #[derive(Serialize, Deserialize, Clone)]
    struct ReviewFromShort {
        pub id: String,
    }
    #[derive(Serialize, Deserialize, Clone)]
    struct ReviewShort {
        pub from: ReviewFromShort,
        pub package: ReviewPackageShort,
    }
    // if yaml takes long. First yaml only 2 data.
    let review_short: ReviewShort = unwrap!(serde_yaml::from_str(review_string));

    for review_pk in review_pks {
        // push only if this review is in selected reviews pk
        if review_short.package.name == review_pk.crate_name
            && review_short.from.id == review_pk.author_id
            && review_short.package.version == review_pk.version
        {
            push_this_review(review_string,reviews);
            break;
        }
    }
    // dbg!(&reviews[0].comment);
}
