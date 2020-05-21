//! crate_reviews_mod

use crate::crate_version_summary_mod::*;
use crate::duration_mod;
use crate::html_template_mod::*;
use crate::review_mod::*;
use crate::utils_mod::*;
use crate::*;

use chrono::Local;
use dirs;
use std::fs;
use unwrap::unwrap;

pub struct CrateReviews {
    pub crate_version_summary: CrateVersionSummary,
    pub reviews: Vec<Review>,
}

impl CrateReviews {
    pub fn new(crate_name: &str, version: &str, kind: &str) -> CrateReviews {
        let start = duration_mod::start_ns();
        eprintln!(
            "{}: crate_name: '{}', version '{}', kind '{}'",
            &Local::now().format("%Y-%m-%d %H:%M:%S"),
            Green.paint(crate_name),
            Green.paint(version),
            Green.paint(kind)
        );

        // first fill a vector with reviews, because I need to filter and sort them
        let mut reviews = query_reviews(crate_name);
        let before_sum_and_filter =
            duration_mod::eprint_duration_ns("  after query_reviews()", start);

        // the summary is always from all reviews. We must filter the reviews later.
        let crate_version_summary = CrateVersionSummary::new(crate_name, &reviews);
        filter_reviews(&mut reviews, version, kind);

        //return
        CrateReviews {
            crate_version_summary,
            reviews,
        }
    }
}

/// crev query returns html
fn query_reviews(crate_name: &str) -> Vec<Review> {
    // first fill a vector with reviews, because I need to filter and sort them
    let mut reviews = vec![];
    // this part can be cached: last 10 queried crates

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
        // for filename_result in unwrap!(glob("/reviews/*.crev")) {
        // read crev file
        let crev_text = unwrap!(fs::read_to_string(filename_crev));
        for part1 in crev_text.split("----- END CREV PROOF -----") {
            let start_delimiter = "----- BEGIN CREV PROOF -----";
            if let Some(start_pos) = part1.find(start_delimiter) {
                let start_pos = start_pos + start_delimiter.len() + 1;
                if let Some(end_pos) = part1.find("----- SIGN CREV PROOF -----") {
                    let proof_string = &part1[start_pos..end_pos];
                    push_proof(proof_string, &mut reviews, &crate_name);
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
                    push_proof(proof_string, &mut reviews, &crate_name);
                }
            }
        }
    }
    // eprintln!("files queried: {}", count_files);
    // sort first by version desc, but semver version and then by date
    reviews.sort_by(|a, b| {
        b.package
            .version_for_sorting
            .cmp(&a.package.version_for_sorting)
    });
    // return
    reviews
}

fn filter_reviews(reviews: &mut Vec<Review>, version: &str, kind: &str) {
    if !version.is_empty() && version != "crate" {
        reviews.retain(|x| x.package.version == version);
    }
    if !kind.is_empty() && kind != "c" {
        // strong
        if kind == "S" {
            reviews.retain(|x| {
                x.review.is_some() && x.review.as_ref().unwrap().rating == Rating::Strong
            });
        } else if kind == "P" {
            reviews.retain(|x| {
                x.review.is_some() && x.review.as_ref().unwrap().rating == Rating::Positive
            });
        } else if kind == "E" {
            reviews.retain(|x| {
                x.review.is_some() && x.review.as_ref().unwrap().rating == Rating::Neutral
            });
        } else if kind == "N" {
            reviews.retain(|x| {
                x.review.is_some() && x.review.as_ref().unwrap().rating == Rating::Negative
            });
        } else if kind == "v" {
            reviews.retain(|x| x.alternatives.is_some());
        } else if kind == "i" {
            reviews.retain(|x| x.issues.is_some());
        } else if kind == "a" {
            reviews.retain(|x| x.advisories.is_some() || x.advisory.is_some());
        }
    }
}

fn push_proof(proof_string: &str, reviews: &mut Vec<Review>, crate_name: &str) {
    let mut proof: Review = unwrap!(serde_yaml::from_str(proof_string));
    // filter: only one crate_name
    if &proof.package.name == crate_name {
        // reviews without review are not important
        // version for sorting
        let (major, minor, patch) = parse_semver(&proof.package.version);
        proof.package.version_for_sorting = Some(proof.version_for_sorting());
        Some(format!(
            "{:09}.{:09}.{:09}-{}",
            major,
            minor,
            patch,
            proof.get_author()
        ));
        reviews.push(proof);
    }
}

impl HtmlTemplatingRender for CrateReviews {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        //return
        "CrateReviews".to_string()
    }
    /// render full html
    fn render_html_file(&self, templates_folder_name: &str) -> String {
        let template_file_name = format!("{}query/crev_query_template.html", templates_folder_name);
        let html = self.render_from_file(&template_file_name);
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
        sub_templates: &Vec<SubTemplate>,
    ) -> Vec<Node> {
        // eprintln!("{}",&format!("render_sub_template: {}", &placeholder));
        match template_name {
            "template_crate_version_summary" => {
                // eprintln!("template_crate_version_summary: {}", "");
                let sub_template = unwrap!(sub_templates
                    .iter()
                    .find(|&template| template.name == template_name));
                let mut nodes = vec![];
                // sub-template NOT repeatable
                let vec_node = unwrap!(self.crate_version_summary.render_template_raw_to_nodes(
                    &sub_template.template,
                    HtmlOrSvg::Html,
                    0
                ));
                nodes.extend_from_slice(&vec_node);
                // return
                nodes
            }
            "template_review_proof" => {
                // eprintln!("template_review_proof: {}", "");
                let sub_template = unwrap!(sub_templates
                    .iter()
                    .find(|&template| template.name == template_name));
                let mut nodes = vec![];
                // sub-template repeatable
                for proof in &self.reviews {
                    let vec_node = unwrap!(proof.render_template_raw_to_nodes(
                        &sub_template.template,
                        HtmlOrSvg::Html,
                        0
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