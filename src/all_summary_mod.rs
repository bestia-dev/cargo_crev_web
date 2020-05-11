//! all_summary_mod

// region: use
use crate::proof_mod::*;
use crate::version_summary_mod::VersionSummary;
use crate::*;
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

impl html_template_mod::HtmlTemplating for AllSummaries {
    /// html_templating boolean id the next node is rendered or not
    fn call_fn_boolean(&self, fn_name: &str) -> bool {
        // println!("{}",&format!("call_fn_boolean: {}", &fn_name));
        match fn_name {
            _ => {
                let x = format!("Unrecognized all_summary_mod call_fn_boolean: \"{}\"", fn_name);
                println!("Error: {}", &x);
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
    fn call_fn_string(&self, fn_name: &str) -> String {
        // println!("{}",&format!("call_fn_string: {}", &fn_name));
        use html_template_mod::to_string_zero_to_empty;
        match fn_name {
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
                let x = format!("Unrecognized all_summary_mod call_fn_string: \"{}\"", fn_name);
                println!("Error: {}", &x);
                x
            }
        }
    }
    /*
    /// return a closure for the listener.
    #[allow(clippy::too_many_lines, clippy::type_complexity)]
    fn call_fn_listener(
        &self,
        fn_name: String,
    ) -> Box<dyn Fn(&mut dyn RootRender, VdomWeak, Event) + 'static> {
        Box::new(move |root, vdom, event| {
            let fn_name = fn_name.clone();
            let fn_name = fn_name.as_str();
            let rrc = root.unwrap_mut::<RootRenderingComponent>();
            //println!("{}",&format!("call_fn_listener: {}", &fn_name));
            match fn_name {

                "open_youtube" => {
                    // randomly choose a link from rrc.videos
                    let num = websysmod::get_random(0, rrc.game_data.videos.len());
                    #[allow(clippy::indexing_slicing)]
                    // cannot panic:the num is 0..video.len
                    websysmod::open_new_tab(&format!(
                        "https://www.youtube.com/watch?v={}",
                        rrc.game_data.videos[num]
                    ));
                }
                _ => {
                    let x = format!("Unrecognized all_summary_mod call_fn_listener: \"{}\"", fn_name);
                    println!("Error: {}",&x);
                }
            }
        })
    }
    */
    /// html_templating functions that return a Node
    #[allow(clippy::needless_return)]
    fn call_fn_node(&self, fn_name: &str) -> html_template_mod::Node {
        // println!("{}",&format!("call_fn_node: {}", &fn_name));
        match fn_name {
            _ => {
                // so much boilerplate
                let node = html_template_mod::Node {
                    node_enum: html_template_mod::NodeEnum::Element(
                        html_template_mod::ElementNode {
                            tag_name: "h2".to_string(),
                            attributes: vec![],
                            children: vec![html_template_mod::Node {
                                node_enum: html_template_mod::NodeEnum::Text(format!(
                                    "Error: Unrecognized all_summary_mod call_fn_node: \"{}\"",
                                    fn_name
                                )),
                            }],
                            namespace: None,
                        },
                    ),
                };
                return node;
            }
        }
    }

    /// html_templating functions that return a vector of Nodes
    #[allow(clippy::needless_return)]
    fn call_fn_vec_nodes(&self, fn_name: &str) -> Vec<html_template_mod::Node> {
        // println!("{}",&format!("call_fn_node: {}", &fn_name));
        match fn_name {
            _ => {
                // so much boilerplate
                let node = html_template_mod::Node {
                    node_enum: html_template_mod::NodeEnum::Element(
                        html_template_mod::ElementNode {
                            tag_name: "h2".to_string(),
                            attributes: vec![],
                            children: vec![html_template_mod::Node {
                                node_enum: html_template_mod::NodeEnum::Text(format!(
                                    "Error: Unrecognized all_summary_mod call_fn_vec_nodes: \"{}\"",
                                    fn_name
                                )),
                            }],
                            namespace: None,
                        },
                    ),
                };
                return vec![node];
            }
        }
    }
}
