//! crate_version_summary_mod

// region: use
use crate::review_mod::*;
use crate::version_summary_mod::VersionSummary;
use crate::*;
use html_template_mod::*;
//use serde_derive::{Deserialize, Serialize};
//use std::fs;
use unwrap::unwrap;
// endregion: use

#[derive(Clone, Debug)]
pub struct CrateVersionSummary {
    pub crate_name: String,
    pub crate_summary: VersionSummary,
    pub version_summaries: Vec<VersionSummary>,
}
impl CrateVersionSummary {
    pub fn new(crate_name: &str, reviews: &Vec<Review>) -> CrateVersionSummary {
        // the first version empty_string is for "all_versions" or crate_summary
        let mut crate_version_summary = CrateVersionSummary {
            crate_name: crate_name.to_string(),
            version_summaries: vec![],
            crate_summary: VersionSummary {
                crate_name: crate_name.to_string(),
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

        for proof in reviews {
            // find version in vector or create new
            let mut option_version: Option<&mut VersionSummary> = None;
            for version_summary in &mut crate_version_summary.version_summaries {
                if version_summary.version == proof.package.version {
                    option_version = Some(version_summary);
                    break;
                }
            }
            if option_version.is_none() {
                // new element
                let mut version_to_push = VersionSummary::new();
                version_to_push.crate_name = crate_name.to_string();
                version_to_push.version = proof.package.version.to_string();
                version_to_push.version_for_sorting =
                    unwrap!(proof.package.version_for_sorting.clone()).to_string();
                crate_version_summary.version_summaries.push(version_to_push);
                option_version = Some(unwrap!(crate_version_summary.version_summaries.last_mut()));
            }
            // Here Option is not needed any more.
            let mut version_summary = unwrap!(option_version);
            crate_version_summary.crate_summary.review_number += 1;
            version_summary.review_number += 1;

            if let Some(review) = &proof.review {
                if review.rating == Rating::Strong {
                    crate_version_summary.crate_summary.rating_strong += 1;
                    version_summary.rating_strong += 1;
                }
                if review.rating == Rating::Positive {
                    crate_version_summary.crate_summary.rating_positive += 1;
                    version_summary.rating_positive += 1;
                }
                if review.rating == Rating::Neutral {
                    crate_version_summary.crate_summary.rating_neutral += 1;
                    version_summary.rating_neutral += 1;
                }
                if review.rating == Rating::Negative {
                    crate_version_summary.crate_summary.rating_negative += 1;
                    version_summary.rating_negative += 1;
                }
                if review.thoroughness == Level::High {
                    crate_version_summary.crate_summary.thoroughness += 2;
                    version_summary.thoroughness += 2;
                }
                if review.thoroughness == Level::Medium {
                    crate_version_summary.crate_summary.thoroughness += 1;
                    version_summary.thoroughness += 1;
                }
                if review.understanding == Level::High {
                    crate_version_summary.crate_summary.understanding += 2;
                    version_summary.understanding += 2;
                }
                if review.understanding == Level::Medium {
                    crate_version_summary.crate_summary.understanding += 1;
                    version_summary.understanding += 1;
                }
            }
            if let Some(_alternative) = &proof.alternatives {
                crate_version_summary.crate_summary.alternatives += 1;
                version_summary.alternatives += 1;
            }
            if let Some(_issue) = &proof.issues {
                crate_version_summary.crate_summary.issues += 1;
                version_summary.issues += 1;
            }
            if let Some(_advisory) = &proof.advisories {
                crate_version_summary.crate_summary.advisories += 1;
                version_summary.advisories += 1;
            }
            if let Some(_advisory) = &proof.advisory {
                crate_version_summary.crate_summary.advisories += 1;
                version_summary.advisories += 1;
            }
        }
        // return
        crate_version_summary
    }
}

impl HtmlTemplatingRender for CrateVersionSummary {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        //return
        "CrateVersionSummary".to_string()
    }
    /// CrateVersionSummary is never a full html file. It is always a sub-template.
    fn render_html_file(&self, _templates_folder_name: &str) -> String {
        //return
        String::new()
    }
    // html_templating boolean id the next node is rendered or not
    fn retain_next_node(&self, placeholder: &str) -> bool {
        // eprintln!("{}",&format!("retain_next_node: {}", &placeholder));
        match placeholder {
            _ => retain_next_node_match_else(&self.data_model_name(), placeholder),
        }
    }

    // html_templating functions that return a String
    #[allow(
        clippy::needless_return,
        clippy::integer_arithmetic,
        clippy::indexing_slicing
    )]
    fn replace_with_string(&self, placeholder: &str, _cursor_pos: usize) -> String {
        // eprintln!("{}",&format!("replace_with_string: {}", &placeholder));
        match placeholder {
            "t_crate_name" => self.crate_name.to_string(),
            "t_crates_io_link" => format!("https://crates.io/crates/{}", self.crate_name),
            "t_lib_rs_link" => format!("https://lib.rs/crates/{}", self.crate_name),
            "t_crate_review_number" => to_string_zero_to_empty(self.crate_summary.review_number),
            "t_crate_rating_strong" => to_string_zero_to_empty(self.crate_summary.rating_strong),
            "t_crate_rating_positive" => {
                to_string_zero_to_empty(self.crate_summary.rating_positive)
            }
            "t_crate_rating_neutral" => to_string_zero_to_empty(self.crate_summary.rating_neutral),
            "t_crate_rating_negative" => {
                to_string_zero_to_empty(self.crate_summary.rating_negative)
            }
            "t_crate_alternatives" => to_string_zero_to_empty(self.crate_summary.alternatives),
            "t_crate_issues" => to_string_zero_to_empty(self.crate_summary.issues),
            "t_crate_advisories" => to_string_zero_to_empty(self.crate_summary.advisories),
            "t_crate_thoroughness" => to_string_zero_to_empty(self.crate_summary.thoroughness),
            "t_crate_understanding" => to_string_zero_to_empty(self.crate_summary.understanding),

            "t_filter_crate" => format!("/cargo_crev_web/query/{}", self.crate_name),
            "t_filter_strong" => format!("/cargo_crev_web/query/{}/crate/S", self.crate_name),
            "t_filter_positive" => format!("/cargo_crev_web/query/{}/crate/P", self.crate_name),
            "t_filter_neutral" => format!("/cargo_crev_web/query/{}/crate/E", self.crate_name),
            "t_filter_negative" => format!("/cargo_crev_web/query/{}/crate/N", self.crate_name),
            "t_filter_alternatives" => format!("/cargo_crev_web/query/{}/crate/v", self.crate_name),
            "t_filter_issues" => format!("/cargo_crev_web/query/{}/crate/i", self.crate_name),
            "t_filter_advisories" => format!("/cargo_crev_web/query/{}/crate/a", self.crate_name),
            _ => replace_with_string_match_else(&self.data_model_name(), placeholder),
        }
    }
    // html_templating functions that return a vector of Nodes
    #[allow(clippy::needless_return)]
    fn replace_with_nodes(&self, placeholder: &str) -> Vec<Node> {
        // eprintln!("{}",&format!("replace_with_nodes: {}", &placeholder));
        match placeholder {
            _ => replace_with_nodes_match_else(&self.data_model_name(), placeholder),
        }
    }
    // html_templating for sub-template
    #[allow(clippy::needless_return)]
    fn render_sub_template(
        &self,
        template_name: &str,
        sub_templates: &Vec<SubTemplate>,
    ) -> Vec<Node> {
        // eprintln!("{}",&format!("&sub_templates.len(): {}", &sub_templates.len()));

        match template_name {
            "template_summary_version" => {
                let sub_template = unwrap!(sub_templates
                    .iter()
                    .find(|&template| template.name == template_name));
                let mut nodes = vec![];
                for version_summary in &self.version_summaries {
                    let vec_node = unwrap!(version_summary.render_template_raw_to_nodes(
                        &sub_template.template,
                        HtmlOrSvg::Html,
                        0
                    ));
                    nodes.extend_from_slice(&vec_node);
                }
                // return
                nodes
            }
            _ => render_sub_template_match_else(&self.data_model_name(), template_name),
        }
    }
}