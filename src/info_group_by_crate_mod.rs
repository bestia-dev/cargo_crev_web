//! info_group_by_crate_mod

use crate::duration_mod;
use crate::html_template_mod::*;
//use crate::utils_mod::*;
use crate::proof_index_mod::*;

use chrono::Local;
use unwrap::unwrap;

#[derive(Clone, Debug)]
pub struct InfoDataByCrate {
    order_by_crate: Vec<ByCrateItem>,
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

impl InfoDataByCrate {

    pub fn new()->InfoDataByCrate{
        let mut proofs_index = InfoData::prepare_proofs_index();
        // sort order for group by, so I don't need to send a mutable
        proofs_index.sort_by(|a, b| Ord::cmp(&a.crate_name, &b.crate_name));
        let order_by_crate = Self::group_by_crate(&proofs_index);

        //return
        InfoDataByCrate {
            order_by_crate,
        }
    }
    /// info about reviews
    pub fn render_html_file(&self, templates_folder_name: &str) -> String {
        let start = duration_mod::start_ns();
        eprintln!("{}: info_group_by_crate_mod", &Local::now().format("%Y-%m-%d %H:%M:%S"),);

        // count the proofs and their numeric values
        
        let before_render = duration_mod::eprint_duration_ns("  after prepare_proofs_index()", start);

        let template_file_name = format!("{}info_group_by_crate_template.html", templates_folder_name);
        let html = self.render_from_file(&template_file_name);

        duration_mod::eprint_duration_ns("  render", before_render);
        duration_mod::eprint_duration_ns("render_html_file()", start);
        // return
        html
    }

    /// create a new vector with data grouped by crate
    pub fn group_by_crate(proofs_index: & Vec<ProofIndexItem> ) -> Vec<ByCrateItem> {

        let mut old_crate_name = "".to_string();
        let mut for_unique_versions: Vec<String> = vec![];
        let mut for_unique_authors: Vec<String> = vec![];
        let mut order_by_crate: Vec<ByCrateItem> = vec![];
        for index_item in proofs_index {
            //the proofs are already sorted by crate_name
            if &index_item.crate_name != &old_crate_name {
                if !old_crate_name.is_empty() {
                    //finalize the previous group
                    use itertools::Itertools;
                    let mut last = unwrap!(order_by_crate.last_mut());
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
                order_by_crate.push(last);
                old_crate_name = index_item.crate_name.to_string();
            }
            // add to the last group
            let mut last = unwrap!(order_by_crate.last_mut());
            last.count_of_reviews += 1;
            for_unique_versions.push(index_item.version.to_string());
            for_unique_authors.push(index_item.author.to_string());
            last.count_of_rating_strong += index_item.rating_strong;
            last.count_of_rating_positive += index_item.rating_positive;
            last.count_of_rating_neutral += index_item.rating_neutral;
            last.count_of_rating_negative += index_item.rating_negative;
            last.count_of_rating_none += index_item.rating_none;
            last.count_of_alternatives += index_item.alternatives;
            last.count_of_issues += index_item.issues;
            last.count_of_advisories += index_item.advisories;
        }
        // println!("data_grouped: {:#?}", order_by_crate);
        //return
        order_by_crate
    }
    //pub fn group_by_author() {}
}
impl HtmlTemplatingRender for InfoDataByCrate {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        //return
        "InfoDataByCrate".to_string()
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
    fn call_fn_string(&self, placeholder: &str, cursor_pos: usize) -> String {
        // eprintln!("{}",&format!("call_fn_string: {}", &placeholder));
        match placeholder {
            // the href for css is good for static data. For dynamic route it must be different.
            "t_css_href" => "/cargo_crev_web/css/cargo_crev_web.css".to_string(),
            "t_favicon_href" => "/cargo_crev_web/favicon.png".to_string(),
            // this is a grid with repeated rows. Use the cursor_pos
            "t_ordinal_number" => (cursor_pos + 1).to_string(),
            "t_crate_name" => self.order_by_crate[cursor_pos].crate_name.to_string(),
            "t_open_crate" => format!("../../query/{}", self.order_by_crate[cursor_pos].crate_name),
            "t_count_of_reviews" => to_string_zero_to_empty(self.order_by_crate[cursor_pos].count_of_reviews),
            "t_unique_versions" => to_string_zero_to_empty(self.order_by_crate[cursor_pos].unique_versions),
            "t_unique_authors" => to_string_zero_to_empty(self.order_by_crate[cursor_pos].unique_authors),
            "t_count_of_rating_strong" => to_string_zero_to_empty(self.order_by_crate[cursor_pos]
                .count_of_rating_strong),
            "t_count_of_rating_positive" => to_string_zero_to_empty(self.order_by_crate[cursor_pos]
                .count_of_rating_positive),
            "t_count_of_rating_neutral" => to_string_zero_to_empty(self.order_by_crate[cursor_pos]
                .count_of_rating_neutral),
            "t_count_of_rating_negative" => to_string_zero_to_empty(self.order_by_crate[cursor_pos]
                .count_of_rating_negative),
            "t_count_of_rating_none" => to_string_zero_to_empty(self.order_by_crate[cursor_pos]
                .count_of_rating_none),
            "t_count_of_alternatives" => to_string_zero_to_empty(self.order_by_crate[cursor_pos]
                .count_of_alternatives),
            "t_count_of_issues" => to_string_zero_to_empty(self.order_by_crate[cursor_pos].count_of_issues),
            "t_count_of_advisories" => to_string_zero_to_empty(self.order_by_crate[cursor_pos]
                .count_of_advisories),
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
        sub_templates: &Vec<SubTemplate>,
    ) -> Vec<Node> {
        // eprintln!("{}",&format!("render_sub_template: {}", &placeholder));
        match template_name {
            "template_crate_summary" => {
                // eprintln!("template_all_summaries: {}", "");
                let sub_template = unwrap!(sub_templates
                    .iter()
                    .find(|&template| template.name == template_name));
                let mut nodes = vec![];
                // sub-template repeatable
                for cursor_for_order_by_crate in 0..self.order_by_crate.len() {
                    let vec_node = unwrap!(self.render_template_raw_to_nodes(
                        &sub_template.template,
                        HtmlOrSvg::Html,
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
