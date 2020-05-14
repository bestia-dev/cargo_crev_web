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
    pub crate_name: String,
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
            crate_name: "".to_string(),
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

impl HtmlTemplatingRender for VersionSummary {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        //return
        "VersionSummary".to_string()
    }
    // html_templating boolean id the next node is rendered or not
    fn call_fn_boolean(&self, placeholder: &str) -> bool {
        // eprintln!("{}",&format!("call_fn_boolean: {}", &placeholder));
        match placeholder {
            _ => call_fn_boolean_match_else(&self.data_model_name(), placeholder),
        }
    }

    // html_templating functions that return a String
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

            "t_filter_version" => {
                format!("/cargo_crev_web/query/{}/{}", self.crate_name, self.version)
            }
            "t_filter_strong" => format!(
                "/cargo_crev_web/query/{}/{}/S",
                self.crate_name, self.version
            ),
            "t_filter_positive" => format!(
                "/cargo_crev_web/query/{}/{}/P",
                self.crate_name, self.version
            ),
            "t_filter_neutral" => format!(
                "/cargo_crev_web/query/{}/{}/E",
                self.crate_name, self.version
            ),
            "t_filter_negative" => format!(
                "/cargo_crev_web/query/{}/{}/N",
                self.crate_name, self.version
            ),
            "t_filter_alternatives" => format!(
                "/cargo_crev_web/query/{}/{}/v",
                self.crate_name, self.version
            ),
            "t_filter_issues" => format!(
                "/cargo_crev_web/query/{}/{}/i",
                self.crate_name, self.version
            ),
            "t_filter_advisories" => format!(
                "/cargo_crev_web/query/{}/{}/a",
                self.crate_name, self.version
            ),
            _ => call_fn_string_match_else(&self.data_model_name(), placeholder),
        }
    }
    // html_templating functions that return a vector of Nodes
    #[allow(clippy::needless_return)]
    fn call_fn_vec_nodes(&self, placeholder: &str) -> Vec<Node> {
        // eprintln!("{}",&format!("call_fn_vec_nodes: {}", &placeholder));
        match placeholder {
            _ => call_fn_vec_nodes_match_else(&self.data_model_name(), placeholder),
        }
    }
    // html_templating for sub-template
    #[allow(clippy::needless_return)]
    fn render_sub_template(
        &self,
        template_name: &str,
        _sub_templates: &Vec<SubTemplate>,
    ) -> Vec<Node> {
        // eprintln!("{}",&format!("render_sub_template: {}", &placeholder));
        match template_name {
            _ => render_sub_template_match_else(&self.data_model_name(), template_name),
        }
    }
}
