//! crate_reviews_mod
use crate::crate_version_summary_mod::*;
use crate::html_server_template_mod::*;
use crate::review_mod::*;
use crate::utils_mod::*;
use crate::data_file_scan_mod::*;
use crate::*;

use unwrap::unwrap;

pub struct CrateReviews {
    pub crate_version_summary: CrateVersionSummary,
    pub reviews: Vec<Review>,
}

impl CrateReviews {
    pub fn new(cached_review_index: CachedReviewIndex, crate_name: &str, version: &str, kind: &str) -> CrateReviews {
        let ns_start = ns_start("");
        let review_index = cached_review_index
            .lock()
            .expect("error cached_review_index.lock()");
        // sort data by file_path
        // the data is sorted by path_file in ReviewIndex.new()
        // nobody else should sort the data
        // search data in the index
        let mut many_file = ManyFileReviewsPk { vec: vec![] };
        let mut old_file_path = s!("");
        let mut one_file = OneFileReviewsPk {
            file_path: s!("don't push the first row"),
            reviews_pk: Some(vec![]),
        };
        for index_item in review_index.vec.iter() {
            if index_item.crate_name == crate_name {
                if index_item.file_path != old_file_path {
                    old_file_path = index_item.file_path.clone();
                    if &one_file.file_path == "don't push the first row" {
                        // only once read
                        // but don't push the dummy
                    } else {
                        // push the old one before creating the new one
                        many_file.vec.push(one_file);
                    }
                    // create new OneFile
                    one_file = OneFileReviewsPk {
                        file_path: index_item.file_path.clone(),
                        reviews_pk: Some(vec![]),
                    };
                }
                // add data to reviews_pk
                unwrap!(one_file.reviews_pk.as_mut()).push(ReviewPk {
                    crate_name: index_item.crate_name.clone(),
                    author_id: index_item.author_id.clone(),
                    version: index_item.version.clone(),
                });
            }
        }
        // save the last file in the loop
        if &one_file.file_path != "don't push the first row" {
            // push the last one 
            many_file.vec.push(one_file.clone());
        }
        else{
            //remove the dummy 
            many_file.vec.pop();
        }
        let ns_read_from_index = ns_print(
            &format!("read from index, file_path count: {}", many_file.vec.len()),
            ns_start,
        );
        let mut reviews = get_vec_of_selected_reviews(many_file);
        ns_print(
            &format!("read from files reviews.len(): {}", reviews.len()),
            ns_read_from_index,
        );
        // sort reviews by version
        reviews.sort_by(|a, b| {
            b.package
                .version_for_sorting
                .cmp(&a.package.version_for_sorting)
        });

        // the summary is always from all reviews. We must filter the reviews later.
        let crate_version_summary = CrateVersionSummary::new(&crate_name, &reviews);
        filter_reviews(&mut reviews, version, kind);

        // return
        CrateReviews {
            crate_version_summary,
            reviews,
        }
    }
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
