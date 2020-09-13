//! review_index_mod

use crate::data_file_scan_mod::*;
use crate::review_mod::*;
use crate::*;

/// iterating in the original file format is not performant
/// it is better to read the files once and make an index of all
/// and then mostly use this index from memory.
/// this index is created every time the web app is initialized
/// or manually when the new and updated files are fetched
//use crate::*;
use unwrap::unwrap;

/// one item in the index
#[derive(Clone, Debug)]
pub struct ReviewIndexItem {
    pub crate_name: String,
    pub version: String,
    pub version_for_sorting: String,
    pub date: chrono::DateTime<chrono::Utc>,
    pub reviewer_name: String,
    pub reviewer_url: String,
    pub reviewer_id: String,
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

        let path_of_remotes_folder = path_of_remotes_folder();
        let path_of_remotes_folder = path_of_remotes_folder.to_string_lossy() + "/";
        //fill from all the files all the reviews
        for file_name in crev_files(&path_of_remotes_folder).iter() {
            // I don't want too long file names
            //dbg!(file_name);
            let file_name = file_name.trim_start_matches(&path_of_remotes_folder.to_string());
            //dbg!(file_name);
            // iterator for reviews return &str
            let reviews_in_one_file = ReviewsInOneFile::new(file_name);
            for review_string in reviews_in_one_file {
                // use only data for index
                Self::push_review_index(&review_string, &mut review_index, file_name);
            }
        }
        // sort by file_path
        review_index
            .vec
            .sort_by(|a, b| b.file_path.cmp(&a.file_path));
        ns_print("ReviewIndex.new()", ns_start);
        // return
        review_index
    }

    /// mutates review_index
    fn push_review_index(review_string: &str, review_index: &mut ReviewIndex, file_path: &str) {
        // deserialize one review
        //dbg!(review_string);
        let review: crate::review_mod::Review = unwrap!(serde_yaml::from_str(review_string));
        // use only some of the data for the index
        // convert to Utc for comparison
        let date: chrono::DateTime<chrono::Utc> =
            chrono::DateTime::from(unwrap!(chrono::DateTime::parse_from_rfc3339(&review.date)));
        let review_index_item = ReviewIndexItem {
            crate_name: s!(&review.package.name),
            version: s!(&review.package.version),
            version_for_sorting: review.version_for_sorting(),
            date: date,
            reviewer_name: review.get_reviewer_name(),
            reviewer_url: s!(&review.from.url),
            reviewer_id: s!(&review.from.id),
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
