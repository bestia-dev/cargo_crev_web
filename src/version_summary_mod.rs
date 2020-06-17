//! crate_version_summary_mod

// region: use

use crate::*;

//use serde_derive::{Deserialize, Serialize};
//use std::fs;
//use unwrap::unwrap;
// endregion: use

#[derive(Clone, Debug, Default)]
pub struct VersionSummary {
    pub crate_name: String,
    pub version: String,
    pub version_for_sorting: String,
    pub last_reviewed_version: String,
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
            crate_name: s!(),
            version: s!(),
            version_for_sorting: s!(),
            last_reviewed_version: s!(),
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
        // return
        s!("VersionSummary")
    }
    /// renders the complete html file. Not a sub-template/fragment.
    fn render_html_file(&self, _templates_folder_name: &str) -> String {
        // return
        String::new()
    }
    /// boolean : is the next node rendered or not
    fn retain_next_node_or_attribute(&self, placeholder: &str) -> bool {
        // dbg!(&placeholder);
        match placeholder {
            _ => retain_next_node_or_attribute_match_else(&self.data_model_name(), placeholder),
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
        // dbg!( &placeholder);
        match placeholder {
            "st_version" => s!(&self.version),
            "st_review_number" => url_s_zero_to_empty(self.review_number),
            "st_rating_strong" => url_s_zero_to_empty(self.rating_strong),
            "st_rating_positive" => url_s_zero_to_empty(self.rating_positive),
            "st_rating_neutral" => url_s_zero_to_empty(self.rating_neutral),
            "st_rating_negative" => url_s_zero_to_empty(self.rating_negative),
            "st_alternatives" => url_s_zero_to_empty(self.alternatives),
            "st_issues" => url_s_zero_to_empty(self.issues),
            "st_advisories" => url_s_zero_to_empty(self.advisories),
            "st_thoroughness" => url_s_zero_to_empty(self.thoroughness),
            "st_understanding" => url_s_zero_to_empty(self.understanding),
            _ => replace_with_string_match_else(&self.data_model_name(), placeholder),
        }
    }
    /// exclusive url encoded for href and src
    fn replace_with_url(
        &self,
        placeholder: &str,
        _subtemplate: &str,
        _pos_cursor: usize,
    ) -> UrlUtf8EncodedString {
        // dbg!( &placeholder);
        match placeholder {
            "su_filter_version" => url_u!(
                "/rust-reviews/crate/{}/{}/",
                &self.crate_name,
                &self.version
            ),
            "su_filter_strong" => url_u!(
                "/rust-reviews/crate/{}/{}/S/",
                &self.crate_name,
                &self.version
            ),
            "su_filter_positive" => url_u!(
                "/rust-reviews/crate/{}/{}/P/",
                &self.crate_name,
                &self.version
            ),
            "su_filter_neutral" => url_u!(
                "/rust-reviews/crate/{}/{}/E/",
                &self.crate_name,
                &self.version
            ),
            "su_filter_negative" => url_u!(
                "/rust-reviews/crate/{}/{}/N/",
                &self.crate_name,
                &self.version
            ),
            "su_filter_alternatives" => url_u!(
                "/rust-reviews/crate/{}/{}/v/",
                &self.crate_name,
                &self.version
            ),
            "su_filter_issues" => url_u!(
                "/rust-reviews/crate/{}/{}/i/",
                &self.crate_name,
                &self.version
            ),
            "su_filter_advisories" => url_u!(
                "/rust-reviews/crate/{}/{}/a/",
                &self.crate_name,
                &self.version
            ),
            _ => replace_with_url_match_else(&self.data_model_name(), placeholder),
        }
    }
    /// returns a vector of Nodes to replace the next Node
    #[allow(clippy::needless_return)]
    fn replace_with_nodes(&self, placeholder: &str) -> Vec<Node> {
        // dbg!( &placeholder);
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
        // dbg!( &placeholder);
        match template_name {
            _ => render_sub_template_match_else(&self.data_model_name(), template_name),
        }
    }
}
