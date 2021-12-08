//! review_index_summary_mod

use crate::*;

use unwrap::unwrap;

#[derive(Clone, Debug)]
pub struct ReviewIndexSummary {
    pub unique_crates: usize,
    pub unique_reviewers: usize,
    pub count_of_reviews: usize,
    pub count_of_rating_strong: usize,
    pub count_of_rating_positive: usize,
    pub count_of_rating_neutral: usize,
    pub count_of_rating_negative: usize,
    pub count_of_rating_none: usize,
    pub count_of_alternatives: usize,
    pub count_of_issues: usize,
    pub count_of_advisories: usize,
}

impl ReviewIndexSummary {
    /// prepares the data
    pub fn new(state_global: ArcMutStateGlobal) -> Self {
        let mut for_unique_crates: Vec<String> = vec![];
        let mut for_unique_reviewers: Vec<String> = vec![];
        let mut summary = ReviewIndexSummary {
            unique_crates: 0,
            unique_reviewers: 0,
            count_of_reviews: 0,
            count_of_rating_strong: 0,
            count_of_rating_positive: 0,
            count_of_rating_neutral: 0,
            count_of_rating_negative: 0,
            count_of_rating_none: 0,
            count_of_alternatives: 0,
            count_of_issues: 0,
            count_of_advisories: 0,
        };
        for index_item in unwrap!(state_global.lock()).review_index.vec.iter() {
            for_unique_crates.push(s!(&index_item.crate_name));
            for_unique_reviewers.push(s!(&index_item.reviewer_name));
            summary.count_of_reviews += 1;
            summary.count_of_rating_strong += index_item.rating_strong;
            summary.count_of_rating_positive += index_item.rating_positive;
            summary.count_of_rating_neutral += index_item.rating_neutral;
            summary.count_of_rating_negative += index_item.rating_negative;
            summary.count_of_rating_none += index_item.rating_none;
            summary.count_of_alternatives += index_item.alternatives;
            summary.count_of_issues += index_item.issues;
            summary.count_of_advisories += index_item.advisories;
        }
        // dbg!( crates);
        use itertools::Itertools;
        summary.unique_crates = for_unique_crates.into_iter().unique().count();
        summary.unique_reviewers = for_unique_reviewers.into_iter().unique().count();

        // return
        summary
    }
}

impl HtmlServerTemplateRender for ReviewIndexSummary {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        // return
        s!("ReviewIndexSummary")
    }
    /// renders the complete html file. Not a sub-template/fragment.
    fn render_html_file(&self, templates_folder_name: &str) -> String {
        let template_file_name = format!("{}index_template.html", templates_folder_name);
        let html = self.render_from_file(&template_file_name);

        // return
        html
    }
    /// boolean : is the next node rendered or not
    fn retain_next_node_or_attribute(&self, placeholder: &str) -> bool {
        // dbg!( &placeholder);
        match placeholder {
            _ => retain_next_node_or_attribute_match_else(&self.data_model_name(), placeholder),
        }
    }

    /// returns a String to replace the next text-node
    #[allow(clippy::needless_return, clippy::integer_arithmetic, clippy::indexing_slicing)]
    fn replace_with_string(&self, placeholder: &str, _subtemplate: &str, _pos_cursor: usize) -> String {
        // dbg!(&placeholder);
        match placeholder {
            "st_cargo_crev_web_version" => s!(env!("CARGO_PKG_VERSION")),
            "st_unique_crates" => s!(self.unique_crates),
            "st_unique_reviewers" => s!(self.unique_reviewers),
            "st_count_of_reviews" => s!(self.count_of_reviews),
            "st_count_of_rating_strong" => s!(self.count_of_rating_strong),
            "st_count_of_rating_positive" => s!(self.count_of_rating_positive),
            "st_count_of_rating_neutral" => s!(self.count_of_rating_neutral),
            "st_count_of_rating_negative" => s!(self.count_of_rating_negative),
            "st_count_of_rating_none" => s!(self.count_of_rating_none),
            "st_count_of_alternatives" => s!(self.count_of_alternatives),
            "st_count_of_issues" => s!(self.count_of_issues),
            "st_count_of_advisories" => s!(self.count_of_advisories),
            _ => replace_with_string_match_else(&self.data_model_name(), placeholder),
        }
    }
    /// exclusive url encoded for href and src
    fn replace_with_url(&self, placeholder: &str, _subtemplate: &str, _pos_cursor: usize) -> UrlUtf8EncodedString {
        // dbg!( &placeholder);
        match placeholder {
            // the href for css is good for static data. For dynamic route it must be different.
            "su_css_route" => url_u!("/rust-reviews/css/rust-reviews.css"),
            "su_favicon_route" => url_u!("/rust-reviews/favicon.png"),
            "su_img_src_logo" => url_u!("/rust-reviews/images/Logo_02.png"),
            _ => replace_with_url_match_else(&self.data_model_name(), placeholder),
        }
    }
    /// returns a vector of Nodes to replace the next Node
    #[allow(clippy::needless_return)]
    fn replace_with_nodes(&self, placeholder: &str) -> Vec<Node> {
        // dbg!(&placeholder);
        match placeholder {
            _ => replace_with_nodes_match_else(&self.data_model_name(), placeholder),
        }
    }
    /// renders sub-template
    #[allow(clippy::needless_return)]
    fn render_sub_template(&self, template_name: &str, _sub_templates: &Vec<SubTemplate>) -> Vec<Node> {
        // dbg!(&placeholder);
        match template_name {
            _ => render_sub_template_match_else(&self.data_model_name(), template_name),
        }
    }
}
