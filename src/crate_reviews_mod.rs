//! crate_reviews_mod
use crate::crate_version_summary_mod::*;
use crate::html_server_template_mod::*;
use crate::review_mod::*;
use crate::utils_mod::*;
use crate::*;

use dirs;
use std::fs;
use unwrap::unwrap;

pub struct CrateReviews {
    pub crate_version_summary: CrateVersionSummary,
    pub reviews: Vec<Review>,
}

impl CrateReviews {
    pub fn new(crate_name: &str, version: &str, kind: &str) -> CrateReviews {
        // first fill a vector with reviews, because I need to filter and sort them
        let mut reviews = get_crate_reviews(crate_name);

        // the summary is always from all reviews. We must filter the reviews later.
        let crate_version_summary = CrateVersionSummary::new(crate_name, &reviews);
        filter_reviews(&mut reviews, version, kind);

         // return
        CrateReviews {
            crate_version_summary,
            reviews,
        }
    }
}

/// crev crate returns html
fn get_crate_reviews(crate_name: &str) -> Vec<Review> {
    // first fill a vector with reviews, because I need to filter and sort them
    let mut reviews = vec![];
    // this part can be cached: last 10 queried crates

    // original cache crev folder: /home/luciano/.cache/crev/remotes
    // on the google vm bestia02: /home/luciano_bestia/.cache/crev/remotes
    // local webfolder example "../sample_data/cache/crev/remotes"
    let path = unwrap!(dirs::home_dir());
    let path = path.join(".cache/crev/remotes");
    // dbg!(path);
    // let mut count_files = 0;
    for filename_crev in &unwrap!(traverse_dir_with_exclude_dir(
        &path,
        "/*.crev",
        // avoid big folders and other folders with *.crev
        &vec![s!("/.git"), s!("/trust")]
    )) {
         // count_files += 1;
        // dbg!(filename_crev);
        // for filename_result in unwrap!(glob("/reviews/*.crev")) {
        // read crev file
        let crev_text = unwrap!(fs::read_to_string(filename_crev));
        for part1 in crev_text.split("----- END CREV PROOF -----") {
            let start_delimiter = "----- BEGIN CREV PROOF -----";
            if let Some(start_pos) = part1.find(start_delimiter) {
                let start_pos = start_pos + start_delimiter.len() + 1;
                if let Some(end_pos) = part1.find("----- SIGN CREV PROOF -----") {
                    let review_string = &part1[start_pos..end_pos];
                    push_review(review_string, &mut reviews, &crate_name);
                }
            }
        }
        // older review has different delimiter. Everything else is the same.
        for part1 in crev_text.split("-----END CREV PACKAGE REVIEW-----") {
            let start_delimiter = "-----BEGIN CREV PACKAGE REVIEW-----";
            if let Some(start_pos) = part1.find(start_delimiter) {
                let start_pos = start_pos + start_delimiter.len() + 1;
                if let Some(end_pos) = part1.find("-----BEGIN CREV PACKAGE REVIEW SIGNATURE-----") {
                    let review_string = &part1[start_pos..end_pos];
                    push_review(review_string, &mut reviews, &crate_name);
                }
            }
        }
    }
    // dbg!(count_files);
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

fn push_review(review_string: &str, reviews: &mut Vec<Review>, crate_name: &str) {
    let mut review: Review = unwrap!(serde_yaml::from_str(review_string));
    // filter: only one crate_name
    if &review.package.name == crate_name {
        // reviews without review are not important
        // version for sorting
        let (major, minor, patch) = parse_semver(&review.package.version);
        review.package.version_for_sorting = Some(review.version_for_sorting());
        Some(format!(
            "{:09}.{:09}.{:09}-{}",
            major,
            minor,
            patch,
            review.get_author_name()
        ));
        reviews.push(review);
    }
}

impl HtmlServerTemplateRender for CrateReviews {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
         // return
        s!("CrateReviews")
    }
    /// renders the complete html file. Not a sub-template/fragment.
    fn render_html_file(&self, templates_folder_name: &str) -> String {
        let template_file_name =
            format!("{}crate/crate_reviews_template.html", templates_folder_name);
        let html = self.render_from_file(&template_file_name);

        // return
        html
    }
    /// boolean : is the next node rendered or not
    fn retain_next_node(&self, placeholder: &str) -> bool {
        // dbg!(&placeholder));
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
        _pos_cursor: usize,
    ) -> String {
        // dbg!( &placeholder);
        match placeholder {
            // the href for css is good for static data. For dynamic route it must be different.
            "st_css_route" => s!("/cargo_crev_web/css/cargo_crev_web.css"),
            "st_favicon_route" => s!("/cargo_crev_web/favicon.png"),
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
    /// renders sub-template
    #[allow(clippy::needless_return)]
    fn render_sub_template(
        &self,
        template_name: &str,
        sub_templates: &Vec<SubTemplate>,
    ) -> Vec<Node> {
        // dbg!(&placeholder);
        match template_name {
            "stmplt_crate_version_summary" => {
                let sub_template = unwrap!(sub_templates
                    .iter()
                    .find(|&template| template.name == template_name));
                let mut nodes = vec![];
                // sub-template NOT repeatable
                let vec_node = unwrap!(self.crate_version_summary.render_template_raw_to_nodes(
                    &sub_template.template,
                    HtmlOrSvg::Html,
                    "",
                    0
                ));
                nodes.extend_from_slice(&vec_node);
                // return
                nodes
            }
            "stmplt_reviews" => {
                let sub_template = unwrap!(sub_templates
                    .iter()
                    .find(|&template| template.name == template_name));
                let mut nodes = vec![];
                // sub-template repeatable
                for review in &self.reviews {
                    let vec_node = unwrap!(review.render_template_raw_to_nodes(
                        &sub_template.template,
                        HtmlOrSvg::Html,
                        "",
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
