//! crate_version_summary_mod

// region: use
use crate::review_mod::*;
use crate::version_summary_mod::VersionSummary;
use crate::*;

//use serde_derive::{Deserialize, Serialize};
//use std::fs;
use unwrap::unwrap;
// endregion: use

#[derive(Clone, Debug)]
pub struct CrateVersionSummary {
    pub crate_name: String,
    pub crate_summary: VersionSummary,
    pub version_summaries: Vec<VersionSummary>,
    //pub last_version: String,
}
impl CrateVersionSummary {
    pub fn new(_state_global: ArcMutStateGlobal, crate_name: &str, reviews: &Vec<Review>) -> Self {
        // the first version empty_string is for "all_versions" or crate_summary

        //let last_version = unwrap!(state_global.lock()).crate_index.get_last_version(crate_name);

        let mut crate_version_summary = CrateVersionSummary {
            crate_name: s!(crate_name),
            version_summaries: vec![],
            crate_summary: VersionSummary {
                crate_name: s!(crate_name),
                version: String::new(),
                version_for_sorting: String::new(),
                last_reviewed_version: String::new(),
                review_number: 0,
                rating_strong: 0,
                rating_positive: 0,
                rating_neutral: 0,
                rating_negative: 0,
                rating_none: 0,
                alternatives: 0,
                issues: 0,
                advisories: 0,
                thoroughness: 0,
                understanding: 0,
            },
            //last_version,
        };

        for review in reviews {
            // find version in vector or create new
            let mut option_version: Option<&mut VersionSummary> = None;
            for version_summary in &mut crate_version_summary.version_summaries {
                if version_summary.version == review.package.version {
                    option_version = Some(version_summary);
                    break;
                }
            }
            if option_version.is_none() {
                // new element
                let mut version_to_push = VersionSummary::new();
                version_to_push.crate_name = s!(crate_name);
                version_to_push.version = s!(&review.package.version);
                version_to_push.version_for_sorting = s!(unwrap!(review.package.version_for_sorting.clone()));
                crate_version_summary.version_summaries.push(version_to_push);
                option_version = Some(unwrap!(crate_version_summary.version_summaries.last_mut()));
            }
            // Here Option is not needed any more.
            let mut version_summary = unwrap!(option_version);
            crate_version_summary.crate_summary.review_number += 1;
            version_summary.review_number += 1;

            if let Some(review) = &review.review {
                if review.rating == Rating::Strong {
                    crate_version_summary.crate_summary.rating_strong += 1;
                    version_summary.rating_strong += 1;
                }
                if review.rating == Rating::Positive {
                    crate_version_summary.crate_summary.rating_positive += 1;
                    version_summary.rating_positive += 1;
                }
                if review.rating == Rating::Neutral {
                    crate_version_summary.crate_summary.rating_neutral += 1;
                    version_summary.rating_neutral += 1;
                }
                if review.rating == Rating::Negative {
                    crate_version_summary.crate_summary.rating_negative += 1;
                    version_summary.rating_negative += 1;
                }
                if review.rating == Rating::None {
                    crate_version_summary.crate_summary.rating_none += 1;
                    version_summary.rating_none += 1;
                }
                if review.thoroughness == Level::High {
                    crate_version_summary.crate_summary.thoroughness += 2;
                    version_summary.thoroughness += 2;
                }
                if review.thoroughness == Level::Medium {
                    crate_version_summary.crate_summary.thoroughness += 1;
                    version_summary.thoroughness += 1;
                }
                if review.understanding == Level::High {
                    crate_version_summary.crate_summary.understanding += 2;
                    version_summary.understanding += 2;
                }
                if review.understanding == Level::Medium {
                    crate_version_summary.crate_summary.understanding += 1;
                    version_summary.understanding += 1;
                }
            } else {
                crate_version_summary.crate_summary.rating_none += 1;
                version_summary.rating_none += 1;
            }
            if let Some(_alternative) = &review.alternatives {
                crate_version_summary.crate_summary.alternatives += 1;
                version_summary.alternatives += 1;
            }
            if let Some(_issue) = &review.issues {
                crate_version_summary.crate_summary.issues += 1;
                version_summary.issues += 1;
            }
            if let Some(_advisory) = &review.advisories {
                crate_version_summary.crate_summary.advisories += 1;
                version_summary.advisories += 1;
            }
            if let Some(_advisory) = &review.advisory {
                crate_version_summary.crate_summary.advisories += 1;
                version_summary.advisories += 1;
            }
        }
        //dbg!(&crate_version_summary.crate_summary.last_reviewed_version);
        // find last version - with review
        crate_version_summary.crate_summary.last_reviewed_version = crate_version_summary
            .version_summaries
            .iter()
            .max_by(|a, b| a.version_for_sorting.cmp(&b.version_for_sorting))
            .unwrap()
            .version
            .clone();
        // return
        crate_version_summary
    }
}

impl HtmlServerTemplateRender for CrateVersionSummary {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        // return
        s!("CrateVersionSummary")
    }
    /// renders the complete html file. Not a sub-template/fragment.
    fn render_html_file(&self, _templates_folder_name: &str) -> String {
        // return
        String::new()
    }
    /// boolean : is the next node rendered or not
    fn retain_next_node_or_attribute(&self, placeholder: &str) -> bool {
        // dbg!(&placeholder);
        match placeholder {
            _ => retain_next_node_or_attribute_match_else(&self.data_model_name(), placeholder),
        }
    }

    /// returns a String to replace the next text-node
    #[allow(clippy::needless_return, clippy::integer_arithmetic, clippy::indexing_slicing)]
    fn replace_with_string(&self, placeholder: &str, _subtemplate: &str, _pos_cursor: usize) -> String {
        // dbg!(&placeholder);
        match placeholder {
            "st_crate_name" => s!(&self.crate_name),
            "st_cargo_toml_dependency" => s!(r#"{} = "{}""#, &self.crate_name, &self.crate_summary.last_reviewed_version),
            //"st_last_version" => s!(&self.last_version),
            "st_lib_rs_url" => s!("https://lib.rs/crates/{}/", &self.crate_name),
            "st_crate_review_number" => url_s_zero_to_empty(self.crate_summary.review_number),
            "st_crate_rating_strong" => url_s_zero_to_empty(self.crate_summary.rating_strong),
            "st_crate_rating_positive" => url_s_zero_to_empty(self.crate_summary.rating_positive),
            "st_crate_rating_neutral" => url_s_zero_to_empty(self.crate_summary.rating_neutral),
            "st_crate_rating_negative" => url_s_zero_to_empty(self.crate_summary.rating_negative),
            "st_crate_rating_none" => url_s_zero_to_empty(self.crate_summary.rating_none),
            "st_crate_alternatives" => url_s_zero_to_empty(self.crate_summary.alternatives),
            "st_crate_issues" => url_s_zero_to_empty(self.crate_summary.issues),
            "st_crate_advisories" => url_s_zero_to_empty(self.crate_summary.advisories),
            "st_crate_thoroughness" => url_s_zero_to_empty(self.crate_summary.thoroughness),
            "st_crate_understanding" => url_s_zero_to_empty(self.crate_summary.understanding),

            _ => replace_with_string_match_else(&self.data_model_name(), placeholder),
        }
    }
    /// exclusive url encoded for href and src
    fn replace_with_url(&self, placeholder: &str, _subtemplate: &str, _pos_cursor: usize) -> UrlUtf8EncodedString {
        // dbg!( &placeholder);
        match placeholder {
            // the href for css is good for static data. For dynamic route it must be different.
            "su_crates_io_url" => url_u!("https://crates.io/crates/{}/", &self.crate_name),
            "su_lib_rs_url" => url_u!("https://lib.rs/crates/{}/", &self.crate_name),
            "su_docs_rs_url" => url_u!("https://docs.rs/{}/", &self.crate_name),
            "su_img_src_logo" => url_u!("/rust-reviews/images/Logo_02.png"),
            "su_your_personal_reviews" => {
                if self.crate_name.is_empty() && self.crate_summary.last_reviewed_version.is_empty() {
                    url_u!("/rust-reviews/your_personal_reviews/")
                } else if self.crate_summary.last_reviewed_version.is_empty() {
                    url_u!("/rust-reviews/your_personal_reviews/{}/", &self.crate_name)
                } else {
                    url_u!(
                        "/rust-reviews/your_personal_reviews/{}/{}/",
                        &self.crate_name,
                        &self.crate_summary.last_reviewed_version
                    )
                }
            }
            "su_filter_crate" => url_u!("/rust-reviews/crate/{}", &self.crate_name),
            "su_filter_strong" => url_u!("/rust-reviews/crate/{}/crate/S", &self.crate_name),
            "su_filter_positive" => url_u!("/rust-reviews/crate/{}/crate/P", &self.crate_name),
            "su_filter_neutral" => url_u!("/rust-reviews/crate/{}/crate/E", &self.crate_name),
            "su_filter_negative" => url_u!("/rust-reviews/crate/{}/crate/N", &self.crate_name),
            "su_filter_none" => url_u!("/rust-reviews/crate/{}/crate/0", &self.crate_name),
            "su_filter_alternatives" => url_u!("/rust-reviews/crate/{}/crate/v", &self.crate_name),
            "su_filter_issues" => url_u!("/rust-reviews/crate/{}/crate/i", &self.crate_name),
            "su_filter_advisories" => url_u!("/rust-reviews/crate/{}/crate/a", &self.crate_name),
            //"su_new_review" => url_u!("/rust-reviews/your_personal_reviews/{}/{}/",&self.crate_name,&self.last_version),
            "su_new_review" => url_u!("/rust-reviews/your_personal_reviews/{}/", &self.crate_name),
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
    fn render_sub_template(&self, template_name: &str, sub_templates: &Vec<SubTemplate>) -> Vec<Node> {
        // dbg!( &sub_templates.len());

        match template_name {
            "stmplt_summary_version" => {
                let sub_template = unwrap!(sub_templates.iter().find(|&template| template.name == template_name));
                let mut nodes = vec![];
                for version_summary in &self.version_summaries {
                    let vec_node = unwrap!(version_summary.render_template_raw_to_nodes(&sub_template.template, HtmlOrSvg::Html, "", 0));
                    nodes.extend_from_slice(&vec_node);
                }
                // return
                nodes
            }
            _ => render_sub_template_match_else(&self.data_model_name(), template_name),
        }
    }
}
