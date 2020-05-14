//! info_mod

use crate::duration_mod;
use crate::html_template_mod::*;
use crate::proof_mod::*;
use crate::utils_mod::*;

use chrono::Local;
use std::{fs, };
use unwrap::unwrap;

#[derive( Clone,Debug)]
pub struct ProofIndexItem {
    crate_name: String,
    version: String,
    version_for_sorting: String,
    author: String,
    repo: String,
    file_path: String,
    rating_strong: usize,
    rating_positive: usize,
    rating_neutral: usize,
    rating_negative: usize,
    rating_none: usize,
    alternatives: usize,
    issues: usize,
    advisories: usize,
}
#[derive( Clone,Debug)]
struct InfoData {
    number_of_reviews: usize,
    number_of_authors: usize,
    proofs_index:Vec<ProofIndexItem>,
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
        proofs_index:prepare_proof_index(),
    };
    // eprintln!("proofs_index: {:#?}",info_data.proofs_index);
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
            "t_css_href" => "/cargo_crev_web/css/cargo_crev_web.css".to_string(),
            "t_favicon_href" => "/cargo_crev_web/favicon.png".to_string(),
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
        _sub_templates: &Vec<SubTemplate>,
    ) -> Vec<Node> {
        // eprintln!("{}",&format!("render_sub_template: {}", &placeholder));
        match template_name {
            _ => render_sub_template_match_else(&self.data_model_name(), template_name),
        }
    }
}

// iterating in the original file format is not performant
// it is better to read the files one time and make an index of all
// and then mostly use this index from memory.
// this index is created every time the web app is initialized
// or manually when the new and updated files are fetched
pub fn prepare_proof_index() -> Vec<ProofIndexItem> {
    let mut proofs_index: Vec<ProofIndexItem> = vec![];
    // original cache crev folder: /home/luciano/.cache/crev/remotes
    // on the google vm bestia02: /home/luciano_bestia/.cache/crev/remotes
    // local webfolder example "crev/cache/crev/remotes"
    let path = unwrap!(dirs::home_dir());
    let path = path.join(".cache/crev/remotes");
    // eprintln!("path: {}", path.display());
    // let mut count_files = 0;
    for filename_crev in &unwrap!(traverse_dir_with_exclude_dir(
        &path,
        "/*.crev",
        // avoid big folders and other folders with *.crev
        &vec!["/.git".to_string(), "/trust".to_string()]
    )) {
        //count_files += 1;
        // eprintln!("filename_crev: {}", filename_crev);
        // for filename_result in unwrap!(glob("/proofs/*.crev")) {
        // read crev file
        let crev_text = unwrap!(fs::read_to_string(filename_crev));
        for part1 in crev_text.split("----- END CREV PROOF -----") {
            let start_delimiter = "----- BEGIN CREV PROOF -----";
            if let Some(start_pos) = part1.find(start_delimiter) {
                let start_pos = start_pos + start_delimiter.len() + 1;
                if let Some(end_pos) = part1.find("----- SIGN CREV PROOF -----") {
                    let proof_string = &part1[start_pos..end_pos];
                    push_proof_index(proof_string, &mut proofs_index, filename_crev);
                }
            }
        }
        // older review has different delimiter. Everything else is the same.
        for part1 in crev_text.split("-----END CREV PACKAGE REVIEW-----") {
            let start_delimiter = "-----BEGIN CREV PACKAGE REVIEW-----";
            if let Some(start_pos) = part1.find(start_delimiter) {
                let start_pos = start_pos + start_delimiter.len() + 1;
                if let Some(end_pos) = part1.find("-----BEGIN CREV PACKAGE REVIEW SIGNATURE-----") {
                    let proof_string = &part1[start_pos..end_pos];
                    push_proof_index(proof_string, &mut proofs_index, filename_crev);
                }
            }
        }
    }
    //return
    proofs_index
}

fn push_proof_index(proof_string: &str, proofs_index: &mut Vec<ProofIndexItem>, file_path: &str) {
    let proof: crate::proof_mod::Proof = unwrap!(serde_yaml::from_str(proof_string));
    // use only some of the data for the index
    let proof_index_item = ProofIndexItem {
        crate_name: proof.package.name.to_string(),
        version: proof.package.version.to_string(),
        version_for_sorting: proof.version_for_sorting(),
        author: proof.get_author(),
        repo: proof.from.url.to_string(),
        file_path: file_path.to_string(),
        rating_strong: conditional_usize(proof.get_rating() == Rating::Strong, 1, 0),
        rating_positive: conditional_usize(proof.get_rating() == Rating::Positive, 1, 0),
        rating_neutral: conditional_usize(proof.get_rating() == Rating::Neutral, 1, 0),
        rating_negative: conditional_usize(proof.get_rating() == Rating::Negative, 1, 0),
        rating_none: conditional_usize(proof.get_rating() == Rating::None, 1, 0),
        // TODO:
        alternatives: 0,
        issues: 0,
        advisories: 0,
    };
    proofs_index.push(proof_index_item);
}
