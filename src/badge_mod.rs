//! badge_mod
// the code is copied from
// https://github.com/nwtgck/svg-badge-scala

use crate::*;

//use serde_derive::{Deserialize, Serialize};
//use std::fs;
use unwrap::unwrap;

#[derive(Clone, Debug)]
pub struct Badge {
    pub subject_text: String,
    pub status_text: String,
    pub width: usize,
    pub subject_width: usize,
    pub status_width: usize,
    pub badge_color: String,
    pub subject_x: usize,
    pub status_x: usize,
    pub height: usize,
}

impl Badge {
    /// prepare the data
    pub fn new(subject_text: &str, status_text: &str, badge_color: &str) -> Self {
        let padding = 10;
        let subject_width = Self::text_to_width(subject_text) + padding;
        let status_width = Self::text_to_width(status_text) + padding;
        let height = 20;
        let width = subject_width + status_width;
        let subject_x = subject_width / 2;
        let status_x = subject_width + (status_width / 2);

        Badge {
            subject_text: s!(subject_text),
            status_text: s!(status_text),
            width,
            subject_width,
            status_width,
            badge_color: s!(badge_color),
            subject_x,
            status_x,
            height,
        }
    }
    pub fn crev_count(crate_name: &str, state_global: ArcMutStateGlobal) -> Self {
        let subject_text = "crev reviews";
        let status: usize = unwrap!(state_global.lock())
            .review_index
            .vec
            .iter()
            .map(|e| if e.crate_name == crate_name { 1 } else { 0 })
            .sum();
        let status_text = &status.to_string();
        let badge_color = "#6c3";
        //return
        Self::new(subject_text, status_text, badge_color)
    }
    /// private functions
    fn text_to_width(text: &str) -> usize {
        text.chars()
            .map(|e| {
                if e.is_lowercase() {
                    8
                } else if e.is_uppercase() {
                    10
                } else {
                    9
                }
            })
            .sum()
    }
}

impl HtmlServerTemplateRender for Badge {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        // return
        s!("Badge")
    }
    /// renders the complete html file. Not a sub-template/fragment.
    fn render_html_file(&self, templates_folder_name: &str) -> String {
        let template_file_name = format!("{}badge_template.html", templates_folder_name);
        let html = self.render_from_file(&template_file_name);

        // return
        html
    }
    /// boolean : is the next node rendered or not
    fn retain_next_node_or_attribute(&self, placeholder: &str) -> bool {
        // dbg!(&placeholder);
        match placeholder {
            _ => retain_next_node_or_attribute_match_else(&self.data_model_name(), placeholder),
        }
    }

    /// returns a String to replace the next text-node
    fn replace_with_string(&self, placeholder: &str, _subtemplate: &str, _pos_cursor: usize) -> String {
        // dbg!(&placeholder);
        match placeholder {
            "st_subject_text" => s!(self.subject_text),
            "st_status_text" => s!(self.status_text),
            "st_width" => s!(self.width),
            "st_height" => s!(self.height),
            "st_subject_width" => s!(self.subject_width),
            "st_badge_color" => s!(self.badge_color),
            "st_subject_x" => s!(self.subject_x),
            "st_status_x" => s!(self.status_x),
            "st_d1" => s!("M0 0h{}v{}H0z", self.subject_width, self.height),
            "st_d2" => s!("M{} 0h{}v{}H{}z", self.subject_width, self.status_width, self.height, self.subject_width),
            "st_d3" => s!("M0 0h{}v{}H0z", self.width, self.height),
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
