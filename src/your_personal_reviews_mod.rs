//! review_mod

use crate::review_mod::*;
use crate::*;

use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
//use std::fs;
use unwrap::unwrap;
//use strum_macros::EnumString;
use std::str::FromStr;

/// simplified review
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ReviewForVim {
    pub review: ReviewReview,
    pub comment: String,
}
#[derive(Clone, Default)]
pub struct YourPersonalReviews {
    pub package_name: String,
    pub package_version: String,
    //pub cargo_toml_line: String,
    pub review_for_vim: ReviewForVim,
    pub yaml_text: String,
}

impl YourPersonalReviews {
    #[allow(unused)]
    /// prepares the data
    pub fn new(form_data: HashMap<String, String>) -> Self {
        let mut your_personal_reviews = YourPersonalReviews::default();
        // just copy form data into struct. Don't process it here.
        for (key, value) in form_data {
            match key.as_ref() {
                //"cargo_toml_line" => your_personal_reviews.cargo_toml_line = value,
                "package_name" => your_personal_reviews.package_name = value.to_string(),
                "package_version" => your_personal_reviews.package_version = value.to_string(),
                "thoroughness" => your_personal_reviews.review_for_vim.review.thoroughness = unwrap!(Level::from_str(&value)),
                "understanding" => your_personal_reviews.review_for_vim.review.understanding = unwrap!(Level::from_str(&value)),
                "rating" => your_personal_reviews.review_for_vim.review.rating = unwrap!(Rating::from_str(&value)),
                "comment" => your_personal_reviews.review_for_vim.comment = value.to_string(),
                _ => {}
            }
        }
        if your_personal_reviews.package_name.is_empty() {
            your_personal_reviews.package_name = "crate_name".into();
        }
        if your_personal_reviews.package_version.is_empty() {
            your_personal_reviews.package_version = "version".into();
        }
        // parse cargo_toml_line if it exist
        /*
        if !your_personal_reviews.cargo_toml_line.is_empty(){
            let (package_name, package_version) = Self::parse_cargo_toml_line(&your_personal_reviews.cargo_toml_line);
            your_personal_reviews.package_name = package_name;
            your_personal_reviews.package_version = package_version;
        }
        */
        your_personal_reviews.yaml_text = unwrap!(serde_yaml::to_string(&your_personal_reviews.review_for_vim));
        //return
        your_personal_reviews
    }

    pub fn new_from_get(crate_name: &str, version: &str) -> Self {
        let mut your_personal_reviews = YourPersonalReviews {
            package_name: crate_name.to_string(),
            package_version: version.to_string(),
            //cargo_toml_line:s!(),
            review_for_vim: ReviewForVim {
                review: ReviewReview {
                    thoroughness: Level::None,
                    understanding: Level::None,
                    rating: Rating::None,
                },
                comment: s!("Comment important aspects:  security:  unsafe:   FFI:     asm!:     file-read:    file-write:     network-access:    build.rs:   macro_rules:  proc_macro:  maintainers/authors:  used in your project:  alternatives you explored:     "),
            },
            yaml_text: s!(),
        };
        your_personal_reviews.yaml_text = unwrap!(serde_yaml::to_string(&your_personal_reviews.review_for_vim));

        //return
        your_personal_reviews
    }
    /*
    pub fn parse_cargo_toml_line(value: &str) -> (String, String) {
        // micro-ecc-sys = "0.2.0"
        let mut spl = value.split('=');
        let crate_name = spl.next().unwrap_or("").trim().to_string();
        println!("{}", crate_name);
        let version = spl
            .next()
            .unwrap_or("")
            .replace(r#"""#, "")
            .trim()
            .to_string();
        println!("{}", version);
        // return
        (crate_name, version)
    }
    */
}

impl HtmlServerTemplateRender for YourPersonalReviews {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        // return
        s!("YourPersonalReviews")
    }
    /// renders the complete html file. Not a sub-template/fragment.
    fn render_html_file(&self, templates_folder_name: &str) -> String {
        let template_file_name = format!("{}your_personal_reviews_template.html", templates_folder_name);
        let html = self.render_from_file(&template_file_name);

        // return
        html
    }
    /// boolean : is the next node rendered or not
    fn retain_next_node_or_attribute(&self, placeholder: &str) -> bool {
        // dbg!(&placeholder);
        match placeholder {
            /*
            "sb_has_alternative" => self.review_for_vim.alternatives.is_some(),
            "sb_has_issue" => self.review_for_vim.issues.is_some(),
            "sb_has_advisories" => self.review_for_vim.advisories.is_some(),
            */
            // radio buttons in html have this terrible attribute checked. Horror.
            "sb_thoroughness_none" => &self.review_for_vim.review.thoroughness.to_string() == "none",
            "sb_thoroughness_low" => &self.review_for_vim.review.thoroughness.to_string() == "low",
            "sb_thoroughness_medium" => &self.review_for_vim.review.thoroughness.to_string() == "medium",
            "sb_thoroughness_high" => &self.review_for_vim.review.thoroughness.to_string() == "high",
            "sb_understanding_none" => &self.review_for_vim.review.understanding.to_string() == "none",
            "sb_understanding_low" => &self.review_for_vim.review.understanding.to_string() == "low",
            "sb_understanding_medium" => &self.review_for_vim.review.understanding.to_string() == "medium",
            "sb_understanding_high" => &self.review_for_vim.review.understanding.to_string() == "high",
            "sb_rating_none" => &self.review_for_vim.review.rating.to_string() == "none",
            "sb_rating_negative" => &self.review_for_vim.review.rating.to_string() == "negative",
            "sb_rating_neutral" => &self.review_for_vim.review.rating.to_string() == "neutral",
            "sb_rating_positive" => &self.review_for_vim.review.rating.to_string() == "positive",
            "sb_rating_strong" => &self.review_for_vim.review.rating.to_string() == "strong",

            _ => retain_next_node_or_attribute_match_else(&self.data_model_name(), placeholder),
        }
    }

    /// returns a String to replace the next text-node
    #[allow(clippy::needless_return, clippy::integer_arithmetic, clippy::indexing_slicing)]
    fn replace_with_string(&self, placeholder: &str, _subtemplate: &str, _pos_cursor: usize) -> String {
        // dbg!(&placeholder);
        // list_trusted_reviewer_id is Option and can be None or Some
        match placeholder {
            "st_cargo_crev_web_version" => s!(env!("CARGO_PKG_VERSION")),
            "st_yaml_text" => s!(self.yaml_text),
            "st_cmd_query" => s!("cargo crev repo query review {}", self.package_name),
            "st_comment" => s!(self.review_for_vim.comment),
            "st_package_name" => s!(self.package_name),
            "st_package_version" => s!(self.package_version),
            "st_bash_command" => format!(
                "cargo crev crate review -u --skip-activity-check {} {}",
                self.package_name, self.package_version
            ),
            /*
            "st_from_url" => s!(self.review_for_vim.from.url),

            "st_alternatives_0_source" => s!(self.st_alternatives_0_source()),
            "st_alternatives_0_name" => s!(self.st_alternatives_0_name()),
            "st_advisories_comment_0_0" => s!(self.st_advisories_comment_0_0()),
            */
            _ => replace_with_string_match_else(&self.data_model_name(), placeholder),
        }
    }
    /// exclusive url encoded for href and src
    fn replace_with_url(&self, placeholder: &str, _subtemplate: &str, _pos_cursor: usize) -> UrlUtf8EncodedString {
        // dbg!( &placeholder);
        match placeholder {
            // the href for css is good for static data. For dynamic route it must be different.
            "su_css_route" => url_u!("/rust-reviews/css/rust-reviews.css"),
            "su_favicon_route" => url_u!("/rust-reviews/favicon.png"),
            "su_img_src_logo" => url_u!("/rust-reviews/images/Logo_02.png"),
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
    fn render_sub_template(&self, template_name: &str, _sub_templates: &Vec<SubTemplate>) -> Vec<Node> {
        // dbg!( &placeholder);
        match template_name {
            _ => render_sub_template_match_else(&self.data_model_name(), template_name),
        }
    }
}
