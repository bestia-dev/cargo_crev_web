//! reviewers_mod

use crate::*;

use unwrap::unwrap;

/// only one field with a generic name vec
#[derive(Clone, Debug)]
pub struct ReviewIndexByReviewer {
    vec: Vec<ByReviewerItem>,
}
#[derive(Clone, Debug)]
pub struct ByReviewerItem {
    pub reviewer_name: String,
    pub reviewer_url: String,
    pub reviewer_id: String,
    pub count_of_reviews: usize,
    pub unique_crates: usize,
    pub count_of_rating_strong: usize,
    pub count_of_rating_positive: usize,
    pub count_of_rating_neutral: usize,
    pub count_of_rating_negative: usize,
    pub count_of_rating_none: usize,
    pub count_of_alternatives: usize,
    pub count_of_issues: usize,
    pub count_of_advisories: usize,
}

impl ReviewIndexByReviewer {
    pub fn new(state_global: ArcMutStateGlobal) -> Self {
        // sort order for group by, so I don't need to send a mutable
        unwrap!(state_global.lock())
            .review_index
            .vec
            .sort_by(|a, b| Ord::cmp(&a.reviewer_name.to_lowercase(), &b.reviewer_name.to_lowercase()));
        let mut old_reviewer_name = s!();
        let mut for_unique_crates: Vec<String> = vec![];
        let mut review_index_by_reviewer = ReviewIndexByReviewer { vec: vec![] };
        for index_item in unwrap!(state_global.lock()).review_index.vec.iter() {
            // the reviews are already sorted by reviewer_name
            if &index_item.reviewer_name != &old_reviewer_name {
                if !old_reviewer_name.is_empty() {
                    // finalize the previous group
                    use itertools::Itertools;
                    let mut last = unwrap!(review_index_by_reviewer.vec.last_mut());
                    last.unique_crates = for_unique_crates.into_iter().unique().count();
                    for_unique_crates = vec![];
                }
                // a new group begins
                let last = ByReviewerItem {
                    reviewer_name: index_item.reviewer_name.clone(),
                    reviewer_url: index_item.reviewer_url.clone(),
                    reviewer_id: index_item.reviewer_id.clone(),
                    unique_crates: 0,
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
                review_index_by_reviewer.vec.push(last);
                old_reviewer_name = s!(&index_item.reviewer_name);
            }
            // add to the last group
            let mut last = unwrap!(review_index_by_reviewer.vec.last_mut());
            last.count_of_reviews += 1;
            for_unique_crates.push(s!(&index_item.reviewer_name));
            last.count_of_rating_strong += index_item.rating_strong;
            last.count_of_rating_positive += index_item.rating_positive;
            last.count_of_rating_neutral += index_item.rating_neutral;
            last.count_of_rating_negative += index_item.rating_negative;
            last.count_of_rating_none += index_item.rating_none;
            last.count_of_alternatives += index_item.alternatives;
            last.count_of_issues += index_item.issues;
            last.count_of_advisories += index_item.advisories;
        }

        // return
        review_index_by_reviewer
    }
}
impl HtmlServerTemplateRender for ReviewIndexByReviewer {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        // return
        s!("ReviewIndexByReviewer")
    }
    /// renders the complete html file. Not a sub-template/fragment.
    fn render_html_file(&self, templates_folder_name: &str) -> String {
        let template_file_name = format!("{}reviewers_template.html", templates_folder_name);
        let html = self.render_from_file(&template_file_name);
        // return
        html
    }

    /// boolean : is the next node rendered or not
    fn retain_next_node_or_attribute(&self, placeholder: &str) -> bool {
        // dbg!(&placeholder);
        match placeholder {
            _ => retain_next_node_or_attribute_match_else(&self.data_model_name(), placeholder),
        }
    }

    /// returns a String to replace the next text-node
    #[allow(clippy::needless_return, clippy::integer_arithmetic, clippy::indexing_slicing)]
    fn replace_with_string(&self, placeholder: &str, _subtemplate: &str, pos_cursor: usize) -> String {
        // dbg!( &placeholder);
        match placeholder {
            "st_cargo_crev_web_version" => s!(env!("CARGO_PKG_VERSION")),
            // this is a grid with repeated rows. Use the pos_cursor
            "st_ordinal_number" => s!(pos_cursor + 1),
            "st_reviewer_name" => s!(&self.vec[pos_cursor].reviewer_name),
            "st_count_of_reviews" => url_s_zero_to_empty(self.vec[pos_cursor].count_of_reviews),
            "st_unique_crates" => url_s_zero_to_empty(self.vec[pos_cursor].unique_crates),
            "st_count_of_rating_strong" => url_s_zero_to_empty(self.vec[pos_cursor].count_of_rating_strong),
            "st_count_of_rating_positive" => url_s_zero_to_empty(self.vec[pos_cursor].count_of_rating_positive),
            "st_count_of_rating_neutral" => url_s_zero_to_empty(self.vec[pos_cursor].count_of_rating_neutral),
            "st_count_of_rating_negative" => url_s_zero_to_empty(self.vec[pos_cursor].count_of_rating_negative),
            "st_count_of_rating_none" => url_s_zero_to_empty(self.vec[pos_cursor].count_of_rating_none),
            "st_count_of_alternatives" => url_s_zero_to_empty(self.vec[pos_cursor].count_of_alternatives),
            "st_count_of_issues" => url_s_zero_to_empty(self.vec[pos_cursor].count_of_issues),
            "st_count_of_advisories" => url_s_zero_to_empty(self.vec[pos_cursor].count_of_advisories),
            _ => replace_with_string_match_else(&self.data_model_name(), placeholder),
        }
    }
    /// exclusive url encoded for href and src
    fn replace_with_url(&self, placeholder: &str, _subtemplate: &str, pos_cursor: usize) -> UrlUtf8EncodedString {
        // dbg!( &placeholder);
        match placeholder {
            // the href for css is good for static data. For dynamic route it must be different.
            "su_css_route" => url_u!("/rust-reviews/css/rust-reviews.css"),
            "su_favicon_route" => url_u!("/rust-reviews/favicon.png"),
            "su_img_src_logo" => url_u!("/rust-reviews/images/Logo_02.png"),
            "su_reviewer_route" => url_u!("/rust-reviews/reviewer/{}/", &self.vec[pos_cursor].reviewer_id),
            "su_reviewer_url" => url_u!(&self.vec[pos_cursor].reviewer_url, ""),
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
    // render sub-template into Vec<Node>
    #[allow(clippy::needless_return)]
    fn render_sub_template(&self, template_name: &str, sub_templates: &Vec<SubTemplate>) -> Vec<Node> {
        // dbg!(&placeholder));
        match template_name {
            "stmplt_reviewer_summary" => {
                let sub_template = unwrap!(sub_templates.iter().find(|&template| template.name == template_name));
                let mut nodes = vec![];
                // sub-template repeatable
                for cursor_for_vec in 0..self.vec.len() {
                    let vec_node =
                        unwrap!(self.render_template_raw_to_nodes(&sub_template.template, HtmlOrSvg::Html, "stmplt_reviewer_summary", cursor_for_vec));
                    nodes.extend_from_slice(&vec_node);
                }
                // return
                nodes
            }
            _ => render_sub_template_match_else(&self.data_model_name(), template_name),
        }
    }
}
