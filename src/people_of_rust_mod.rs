//! people_of_rust_mod

use crate::*;

//use unwrap::unwrap;
//use strum_macros;

pub struct PeopleOfRust {}

impl PeopleOfRust {
    /// prepares the data
    pub fn new() -> Self {
        // return
        PeopleOfRust {}
    }
}

impl HtmlServerTemplateRender for PeopleOfRust {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        // return
        s!("PeopleOfRust")
    }
    /// renders the complete html file. Not a sub-template/fragment.
    fn render_html_file(&self, templates_folder_name: &str) -> String {
        let template_file_name = format!("{}people_of_rust_template.html", templates_folder_name);
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
    #[allow(clippy::needless_return, clippy::integer_arithmetic, clippy::indexing_slicing)]
    fn replace_with_string(&self, placeholder: &str, _subtemplate: &str, _pos_cursor: usize) -> String {
        // dbg&placeholder));
        match placeholder {
            "st_cargo_crev_web_version" => s!(env!("CARGO_PKG_VERSION")),
            _ => replace_with_string_match_else(&self.data_model_name(), placeholder),
        }
    }
    /// exclusive url encoded for href and src
    fn replace_with_url(&self, placeholder: &str, _subtemplate: &str, _pos_cursor: usize) -> UrlUtf8EncodedString {
        // dbg!( &placeholder);
        match placeholder {
            "su_css_route" => url_u!("/rust-reviews/css/rust-reviews.css"),
            "su_favicon_route" => url_u!("/rust-reviews/favicon.png"),
            "su_img_src_logo" => url_u!("/rust-reviews/images/Logo_02.png"),
            _ => replace_with_url_match_else(&self.data_model_name(), placeholder),
        }
    }
    /// returns a vector of Nodes to replace the next Node
    #[allow(clippy::needless_return)]
    fn replace_with_nodes(&self, placeholder: &str) -> Vec<Node> {
        // dbg!( &placeholder);
        match placeholder {
            _ => replace_with_nodes_match_else(&self.data_model_name(), placeholder),
        }
    }
    /// renders sub-template
    #[allow(clippy::needless_return)]
    fn render_sub_template(&self, template_name: &str, _sub_templates: &Vec<SubTemplate>) -> Vec<Node> {
        // dbg!(&placeholder);
        match template_name {
            _ => render_sub_template_match_else(&self.data_model_name(), template_name),
        }
    }
}
