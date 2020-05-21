//! review_index_summary_mod

//use crate::data_file_scan_mod::*;
use crate::duration_mod;
use crate::html_template_mod::*;
//use crate::review_mod::*;
//use crate::utils_mod::*;
use crate::review_index_mod::*;

use chrono::Local;
//use unwrap::unwrap;
#[derive(Clone, Debug)]
pub struct ReviewIndexSummary {
    pub unique_crates: usize,
    pub unique_authors: usize,
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
    pub fn new() -> ReviewIndexSummary {
        let review_index = ReviewIndex::new();
        let mut for_unique_crates: Vec<String> = vec![];
        let mut for_unique_authors: Vec<String> = vec![];
        let mut summary = ReviewIndexSummary {
            unique_crates: 0,
            unique_authors: 0,
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
        for index_item in review_index.vec {
            for_unique_crates.push(index_item.crate_name.to_string());
            for_unique_authors.push(index_item.author.to_string());
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
        // println!("data_grouped: {:#?}", order_by_crate);
        use itertools::Itertools;
        summary.unique_crates = for_unique_crates.into_iter().unique().count();
        summary.unique_authors = for_unique_authors.into_iter().unique().count();

        // return
        summary
    }
}

impl HtmlTemplatingRender for ReviewIndexSummary {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        //return
        "ReviewIndexSummary".to_string()
    }
    /// render html file
    fn render_html_file(&self, templates_folder_name: &str) -> String {
        let start = duration_mod::start_ns();
        eprintln!(
            "{}: review_index_mod",
            &Local::now().format("%Y-%m-%d %H:%M:%S"),
        );

        // count the reviews and their numeric values
        let before_render = duration_mod::eprint_duration_ns("  after new()", start);

        let template_file_name =
            format!("{}review_index_summary_template.html", templates_folder_name);
        let html = self.render_from_file(&template_file_name);

        duration_mod::eprint_duration_ns("  render", before_render);
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
    fn replace_with_string(&self, placeholder: &str, _cursor_pos: usize) -> String {
        // eprintln!("{}",&format!("replace_with_string: {}", &placeholder));
        match placeholder {
            // the href for css is good for static data. For dynamic route it must be different.
            "t_css_href" => "/cargo_crev_web/css/cargo_crev_web.css".to_string(),
            "t_favicon_href" => "/cargo_crev_web/favicon.png".to_string(),
            "t_unique_crates" => self.unique_crates.to_string(),
            "t_unique_authors" => self.unique_authors.to_string(),
            "t_count_of_reviews" => self.count_of_reviews.to_string(),
            "t_count_of_rating_strong" => self.count_of_rating_strong.to_string(),
            "t_count_of_rating_positive" => self.count_of_rating_positive.to_string(),
            "t_count_of_rating_neutral" => self.count_of_rating_neutral.to_string(),
            "t_count_of_rating_negative" => self.count_of_rating_negative.to_string(),
            "t_count_of_rating_none" => self.count_of_rating_none.to_string(),
            "t_count_of_alternatives" => self.count_of_alternatives.to_string(),
            "t_count_of_issues" => self.count_of_issues.to_string(),
            "t_count_of_advisories" => self.count_of_advisories.to_string(),
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
