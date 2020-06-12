//! data_file_scan_mod

use crate::review_mod::*;
use crate::*;

use std::{fs, path::Path, path::PathBuf};
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

pub fn path_of_remotes_folder() -> PathBuf {
    // original cache crev folder: /home/luciano/.cache/crev/remotes
    // on the google vm bestia02: /home/luciano_bestia/.cache/crev/remotes
    // local webfolder example "../sample_data/cache/crev/remotes"
    let path = unwrap!(dirs::home_dir());
    let path = path.join(".cache/crev/remotes");
    // dbg!(path);
    //return
    path
}

/// all file names
pub fn crev_files(folder_path: &str) -> Vec<String> {
    // return
    unwrap!(traverse_dir_with_exclude_dir(
        Path::new(folder_path),
        "/*.crev",
        // avoid big folders and other folders with *.crev
        &vec![s!("/.git"), s!("/trust")]
    ))
}

pub struct ReviewsInOneFile {
    crev_text: String,
    pos_cursor: usize,
    is_old_format: bool,
}

impl ReviewsInOneFile {
    pub fn new(file_name: &str) -> Self {
        let path = path_of_remotes_folder().join(file_name);
        //dbg!(&path);
        // read crev file
        let crev_text = unwrap!(fs::read_to_string(path));
        //return
        ReviewsInOneFile {
            crev_text,
            pos_cursor: 0,
            is_old_format: false,
        }
    }
}

// todo: I want to return &str, but the lifetimes are super confusing
// or at least use only one buffer to avoid repeated allocation
impl Iterator for ReviewsInOneFile {
    type Item = String;
    /// returns the next review text
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_old_format == false {
            let range = find_range_between_delimiters(
                &self.crev_text,
                &mut self.pos_cursor,
                "----- BEGIN CREV PROOF -----",
                "----- SIGN CREV PROOF -----",
            );
            if let Some(range) = range {
                self.pos_cursor = range.end;
                return Some(self.crev_text[range].to_string());
            } else {
                self.is_old_format = true;
            }
        }
        // the second if must continue if the first if changes the bool
        if self.is_old_format == true {
            //the old format
            let range = find_range_between_delimiters(
                &self.crev_text,
                &mut self.pos_cursor,
                "-----BEGIN CREV PACKAGE REVIEW-----",
                "-----BEGIN CREV PACKAGE REVIEW SIGNATURE-----",
            );
            if let Some(range) = range {
                self.pos_cursor = range.end;
                return Some(self.crev_text[range].to_string());
            } else {
                // end of iterator
                return None;
            }
        }
        //return
        None
    }
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
fn get_vec_from_one_file(reviews: &mut Vec<Review>, one_file_review_pk: &OneFileReviewsPk) {
    let file_path = &one_file_review_pk.file_path;
    // first fill a vector with reviews, because I need to filter and sort them
    let path = path_of_remotes_folder().join(file_path);
    let path = path.to_string_lossy();
    //dbg!(&path);
    // read crev file
    // iterator for reviews return &str
    let reviews_in_one_file = ReviewsInOneFile::new(&path);
    for review_string in reviews_in_one_file {
        if let Some(reviews_pk) = &one_file_review_pk.reviews_pk {
            // push this review if it is in selected reviews
            push_review_if_selected(&review_string, reviews, &reviews_pk);
        } else {
            //push this reviews unconditionally
            push_this_review(&review_string, reviews);
        }
    }
}

fn push_this_review(review_string: &str, reviews: &mut Vec<Review>) {
    // version for sorting
    let mut review: Review = unwrap!(serde_yaml::from_str(review_string));
    review.package.version_for_sorting = Some(version_for_sorting(
        &review.package.version,
        &review.get_author_name(),
    ));
    reviews.push(review);
}

fn push_review_if_selected(
    review_string: &str,
    reviews: &mut Vec<Review>,
    review_pks: &Vec<ReviewPk>,
) {
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
            push_this_review(review_string, reviews);
            break;
        }
    }
    // dbg!(&reviews[0].comment);
}
