//! proof_index_mod

use crate::duration_mod;
use crate::html_template_mod::*;
use crate::proof_mod::*;
use crate::utils_mod::*;

use chrono::Local;
use std::fs;
use unwrap::unwrap;

#[derive(Clone, Debug)]
pub struct ProofIndexItem {
    pub crate_name: String,
    pub version: String,
    pub version_for_sorting: String,
    pub author: String,
    pub repo: String,
    pub file_path: String,
    pub rating_strong: usize,
    pub rating_positive: usize,
    pub rating_neutral: usize,
    pub rating_negative: usize,
    pub rating_none: usize,
    pub alternatives: usize,
    pub issues: usize,
    pub advisories: usize,
}
#[derive(Clone, Debug)]
pub struct InfoData {
    proofs_index: Vec<ProofIndexItem>,
    summary:IndexSummary,
}
#[derive(Clone, Debug)]
pub struct IndexSummary {
    pub unique_crates:usize,
    pub unique_authors:usize,
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

impl InfoData {
        /// prepares the data
        pub fn new() -> InfoData {
            // prepare fields one by one
            let proofs_index = Self::prepare_proofs_index();
            let summary=Self::create_summary(&proofs_index);
            // return struct with all fields
            InfoData {
                proofs_index,
                summary
            }
        }

    /// info about reviews
    pub fn render_html_file(&self, templates_folder_name: &str) -> String {
        let start = duration_mod::start_ns();
        eprintln!("{}: proof_index_mod", &Local::now().format("%Y-%m-%d %H:%M:%S"),);

        // count the proofs and their numeric values
        let before_render = duration_mod::eprint_duration_ns("  after prepare_proofs_index()", start);

        let template_file_name = format!("{}info_template.html", templates_folder_name);
        let html = self.render_from_file(&template_file_name);

        duration_mod::eprint_duration_ns("  render", before_render);
        duration_mod::eprint_duration_ns("render_html_file()", start);
        // return
        html
    }

    // iterating in the original file format is not performant
    // it is better to read the files one time and make an index of all
    // and then mostly use this index from memory.
    // this index is created every time the web app is initialized
    // or manually when the new and updated files are fetched

    /// todo: this could be cached for faster performance
    pub fn prepare_proofs_index() -> Vec<ProofIndexItem> {
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
                        Self::push_proof_index(proof_string, &mut proofs_index, filename_crev);
                    }
                }
            }
            // older review has different delimiter. Everything else is the same.
            for part1 in crev_text.split("-----END CREV PACKAGE REVIEW-----") {
                let start_delimiter = "-----BEGIN CREV PACKAGE REVIEW-----";
                if let Some(start_pos) = part1.find(start_delimiter) {
                    let start_pos = start_pos + start_delimiter.len() + 1;
                    if let Some(end_pos) =
                        part1.find("-----BEGIN CREV PACKAGE REVIEW SIGNATURE-----")
                    {
                        let proof_string = &part1[start_pos..end_pos];
                        Self::push_proof_index(proof_string, &mut proofs_index, filename_crev);
                    }
                }
            }
        }
        //return
        proofs_index
    }

    fn push_proof_index(
        proof_string: &str,
        proofs_index: &mut Vec<ProofIndexItem>,
        file_path: &str,
    ) {
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

            alternatives: {
                if let Some(alternatives) = proof.alternatives {
                    alternatives.len()
                } else {
                    0
                }
            },
            issues: {
                if let Some(issues) = proof.issues {
                    issues.len()
                } else {
                    0
                }
            },
            advisories: {
                if let Some(advisories) = proof.advisories {
                    advisories.len()
                } else {
                    0
                }
            },
        };
        proofs_index.push(proof_index_item);
    }
/// create summary
pub fn create_summary(proofs_index: & Vec<ProofIndexItem>) -> IndexSummary {

    let mut for_unique_crates: Vec<String> = vec![];
    let mut for_unique_authors: Vec<String> = vec![];
    let mut summary = IndexSummary {
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
    for index_item in proofs_index {
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

    //return
    summary
}
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
    fn call_fn_string(&self, placeholder: &str, _cursor_pos: usize) -> String {
        // eprintln!("{}",&format!("call_fn_string: {}", &placeholder));
        match placeholder {
            // the href for css is good for static data. For dynamic route it must be different.
            "t_css_href" => "/cargo_crev_web/css/cargo_crev_web.css".to_string(),
            "t_favicon_href" => "/cargo_crev_web/favicon.png".to_string(),
            "t_unique_crates" => self.summary.unique_crates.to_string(),
            "t_unique_authors" => self.summary.unique_authors.to_string(),
            "t_count_of_reviews" => self.summary.count_of_reviews.to_string(),
            "t_count_of_rating_strong" => self.summary.count_of_rating_strong.to_string(),
            "t_count_of_rating_positive" => self.summary.count_of_rating_positive.to_string(),
            "t_count_of_rating_neutral" => self.summary.count_of_rating_neutral.to_string(),
            "t_count_of_rating_negative" => self.summary.count_of_rating_negative.to_string(),
            "t_count_of_rating_none" => self.summary.count_of_rating_none.to_string(),
            "t_count_of_alternatives" => self.summary.count_of_alternatives.to_string(),
            "t_count_of_issues" => self.summary.count_of_issues.to_string(),
            "t_count_of_advisories" => self.summary.count_of_advisories.to_string(),
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
