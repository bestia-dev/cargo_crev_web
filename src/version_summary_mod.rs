//! all_summary_mod

// region: use
use crate::*;
use html_template_mod::*;
//use serde_derive::{Deserialize, Serialize};
//use std::fs;
//use unwrap::unwrap;
// endregion: use

#[derive(Clone, Debug)]
pub struct VersionSummary {
    pub version: String,
    pub version_for_sorting: String,
    pub review_number: usize,
    pub rating_strong: usize,
    pub rating_positive: usize,
    pub rating_neutral: usize,
    pub rating_negative: usize,
    pub alternatives: usize,
    pub issues: usize,
    pub advisories: usize,
    pub thoroughness: usize,
    pub understanding: usize,
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

impl HtmlTemplating for VersionSummary {
    /// html_templating boolean id the next node is rendered or not
    fn call_fn_boolean(&self, placeholder: &str) -> bool {
        // eprintln!("{}",&format!("call_fn_boolean: {}", &placeholder));
        match placeholder {
            _ => {
                eprintln!(
                    "Error: Unrecognized version_summary_mod call_fn_boolean: \"{}\"",
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
        // eprintln!("{}",&format!("call_fn_string: {}", &placeholder));
        match placeholder {
            "t_version" => self.version.to_string(),
            "t_review_number" => to_string_zero_to_empty(self.review_number),
            "t_rating_strong" => to_string_zero_to_empty(self.rating_strong),
            "t_rating_positive" => to_string_zero_to_empty(self.rating_positive),
            "t_rating_neutral" => to_string_zero_to_empty(self.rating_neutral),
            "t_rating_negative" => to_string_zero_to_empty(self.rating_negative),
            "t_alternatives" => to_string_zero_to_empty(self.alternatives),
            "t_issues" => to_string_zero_to_empty(self.issues),
            "t_advisories" => to_string_zero_to_empty(self.advisories),
            "t_thoroughness" => to_string_zero_to_empty(self.thoroughness),
            "t_understanding" => to_string_zero_to_empty(self.understanding),
            _ => {
                let err_msg = format!(
                    "Unrecognized version_summary_mod call_fn_string: \"{}\"",
                    placeholder
                );
                eprintln!("{}", &err_msg);
                err_msg
            }
        }
    }
    /// html_templating functions that return a vector of Nodes
    #[allow(clippy::needless_return)]
    fn call_fn_vec_nodes(&self, placeholder: &str) -> Vec<Node> {
        // eprintln!("{}",&format!("call_fn_vec_nodes: {}", &placeholder));
        match placeholder {
            _ => {
                // so much boilerplate
                let err_msg = format!(
                    "Error: Unrecognized version_summary_mod call_fn_vec_nodes: \"{}\"",
                    placeholder
                );
                eprintln!("{}", err_msg);
                let node = Node {
                    node_enum: NodeEnum::Element(ElementNode {
                        tag_name: "h2".to_string(),
                        attributes: vec![],
                        children: vec![Node {
                            node_enum: NodeEnum::Text(err_msg),
                        }],
                        namespace: None,
                    }),
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
        _sub_templates: &Vec<SubTemplate>,
    ) -> Vec<Node> {
        // eprintln!("{}",&format!("render_sub_template: {}", &placeholder));
        match template_name {
            _ => {
                // so much boilerplate
                let err_msg = format!(
                    "Error: Unrecognized version_summary_mod render_sub_template: \"{}\"",
                    template_name
                );
                eprintln!("{}", &err_msg);
                let node = Node {
                    node_enum: NodeEnum::Element(ElementNode {
                        tag_name: "h2".to_string(),
                        attributes: vec![],
                        children: vec![Node {
                            node_enum: NodeEnum::Text(err_msg),
                        }],
                        namespace: None,
                    }),
                };
                return vec![node];
            }
        }
    }
}
