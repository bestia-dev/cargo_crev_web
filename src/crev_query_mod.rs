//! crev_query_mod

use crate::all_summary_mod::*;
use crate::duration_mod;
use crate::html_template_mod::*;
use crate::proof_mod::*;
use crate::utils_mod::*;
use crate::*;

use chrono::Local;
use dirs;
use std::fs;
use unwrap::unwrap;

pub struct CrevQueryData {
    pub all_summaries: AllSummaries,
    pub proofs: Vec<Proof>,
}

impl CrevQueryData {
    pub fn new(crate_name: &str, version: &str, kind: &str) -> CrevQueryData {
        let start = duration_mod::start_ns();
        eprintln!(
            "{}: crate_name: '{}', version '{}', kind '{}'",
            &Local::now().format("%Y-%m-%d %H:%M:%S"),
            Green.paint(crate_name),
            Green.paint(version),
            Green.paint(kind)
        );

        // first fill a vector with proofs, because I need to filter and sort them
        let mut proofs = proofs_crev_query(crate_name);
        let before_sum_and_filter =
            duration_mod::eprint_duration_ns("  after proofs_crev_query()", start);

        // the summary is always from all proofs. We must filter the proofs later.
        let all_summaries = AllSummaries::new(crate_name, &proofs);
        filter_proofs(&mut proofs, version, kind);

        //return
        CrevQueryData {
            all_summaries,
            proofs,
        }
    }
}

/// crev query returns html
fn proofs_crev_query(crate_name: &str) -> Vec<Proof> {
    // first fill a vector with proofs, because I need to filter and sort them
    let mut proofs = vec![];
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
        // for filename_result in unwrap!(glob("/proofs/*.crev")) {
        // read crev file
        let crev_text = unwrap!(fs::read_to_string(filename_crev));
        for part1 in crev_text.split("----- END CREV PROOF -----") {
            let start_delimiter = "----- BEGIN CREV PROOF -----";
            if let Some(start_pos) = part1.find(start_delimiter) {
                let start_pos = start_pos + start_delimiter.len() + 1;
                if let Some(end_pos) = part1.find("----- SIGN CREV PROOF -----") {
                    let proof_string = &part1[start_pos..end_pos];
                    push_proof(proof_string, &mut proofs, &crate_name);
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
                    push_proof(proof_string, &mut proofs, &crate_name);
                }
            }
        }
    }
    // eprintln!("files queried: {}", count_files);
    // sort first by version desc, but semver version and then by date
    proofs.sort_by(|a, b| {
        b.package
            .version_for_sorting
            .cmp(&a.package.version_for_sorting)
    });
    // return
    proofs
}

fn filter_proofs(proofs: &mut Vec<Proof>, version: &str, kind: &str) {
    if !version.is_empty() && version != "crate" {
        proofs.retain(|x| x.package.version == version);
    }
    if !kind.is_empty() && kind != "c" {
        // strong
        if kind == "S" {
            proofs.retain(|x| {
                x.review.is_some() && x.review.as_ref().unwrap().rating == Rating::Strong
            });
        } else if kind == "P" {
            proofs.retain(|x| {
                x.review.is_some() && x.review.as_ref().unwrap().rating == Rating::Positive
            });
        } else if kind == "E" {
            proofs.retain(|x| {
                x.review.is_some() && x.review.as_ref().unwrap().rating == Rating::Neutral
            });
        } else if kind == "N" {
            proofs.retain(|x| {
                x.review.is_some() && x.review.as_ref().unwrap().rating == Rating::Negative
            });
        } else if kind == "v" {
            proofs.retain(|x| x.alternatives.is_some());
        } else if kind == "i" {
            proofs.retain(|x| x.issues.is_some());
        } else if kind == "a" {
            proofs.retain(|x| x.advisories.is_some() || x.advisory.is_some());
        }
    }
}

fn push_proof(proof_string: &str, proofs: &mut Vec<Proof>, crate_name: &str) {
    let mut proof: Proof = unwrap!(serde_yaml::from_str(proof_string));
    // filter: only one crate_name
    if &proof.package.name == crate_name {
        // proofs without review are not important
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
        proofs.push(proof);
    }
}

impl HtmlTemplatingRender for CrevQueryData {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        //return
        "CrevQueryData".to_string()
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
            "template_all_summaries" => {
                // eprintln!("template_all_summaries: {}", "");
                let sub_template = unwrap!(sub_templates
                    .iter()
                    .find(|&template| template.name == template_name));
                let mut nodes = vec![];
                // sub-template NOT repeatable
                let vec_node = unwrap!(self.all_summaries.render_template_raw_to_nodes(
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
                for proof in &self.proofs {
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
