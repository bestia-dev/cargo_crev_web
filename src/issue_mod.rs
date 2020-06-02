//! issue_mod

use crate::html_server_template_mod::*;
use crate::review_mod::Level;
use crate::*;
//use unwrap::unwrap;
//use strum_macros;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Issue {
    pub id: String,
    pub severity: Level,
    pub comment: String,
}

impl HtmlServerTemplateRender for Issue {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        // return
        s!("Issue")
    }
    /// renders the complete html file. Not a sub-template/fragment.
    fn render_html_file(&self, _templates_folder_name: &str) -> String {
        // not needed for Issue
        // return
        String::new()
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
        // dbg&placeholder));
        match placeholder {
            "st_issue_id" => s!(&self.id),
            "st_issue_severity" => self.severity.to_string(),
            "st_issue_comment" => s!(&self.comment),
            _ => replace_with_string_match_else(&self.data_model_name(), placeholder),
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
    fn render_sub_template(
        &self,
        template_name: &str,
        _sub_templates: &Vec<SubTemplate>,
    ) -> Vec<Node> {
        // dbg!(&placeholder);
        match template_name {
            _ => render_sub_template_match_else(&self.data_model_name(), template_name),
        }
    }
}
