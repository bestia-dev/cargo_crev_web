//! review_mod

use crate::review_mod::*;
use crate::*;

//use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use unwrap::unwrap;

pub struct ReviewNew {
    pub rev: Review,
    pub yaml_text: String,
}

impl ReviewNew {
    #[allow(unused)]
    /// prepares the data
    pub fn new() -> Self {
        ReviewNew {
            rev: Review {
                ..Default::default()
            },
            yaml_text: s!(""),
        }
    }
    pub fn read_review(path: &str) -> Self {
        //let path = "../sample_data/review_1.yaml";
        let yaml_text = unwrap!(fs::read_to_string(path));
        let review: Review = unwrap!(serde_yaml::from_str(&yaml_text));
        ReviewNew {
            rev: review,
            yaml_text,
        }
    }
    pub fn from_form_data(form_data: HashMap<String, String>) -> Self {
        let mut rev = Review {
            ..Default::default()
        };
        for (key, value) in form_data {
            match key.as_ref() {
                "comment" => rev.comment = Some(value.to_string()),
                _ => {}
            }
        }
        let yaml_text = unwrap!(serde_yaml::to_string(&rev));
        //return
        ReviewNew { rev, yaml_text }
    }

    pub fn st_comment(&self) -> String {
        if let Some(comment) = &self.rev.comment {
            comment.clone()
        } else {
            s!("")
        }
    }

    pub fn st_thoroughness(&self) -> String {
        if let Some(review) = &self.rev.review {
            review.thoroughness.to_string()
        } else {
            s!("")
        }
    }

    pub fn st_understanding(&self) -> String {
        if let Some(review) = &self.rev.review {
            review.understanding.to_string()
        } else {
            s!("")
        }
    }

    pub fn st_rating(&self) -> String {
        if let Some(review) = &self.rev.review {
            review.rating.to_string()
        } else {
            s!("")
        }
    }

    pub fn st_alternatives_0_source(&self) -> String {
        if let Some(alternatives) = &self.rev.alternatives {
            alternatives[0].source.clone()
        } else {
            s!("")
        }
    }

    pub fn st_alternatives_0_name(&self) -> String {
        if let Some(alternatives) = &self.rev.alternatives {
            alternatives[0].name.clone()
        } else {
            s!("")
        }
    }
    pub fn st_advisories_comment_0_0(&self) -> String {
        if let Some(advisories) = &self.rev.advisories {
            advisories[0].comment.clone()
        } else {
            s!("")
        }
    }
}

impl HtmlServerTemplateRender for ReviewNew {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        // return
        s!("ReviewNew")
    }
    /// renders the complete html file. Not a sub-template/fragment.
    fn render_html_file(&self, templates_folder_name: &str) -> String {
        let template_file_name = format!("{}review_new_template.html", templates_folder_name);
        let html = self.render_from_file(&template_file_name);

        // return
        html
    }
    /// boolean : is the next node rendered or not
    fn retain_next_node_or_attribute(&self, placeholder: &str) -> bool {
        // dbg!(&placeholder);
        match placeholder {
            "sb_has_alternative" => self.rev.alternatives.is_some(),
            "sb_has_issue" => self.rev.issues.is_some(),
            "sb_has_advisories" => self.rev.advisories.is_some(),
            // radio buttons in html have this terrible attribute checked. Horror.
            "sb_thoroughness_none" => &self.st_thoroughness() == "none",
            "sb_thoroughness_low" => &self.st_thoroughness() == "none",
            "sb_thoroughness_medium" => &self.st_thoroughness() == "medium",
            "sb_thoroughness_high" => &self.st_thoroughness() == "high",
            "sb_understanding_none" => &self.st_understanding() == "none",
            "sb_understanding_low" => &self.st_understanding() == "low",
            "sb_understanding_medium" => &self.st_understanding() == "medium",
            "sb_understanding_high" => &self.st_understanding() == "high",
            "sb_rating_none" => &self.st_rating() == "none",
            "sb_rating_negative" => &self.st_rating() == "negative",
            "sb_rating_neutral" => &self.st_rating() == "neutral",
            "sb_rating_positive" => &self.st_rating() == "positive",
            "sb_rating_strong" => &self.st_rating() == "strong",

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
        // list_fetched_author_id is Option and can be None or Some
        match placeholder {
            "st_yaml_text" => s!(self.yaml_text),
            "st_date" => s!(self.rev.date),
            "st_comment" => s!(self.st_comment()),
            "st_from_url" => s!(self.rev.from.url),
            "st_package_name" => s!(self.rev.package.name),
            "st_package_version" => s!(self.rev.package.version),
            "st_alternatives_0_source" => s!(self.st_alternatives_0_source()),
            "st_alternatives_0_name" => s!(self.st_alternatives_0_name()),
            "st_advisories_comment_0_0" => s!(self.st_advisories_comment_0_0()),
            _ => replace_with_string_match_else(&self.data_model_name(), placeholder),
        }
    }
    /// exclusive url encoded for href and src
    fn replace_with_url(
        &self,
        placeholder: &str,
        _subtemplate: &str,
        _pos_cursor: usize,
    ) -> UrlUtf8EncodedString {
        // dbg!( &placeholder);
        match placeholder {
            // the href for css is good for static data. For dynamic route it must be different.
            "su_css_route" => url_u!("/rust-reviews/css/rust-reviews.css"),
            "su_favicon_route" => url_u!("/rust-reviews/favicon.png"),
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
        _sub_templates: &Vec<SubTemplate>,
    ) -> Vec<Node> {
        // dbg!( &placeholder);
        match template_name {
            _ => render_sub_template_match_else(&self.data_model_name(), template_name),
        }
    }
}
