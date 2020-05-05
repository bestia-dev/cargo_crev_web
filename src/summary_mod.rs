//use glob::glob;
// use crate::html_template_mod::*;
use crate::proof_mod::*;
//use serde_derive::{Deserialize, Serialize};
//use std::fs;
use unwrap::unwrap;

#[derive(Clone, Debug)]
pub struct AllSummaries {
    crate_name: String,
    crate_summary: VersionSummary,
    version_summaries: Vec<VersionSummary>,
}
#[derive(Clone, Debug)]
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

pub fn proof_summary(crate_name: &str, proofs: &mut Vec<Proof>) -> AllSummaries {
    // the first version empty_string is for "all_versions" or crate_summary
    let mut all_summaries = AllSummaries {
        crate_name: crate_name.to_string(),
        version_summaries: vec![],
        crate_summary: VersionSummary {
            version: String::new(),
            version_for_sorting: String::new(),
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
        },
    };

    for proof in proofs {
        //find version in vector or create new
        let mut option_version: Option<&mut VersionSummary> = None;
        for version_summary in &mut all_summaries.version_summaries {
            if version_summary.version == proof.package.version {
                option_version = Some(version_summary);
                break;
            }
        }
        if option_version.is_none() {
            //new element
            let mut version_to_push = VersionSummary::new();
            version_to_push.version = proof.package.version.to_string();
            version_to_push.version_for_sorting =
                unwrap!(proof.package.version_for_sorting.clone()).to_string();
            all_summaries.version_summaries.push(version_to_push);
            option_version = Some(unwrap!(all_summaries.version_summaries.last_mut()));
        }
        // Here Option is not needed any more.
        let mut version_summary = unwrap!(option_version);
        all_summaries.crate_summary.review_number += 1;
        version_summary.review_number += 1;

        if let Some(review) = &proof.review {
            if review.rating == Rating::Strong {
                all_summaries.crate_summary.rating_strong += 1;
                version_summary.rating_strong += 1;
            }
        }
    }
    //return
    all_summaries
}
