//! reviewers_reviews_mod

use crate::data_file_scan_mod::*;
use crate::review_mod::*;
use crate::*;

use unwrap::unwrap;

//use unwrap::unwrap;
pub struct ReviewerReviews {
    pub reviewer_name: String,
    pub reviewer_url: String,
    pub reviewer_id: String,
    pub reviews: Vec<Review>,
}

impl ReviewerReviews {
    pub fn new(state_global: ArcMutStateGlobal, reviewer_id: &str) -> Self {
        let ns_start = ns_start("");
        // sort data by file_path
        // the data is sorted by path_file in ReviewIndex.new()
        // nobody else should sort the data
        // search data in the index
        let mut many_file = ManyFileReviewsPk { vec: vec![] };
        let mut old_file_path = s!();
        let mut one_file = OneFileReviewsPk {
            file_path: s!("don't push the first row"),
            reviews_pk: Some(vec![]),
        };
        let mut reviewer_name = s!();
        let mut reviewer_url = s!();
        for index_item in unwrap!(state_global.lock()).review_index.vec.iter() {
            if index_item.reviewer_id == reviewer_id {
                if index_item.file_path != old_file_path {
                    old_file_path = index_item.file_path.clone();
                    if &one_file.file_path == "don't push the first row" {
                        // only once read the reviewer_name
                        // but don't push the dummy
                        reviewer_name = index_item.reviewer_name.clone();
                        reviewer_url = index_item.reviewer_url.clone();
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
                    reviewer_id: index_item.reviewer_id.clone(),
                    version: index_item.version.clone(),
                });
            }
        }
        // save the last file in the loop
        if &one_file.file_path != "don't push the first row" {
            // push the last one
            many_file.vec.push(one_file.clone());
        } else {
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
        // sort reviews by crate and version
        reviews.sort_by(|a, b| {
            b.package
                .version_for_sorting
                .cmp(&a.package.version_for_sorting)
        });
        reviews.sort_by(|a, b| a.package.name.cmp(&b.package.name));
        // return
        ReviewerReviews {
            reviewer_name: reviewer_name,
            reviewer_url: reviewer_url,
            reviewer_id: s!(reviewer_id),
            reviews,
        }
    }
}

impl HtmlServerTemplateRender for ReviewerReviews {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        // return
        s!("ReviewerReviews")
    }
    /// renders the complete html file. Not a sub-template/fragment.
    fn render_html_file(&self, templates_folder_name: &str) -> String {
        let template_file_name = format!("{}reviewer_reviews_template.html", templates_folder_name);
        let html = self.render_from_file(&template_file_name);
        // return
        html
    }
    /// boolean : is the next node rendered or not
    fn retain_next_node_or_attribute(&self, placeholder: &str) -> bool {
        // dbg!(&placeholder));
        match placeholder {
            _ => retain_next_node_or_attribute_match_else(&self.data_model_name(), placeholder),
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
        // dbg!(&placeholder);
        match placeholder {
            "st_cargo_crev_web_version" => s!(env!("CARGO_PKG_VERSION")),
            "st_reviewer_name" => s!(&self.reviewer_name),
            "st_reviewer_id" => s!(&self.reviewer_id),
            "st_reviewer_url" => s!(&self.reviewer_url),
            "st_cmd_fetch" => s!("cargo crev repo fetch url {}", self.reviewer_url),
            "st_cmd_trust" => s!("cargo crev id trust {}", self.reviewer_id),
            _ => replace_with_string_match_else(&self.data_model_name(), placeholder),
        }
    }
    /// exclusive url encoded for href and src attribute values
    fn replace_with_url(
        &self,
        placeholder: &str,
        _subtemplate: &str,
        _pos_cursor: usize,
    ) -> UrlUtf8EncodedString {
        // dbg!(&placeholder);
        match placeholder {
            // the href for css is good for static data. For dynamic route it must be different.
            "su_css_route" => url_u!("/rust-reviews/css/rust-reviews.css"),
            "su_favicon_route" => url_u!("/rust-reviews/favicon.png"),
            "su_img_src_logo" => url_u!("/rust-reviews/images/Logo_02.png"),
            "su_reviewer_url" => url_u!(&self.reviewer_url, ""),
            "su_lib_rs_url" => url_u!("https://lib.rs/~{}", &self.reviewer_name),
            "su_crates_io_url" => url_u!("https://crates.io/users/{}", &self.reviewer_name),
            _ => replace_with_url_match_else(&self.data_model_name(), placeholder),
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
