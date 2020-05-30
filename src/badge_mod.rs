//! badge_mod

use crate::*;

//use serde_derive::{Deserialize, Serialize};
//use std::fs;
//use unwrap::unwrap;

pub struct Badge{
    pub subject_text: String,
    pub status_text: String,
    pub width: usize,
    pub subject_width: usize,
    pub status_width: usize,
    pub badge_color: String,
    pub subject_x: usize,
    pub status_x : usize,
    pub height:usize,

}

impl Badge {
    /// prepare the data
    pub fn new(
        subject_text: &str, status_text: &str, badge_color: &str,
        width: usize, subject_width: usize, ) -> Self {

        let height      = 20;
        let status_width  = width - subject_width;
        let subject_x   = subject_width / 2;
        let status_x   = subject_width + status_width / 2;

        Badge{
            subject_text:s!(subject_text),
            status_text:s!(status_text),
            width, 
            subject_width,
            status_width,
            badge_color: s!(badge_color),
            subject_x,
            status_x, 
            height,
        }
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
        let template_file_name = format!(
            "{}badge_template.html",
            templates_folder_name
        );
        let html = self.render_from_file(&template_file_name);

        // return
        html
    }
    /// boolean : is the next node rendered or not
    fn retain_next_node(&self, placeholder: &str) -> bool {
        // dbg!(&placeholder);
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
        // dbg!(&placeholder);
        match placeholder {
            // the href for css is good for static data. For dynamic route it must be different.
            "st_css_route" => s!("/cargo_crev_web/css/cargo_crev_web.css"),
            "st_favicon_route" => s!("/cargo_crev_web/favicon.png"),
            "st_subject_text" => self.subject_text.clone(),
            "st_status_text" => self.status_text.clone(),
            "st_width" => self.width.to_string(), 
            "st_subject_width" => self.subject_width.to_string(), 
            "st_badge_color" => self.badge_color.to_string(), 
            "st_subject_x" => self.subject_x.to_string(), 
            "st_status_x" => self.status_x.to_string(), 
            "st_d1"=> format!("M0 0h{}v{}H0z", self.subject_width, self.height),
            "st_d2"=> format!("M{} 0h{}v{}H{}z", self.subject_width, self.status_width,  self.height, self.subject_width),
            "st_d3"=> format!("M0 0h{}v{}H0z", self.width, self.height),
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
        _sub_templates: &Vec<SubTemplate>,
    ) -> Vec<Node> {
        // dbg!( &placeholder);
        match template_name {
            _ => render_sub_template_match_else(&self.data_model_name(), template_name),
        }
    }
}
