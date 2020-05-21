//! info_group_by_author_mod

use crate::duration_mod;
use crate::html_template_mod::*;
//use crate::utils_mod::*;
use crate::proof_index_mod::*;

use chrono::Local;
use unwrap::unwrap;

/// only one field with a generic name vec
#[derive(Clone, Debug)]
pub struct ProofIndexByAuthor {
    vec: Vec<ByAuthorItem>,
}
#[derive(Clone, Debug)]
pub struct ByAuthorItem {
    pub author: String,
    pub author_url: String,
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

impl ProofIndexByAuthor {
    pub fn new() -> ProofIndexByAuthor {
        let mut proof_index = ProofIndex::new();
        // sort order for group by, so I don't need to send a mutable
        proof_index
            .vec
            .sort_by(|a, b| Ord::cmp(&a.author, &b.author));
        let mut old_author = "".to_string();
        let mut for_unique_crates: Vec<String> = vec![];
        let mut proof_index_by_author = ProofIndexByAuthor{vec: vec![]};
        for index_item in &proof_index.vec {
            //the proofs are already sorted by author
            if &index_item.author != &old_author {
                if !old_author.is_empty() {
                    //finalize the previous group
                    use itertools::Itertools;
                    let mut last = unwrap!(proof_index_by_author.vec.last_mut());
                    last.unique_crates = for_unique_crates.into_iter().unique().count();
                    for_unique_crates = vec![];
                }
                //a new group begins
                let last = ByAuthorItem {
                    author: index_item.author.clone(),
                    author_url: index_item.author_url.clone(),
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
                proof_index_by_author.vec.push(last);
                old_author = index_item.author.to_string();
            }
            // add to the last group
            let mut last = unwrap!(proof_index_by_author.vec.last_mut());
            last.count_of_reviews += 1;
            for_unique_crates.push(index_item.author.to_string());
            last.count_of_rating_strong += index_item.rating_strong;
            last.count_of_rating_positive += index_item.rating_positive;
            last.count_of_rating_neutral += index_item.rating_neutral;
            last.count_of_rating_negative += index_item.rating_negative;
            last.count_of_rating_none += index_item.rating_none;
            last.count_of_alternatives += index_item.alternatives;
            last.count_of_issues += index_item.issues;
            last.count_of_advisories += index_item.advisories;
        }
        // println!("data_grouped: {:#?}", vec);
        //return
        proof_index_by_author
    }
}
impl HtmlTemplatingRender for ProofIndexByAuthor {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        //return
        "ProofIndexByAuthor".to_string()
    }
    /// render full html file from full template
    fn render_html_file(&self, templates_folder_name: &str) -> String {
        let start = duration_mod::start_ns();
        eprintln!(
            "{}: info_group_by_author_mod",
            &Local::now().format("%Y-%m-%d %H:%M:%S"),
        );
        let template_file_name = format!(
            "{}info_group_by_author_template.html",
            templates_folder_name
        );
        let html = self.render_from_file(&template_file_name);
        duration_mod::eprint_duration_ns("render_html_file()", start);
        // return
        html
    }

    // html_templating boolean id the next node is rendered or not
    fn retain_next_node(&self, placeholder: &str) -> bool {
        // eprintln!("{}",&format!("retain_next_node: {}", &placeholder));
        match placeholder {
            _ => retain_next_node_match_else(&self.data_model_name(), placeholder),
        }
    }

    // html_templating functions that return a String
    #[allow(
        clippy::needless_return,
        clippy::integer_arithmetic,
        clippy::indexing_slicing
    )]
    fn replace_with_string(&self, placeholder: &str, cursor_pos: usize) -> String {
        // eprintln!("{}",&format!("replace_with_string: {}", &placeholder));
        match placeholder {
            // the href for css is good for static data. For dynamic route it must be different.
            "t_css_href" => "/cargo_crev_web/css/cargo_crev_web.css".to_string(),
            "t_favicon_href" => "/cargo_crev_web/favicon.png".to_string(),
            // this is a grid with repeated rows. Use the cursor_pos
            "t_ordinal_number" => (cursor_pos + 1).to_string(),
            "t_author" => self.vec[cursor_pos].author.to_string(),
            "t_author_url" => format!("{}", self.vec[cursor_pos].author_url),
            "t_count_of_reviews" => {
                to_string_zero_to_empty(self.vec[cursor_pos].count_of_reviews)
            }
            "t_unique_crates" => {
                to_string_zero_to_empty(self.vec[cursor_pos].unique_crates)
            }
            "t_count_of_rating_strong" => {
                to_string_zero_to_empty(self.vec[cursor_pos].count_of_rating_strong)
            }
            "t_count_of_rating_positive" => {
                to_string_zero_to_empty(self.vec[cursor_pos].count_of_rating_positive)
            }
            "t_count_of_rating_neutral" => {
                to_string_zero_to_empty(self.vec[cursor_pos].count_of_rating_neutral)
            }
            "t_count_of_rating_negative" => {
                to_string_zero_to_empty(self.vec[cursor_pos].count_of_rating_negative)
            }
            "t_count_of_rating_none" => {
                to_string_zero_to_empty(self.vec[cursor_pos].count_of_rating_none)
            }
            "t_count_of_alternatives" => {
                to_string_zero_to_empty(self.vec[cursor_pos].count_of_alternatives)
            }
            "t_count_of_issues" => {
                to_string_zero_to_empty(self.vec[cursor_pos].count_of_issues)
            }
            "t_count_of_advisories" => {
                to_string_zero_to_empty(self.vec[cursor_pos].count_of_advisories)
            }
            _ => replace_with_string_match_else(&self.data_model_name(), placeholder),
        }
    }
    // html_templating functions that return a vector of Nodes
    #[allow(clippy::needless_return)]
    fn replace_with_nodes(&self, placeholder: &str) -> Vec<Node> {
        // eprintln!("{}",&format!("replace_with_nodes: {}", &placeholder));
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
        // eprintln!("{}",&format!("render_sub_template: {}", &placeholder));
        match template_name {
            "template_author_summary" => {
                // eprintln!("template_all_summaries: {}", "");
                let sub_template = unwrap!(sub_templates
                    .iter()
                    .find(|&template| template.name == template_name));
                let mut nodes = vec![];
                // sub-template repeatable
                for cursor_for_vec in 0..self.vec.len() {
                    let vec_node = unwrap!(self.render_template_raw_to_nodes(
                        &sub_template.template,
                        HtmlOrSvg::Html,
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
