//! info_group_by_author_mod

use crate::html_server_template_mod::*;
use crate::url_encode_mod::*;
use crate::*;

use unwrap::unwrap;

/// only one field with a generic name vec
#[derive(Clone, Debug)]
pub struct ReviewIndexByAuthor {
    vec: Vec<ByAuthorItem>,
}
#[derive(Clone, Debug)]
pub struct ByAuthorItem {
    pub author_name: String,
    pub author_url: String,
    pub author_id: String,
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

impl ReviewIndexByAuthor {
    pub fn new(cached_review_index: CachedReviewIndex) -> Self {
        let mut review_index = cached_review_index
            .lock()
            .expect("error cached_review_index.lock()");
        // sort order for group by, so I don't need to send a mutable
        review_index
            .vec
            .sort_by(|a, b| Ord::cmp(&a.author_name, &b.author_name));
        let mut old_author_name = s!("");
        let mut for_unique_crates: Vec<String> = vec![];
        let mut review_index_by_author = ReviewIndexByAuthor { vec: vec![] };
        for index_item in &review_index.vec {
             // the reviews are already sorted by author_name
            if &index_item.author_name != &old_author_name {
                if !old_author_name.is_empty() {
                     // finalize the previous group
                    use itertools::Itertools;
                    let mut last = unwrap!(review_index_by_author.vec.last_mut());
                    last.unique_crates = for_unique_crates.into_iter().unique().count();
                    for_unique_crates = vec![];
                }
                 // a new group begins
                let last = ByAuthorItem {
                    author_name: index_item.author_name.clone(),
                    author_url: index_item.author_url.clone(),
                    author_id: index_item.author_id.clone(),
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
                review_index_by_author.vec.push(last);
                old_author_name = s!(&index_item.author_name);
            }
            // add to the last group
            let mut last = unwrap!(review_index_by_author.vec.last_mut());
            last.count_of_reviews += 1;
            for_unique_crates.push(s!(&index_item.author_name));
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
        review_index_by_author
    }
}
impl HtmlServerTemplateRender for ReviewIndexByAuthor {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
         // return
        s!("ReviewIndexByAuthor")
    }
    /// renders the complete html file. Not a sub-template/fragment.
    fn render_html_file(&self, templates_folder_name: &str) -> String {
        let template_file_name = format!(
            "{}info_group_by_author_template.html",
            templates_folder_name
        );
        let html = self.render_from_file(&template_file_name);
        // return
        html
    }

    /// boolean : is the next node rendered or not
    fn retain_next_node(&self, placeholder: &str) -> bool {
        // dbg!(&placeholder);
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
        pos_cursor: usize,
    ) -> String {
        // dbg!( &placeholder);
        match placeholder {
            // the href for css is good for static data. For dynamic route it must be different.
            "st_css_route" => s!("/cargo_crev_web/css/cargo_crev_web.css"),
            "st_favicon_route" => s!("/cargo_crev_web/favicon.png"),
            // this is a grid with repeated rows. Use the pos_cursor
            "st_ordinal_number" => (pos_cursor + 1).to_string(),
            "st_author_name" => s!(&self.vec[pos_cursor].author_name),
            "st_author_url" => format!("{}", self.vec[pos_cursor].author_url),
            "st_author_route" => format!(
                "/cargo_crev_web/author/{}/",
                url_encode(&self.vec[pos_cursor].author_id)
            ),
            "st_count_of_reviews" => to_string_zero_to_empty(self.vec[pos_cursor].count_of_reviews),
            "st_unique_crates" => to_string_zero_to_empty(self.vec[pos_cursor].unique_crates),
            "st_count_of_rating_strong" => {
                to_string_zero_to_empty(self.vec[pos_cursor].count_of_rating_strong)
            }
            "st_count_of_rating_positive" => {
                to_string_zero_to_empty(self.vec[pos_cursor].count_of_rating_positive)
            }
            "st_count_of_rating_neutral" => {
                to_string_zero_to_empty(self.vec[pos_cursor].count_of_rating_neutral)
            }
            "st_count_of_rating_negative" => {
                to_string_zero_to_empty(self.vec[pos_cursor].count_of_rating_negative)
            }
            "st_count_of_rating_none" => {
                to_string_zero_to_empty(self.vec[pos_cursor].count_of_rating_none)
            }
            "st_count_of_alternatives" => {
                to_string_zero_to_empty(self.vec[pos_cursor].count_of_alternatives)
            }
            "st_count_of_issues" => to_string_zero_to_empty(self.vec[pos_cursor].count_of_issues),
            "st_count_of_advisories" => {
                to_string_zero_to_empty(self.vec[pos_cursor].count_of_advisories)
            }
            _ => replace_with_string_match_else(&self.data_model_name(), placeholder),
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
    fn render_sub_template(
        &self,
        template_name: &str,
        sub_templates: &Vec<SubTemplate>,
    ) -> Vec<Node> {
        // dbg!(&placeholder));
        match template_name {
            "stmplt_author_summary" => {
                let sub_template = unwrap!(sub_templates
                    .iter()
                    .find(|&template| template.name == template_name));
                let mut nodes = vec![];
                // sub-template repeatable
                for cursor_for_vec in 0..self.vec.len() {
                    let vec_node = unwrap!(self.render_template_raw_to_nodes(
                        &sub_template.template,
                        HtmlOrSvg::Html,
                        "stmplt_author_summary",
                        cursor_for_vec
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
