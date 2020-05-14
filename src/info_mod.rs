//! info_mod

use crate::duration_mod;
use crate::html_template_mod::*;
use chrono::Local;

struct InfoData {
    number_of_reviews: usize,
    number_of_authors: usize,
}

/// info about reviews
pub fn html_for_info(templates_folder_name: &str) -> String {
    let start = duration_mod::start_ns();
    eprintln!("{}: info_mod", &Local::now().format("%Y-%m-%d %H:%M:%S"),);

    // count the proofs and their numeric values
    let info_data = proofs_info();
    let before_render = duration_mod::eprint_duration_ns("  after proofs_info()", start);

    let template_file_name = format!("{}info_template.html", templates_folder_name);
    let html = info_data.render_from_file(&template_file_name);

    duration_mod::eprint_duration_ns("  render", before_render);
    duration_mod::eprint_duration_ns("html_for_info()", start);
    // return
    html
}

fn proofs_info() -> InfoData {
    let info_data = InfoData {
        number_of_reviews: 0,
        number_of_authors: 0,
    };
    //return
    info_data
}

impl HtmlTemplatingRender for InfoData {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        //return
        "InfoData".to_string()
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
            // the href for css is good for static data. For dynamic route it must be different.
            "t_number_of_reviews" => self.number_of_reviews.to_string(),
            "t_number_of_authors" => self.number_of_authors.to_string(),
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
            _ => render_sub_template_match_else(&self.data_model_name(), template_name),
        }
    }
}
