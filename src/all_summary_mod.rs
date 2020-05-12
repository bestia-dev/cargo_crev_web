//! all_summary_mod

// region: use
use crate::proof_mod::*;
use crate::version_summary_mod::VersionSummary;
use crate::*;
use html_template_mod::*;
//use serde_derive::{Deserialize, Serialize};
//use std::fs;
use unwrap::unwrap;
// endregion: use

#[derive(Clone, Debug)]
pub struct AllSummaries {
    pub crate_name: String,
    pub crate_summary: VersionSummary,
    pub version_summaries: Vec<VersionSummary>,
}

pub fn calculate_all_summary_for_proofs(crate_name: &str, proofs: &Vec<Proof>) -> AllSummaries {
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
            if review.rating == Rating::Positive {
                all_summaries.crate_summary.rating_positive += 1;
                version_summary.rating_positive += 1;
            }
            if review.rating == Rating::Neutral {
                all_summaries.crate_summary.rating_neutral += 1;
                version_summary.rating_neutral += 1;
            }
            if review.rating == Rating::Negative {
                all_summaries.crate_summary.rating_negative += 1;
                version_summary.rating_negative += 1;
            }
            if review.thoroughness == Level::High {
                all_summaries.crate_summary.thoroughness += 2;
                version_summary.thoroughness += 2;
            }
            if review.thoroughness == Level::Medium {
                all_summaries.crate_summary.thoroughness += 1;
                version_summary.thoroughness += 1;
            }
            if review.understanding == Level::High {
                all_summaries.crate_summary.understanding += 2;
                version_summary.understanding += 2;
            }
            if review.understanding == Level::Medium {
                all_summaries.crate_summary.understanding += 1;
                version_summary.understanding += 1;
            }
        }
        if let Some(_alternative) = &proof.alternatives {
            all_summaries.crate_summary.alternatives += 1;
            version_summary.alternatives += 1;
        }
        if let Some(_issue) = &proof.issues {
            all_summaries.crate_summary.issues += 1;
            version_summary.issues += 1;
        }
        if let Some(_advisory) = &proof.advisories {
            all_summaries.crate_summary.advisories += 1;
            version_summary.advisories += 1;
        }
        if let Some(_advisory) = &proof.advisory {
            all_summaries.crate_summary.advisories += 1;
            version_summary.advisories += 1;
        }
    }
    //return
    all_summaries
}

impl HtmlTemplating for AllSummaries {
    /// html_templating boolean id the next node is rendered or not
    fn call_fn_boolean(&self, placeholder: &str) -> bool {
        // println!("{}",&format!("call_fn_boolean: {}", &placeholder));
        match placeholder {
            _ => {
                println!(
                    "Error: Unrecognized all_summary_mod call_fn_boolean: \"{}\"",
                    placeholder
                );
                true
            }
        }
    }

    /// html_templating functions that return a String
    #[allow(
        clippy::needless_return,
        clippy::integer_arithmetic,
        clippy::indexing_slicing
    )]
    fn call_fn_string(&self, placeholder: &str) -> String {
        // println!("{}",&format!("call_fn_string: {}", &placeholder));
        match placeholder {
            "t_crate_name" => self.crate_name.to_string(),
            "t_crate_link" => format!("https://crates.io/crates/{}", self.crate_name),
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
            _ => {
                let err_msg = format!(
                    "Error: Unrecognized all_summary_mod call_fn_string: \"{}\"",
                    placeholder
                );
                println!("{}", &err_msg);
                err_msg
            }
        }
    }
    /// html_templating functions that return a vector of Nodes
    #[allow(clippy::needless_return)]
    fn call_fn_vec_nodes(&self, placeholder: &str) -> Vec<ElementNode> {
        // println!("{}",&format!("call_fn_vec_nodes: {}", &placeholder));
        match placeholder {
            _ => {
                // so much boilerplate
                let err_msg = format!(
                    "Error: Unrecognized all_summary_mod call_fn_vec_nodes: \"{}\"",
                    placeholder
                );
                eprintln!("{}", &err_msg);
                let node = ElementNode {
                    tag_name: "h2".to_string(),
                    attributes: vec![],
                    children: vec![Node {
                        node_enum: NodeEnum::Text(err_msg),
                    }],
                    namespace: None,
                };
                return vec![node];
            }
        }
    }
    /// html_templating for sub-template
    #[allow(clippy::needless_return)]
    fn render_sub_template(
        &self,
        template_name: &str,
        sub_templates: &Vec<SubTemplate>,
    ) -> Vec<ElementNode> {
        // println!("{}",&format!("render_sub_template: {}", &placeholder));
        match template_name {
            "template_summary_version" => {
                let sub_template = unwrap!(sub_templates
                    .iter()
                    .find(|&template| template.name == template_name));
                let mut nodes = vec![];
                for version_summary in &self.version_summaries {
                    //find the sub template name in templates
                    // here is always the root node <template>
                    // it needs to be removed
                    let template_node = unwrap!(version_summary
                        .extract_children_sub_templates_and_render_template_to_element_node(
                            &sub_template.template,
                            HtmlOrSvg::Html,
                        ));
                    nodes.push(template_node);
                }
                //return
                nodes
            }
            _ => {
                // so much boilerplate
                let err_msg = format!(
                    "Error: Unrecognized all_summary_mod render_sub_template: \"{}\"",
                    template_name
                );
                println!("{}", &err_msg);
                let node = ElementNode {
                    tag_name: "h2".to_string(),
                    attributes: vec![],
                    children: vec![Node {
                        node_enum: NodeEnum::Text(err_msg),
                    }],
                    namespace: None,
                };
                return vec![node];
            }
        }
    }
}
