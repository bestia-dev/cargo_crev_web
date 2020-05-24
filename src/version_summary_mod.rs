//! crate_version_summary_mod

// region: use
use crate::*;
use html_server_template_mod::*;
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
            crate_name: s!(""),
            version: s!(""),
            version_for_sorting: s!(""),
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

impl HtmlServerTemplateRender for VersionSummary {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        //return
        s!("VersionSummary")
    }
    /// renders the complete html file. Not a sub-template/fragment.
    fn render_html_file(&self, _templates_folderange_name: &str) -> String {
        //return
        String::new()
    }
    /// boolean : is the next node rendered or not
    fn retain_next_node(&self, placeholder: &str) -> bool {
        // eprintln!("{}",&format!("retain_next_node: {}", &placeholder));
        match placeholder {
            _ => retain_next_node_match_else(&self.data_model_name(), placeholder),
        }
    }

    /// returns a String to replace the next text-node
    #[allow(
        clippy::needless_return,
        clippy::integer_arithmetic,
        clippy::indexing_slicing
    )]
    fn replace_with_string(
        &self,
        placeholder: &str,
        _subtemplate: &str,
        _pos_cursor: usize,
    ) -> String {
        // eprintln!("{}",&format!("replace_with_string: {}", &placeholder));
        match placeholder {
            "st_version" => s!(&self.version),
            "st_review_number" => to_string_zero_to_empty(self.review_number),
            "st_rating_strong" => to_string_zero_to_empty(self.rating_strong),
            "st_rating_positive" => to_string_zero_to_empty(self.rating_positive),
            "st_rating_neutral" => to_string_zero_to_empty(self.rating_neutral),
            "st_rating_negative" => to_string_zero_to_empty(self.rating_negative),
            "st_alternatives" => to_string_zero_to_empty(self.alternatives),
            "st_issues" => to_string_zero_to_empty(self.issues),
            "st_advisories" => to_string_zero_to_empty(self.advisories),
            "st_thoroughness" => to_string_zero_to_empty(self.thoroughness),
            "st_understanding" => to_string_zero_to_empty(self.understanding),

            "st_filter_version" => {
                format!("/cargo_crev_web/crate/{}/{}", self.crate_name, self.version)
            }
            "st_filter_strong" => format!(
                "/cargo_crev_web/crate/{}/{}/S",
                self.crate_name, self.version
            ),
            "st_filter_positive" => format!(
                "/cargo_crev_web/crate/{}/{}/P",
                self.crate_name, self.version
            ),
            "st_filter_neutral" => format!(
                "/cargo_crev_web/crate/{}/{}/E",
                self.crate_name, self.version
            ),
            "st_filter_negative" => format!(
                "/cargo_crev_web/crate/{}/{}/N",
                self.crate_name, self.version
            ),
            "st_filter_alternatives" => format!(
                "/cargo_crev_web/crate/{}/{}/v",
                self.crate_name, self.version
            ),
            "st_filter_issues" => format!(
                "/cargo_crev_web/crate/{}/{}/i",
                self.crate_name, self.version
            ),
            "st_filter_advisories" => format!(
                "/cargo_crev_web/crate/{}/{}/a",
                self.crate_name, self.version
            ),
            _ => replace_with_string_match_else(&self.data_model_name(), placeholder),
        }
    }
    /// returns a vector of Nodes to replace the next Node
    #[allow(clippy::needless_return)]
    fn replace_with_nodes(&self, placeholder: &str) -> Vec<Node> {
        // eprintln!("{}",&format!("replace_with_nodes: {}", &placeholder));
        match placeholder {
            _ => replace_with_nodes_match_else(&self.data_model_name(), placeholder),
        }
    }
    /// renders sub-template
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
