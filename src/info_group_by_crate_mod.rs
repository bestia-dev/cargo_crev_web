//! info_group_by_crate_mod

use crate::html_server_template_mod::*;
use crate::url_encode_mod::*;
use crate::CachedReviewIndex;
use crate::*;

use unwrap::unwrap;

/// only one field with a generic name vec
#[derive(Clone, Debug)]
pub struct ReviewIndexByCrate {
    pub vec: Vec<ByCrateItem>,
}
#[derive(Clone, Debug)]
pub struct ByCrateItem {
    pub crate_name: String,
    pub count_of_reviews: usize,
    pub unique_versions: usize,
    pub unique_authors: usize,
    pub count_of_rating_strong: usize,
    pub count_of_rating_positive: usize,
    pub count_of_rating_neutral: usize,
    pub count_of_rating_negative: usize,
    pub count_of_rating_none: usize,
    pub count_of_alternatives: usize,
    pub count_of_issues: usize,
    pub count_of_advisories: usize,
}

impl ReviewIndexByCrate {
    pub fn new(cached_review_index: CachedReviewIndex) -> Self {
        let mut review_index = cached_review_index
            .lock()
            .expect("error cached_review_index.lock()");

        // sort order for group by, so I don't need to send a mutable
        review_index
            .vec
            .sort_by(|a, b| Ord::cmp(&a.crate_name, &b.crate_name));

        let mut old_crate_name = s!("");
        let mut for_unique_versions: Vec<String> = vec![];
        let mut for_unique_authors: Vec<String> = vec![];
        let mut review_index_by_crate = ReviewIndexByCrate { vec: vec![] };
        for index_item in &review_index.vec {
            //the reviews are already sorted by crate_name
            if &index_item.crate_name != &old_crate_name {
                if !old_crate_name.is_empty() {
                    //finalize the previous group
                    use itertools::Itertools;
                    let mut last = unwrap!(review_index_by_crate.vec.last_mut());
                    last.unique_versions = for_unique_versions.into_iter().unique().count();
                    for_unique_versions = vec![];
                    last.unique_authors = for_unique_authors.into_iter().unique().count();
                    for_unique_authors = vec![];
                }
                //a new group begins
                let last = ByCrateItem {
                    crate_name: index_item.crate_name.clone(),
                    count_of_reviews: 0,
                    unique_versions: 0,
                    unique_authors: 0,
                    count_of_rating_strong: 0,
                    count_of_rating_positive: 0,
                    count_of_rating_neutral: 0,
                    count_of_rating_negative: 0,
                    count_of_rating_none: 0,
                    count_of_alternatives: 0,
                    count_of_issues: 0,
                    count_of_advisories: 0,
                };
                review_index_by_crate.vec.push(last);
                old_crate_name = s!(&index_item.crate_name);
            }
            // add to the last group
            let mut last = unwrap!(review_index_by_crate.vec.last_mut());
            last.count_of_reviews += 1;
            for_unique_versions.push(s!(&index_item.version));
            for_unique_authors.push(s!(&index_item.author));
            last.count_of_rating_strong += index_item.rating_strong;
            last.count_of_rating_positive += index_item.rating_positive;
            last.count_of_rating_neutral += index_item.rating_neutral;
            last.count_of_rating_negative += index_item.rating_negative;
            last.count_of_rating_none += index_item.rating_none;
            last.count_of_alternatives += index_item.alternatives;
            last.count_of_issues += index_item.issues;
            last.count_of_advisories += index_item.advisories;
        }

        //return
        review_index_by_crate
    }
}
impl HtmlServerTemplateRender for ReviewIndexByCrate {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        //return
        s!("ReviewIndexByCrate")
    }
    /// renders the complete html file. Not a sub-template/fragment.
    fn render_html_file(&self, templates_folderange_name: &str) -> String {
        let template_file_name =
            format!("{}info_group_by_crate_template.html", templates_folderange_name);
        let html = self.render_from_file(&template_file_name);

        // return
        html
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
        pos_cursor: usize,
    ) -> String {
        // eprintln!("{}",&format!("replace_with_string: {}", &placeholder));
        match placeholder {
            // the href for css is good for static data. For dynamic route it must be different.
            "st_css_route" => s!("/cargo_crev_web/css/cargo_crev_web.css"),
            "st_favicon_route" => s!("/cargo_crev_web/favicon.png"),
            // this is a grid with repeated rows. Use the pos_cursor
            "st_ordinal_number" => (pos_cursor + 1).to_string(),
            "st_crate_name" => s!(&self.vec[pos_cursor].crate_name),
            "st_crate_route" => format!(
                "/cargo_crev_web/crate/{}/",
                url_encode(&self.vec[pos_cursor].crate_name)
            ),
            "st_count_of_reviews" => to_string_zero_to_empty(self.vec[pos_cursor].count_of_reviews),
            "st_unique_versions" => to_string_zero_to_empty(self.vec[pos_cursor].unique_versions),
            "st_unique_authors" => to_string_zero_to_empty(self.vec[pos_cursor].unique_authors),
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
        sub_templates: &Vec<SubTemplate>,
    ) -> Vec<Node> {
        // eprintln!("{}",&format!("render_sub_template: {}", &placeholder));
        match template_name {
            "stmplt_crate_summary" => {
                // eprintln!("stmplt_crate_version_summary: {}", "");
                let sub_template = unwrap!(sub_templates
                    .iter()
                    .find(|&template| template.name == template_name));
                let mut nodes = vec![];
                // sub-template repeatable
                for cursor_for_order_by_crate in 0..self.vec.len() {
                    let vec_node = unwrap!(self.render_template_raw_to_nodes(
                        &sub_template.template,
                        HtmlOrSvg::Html,
                        template_name,
                        cursor_for_order_by_crate
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
