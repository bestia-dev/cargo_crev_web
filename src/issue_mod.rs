//! issue_mod

use crate::html_template_mod::*;
use crate::proof_mod::Level;
//use unwrap::unwrap;
//use strum_macros;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Issue {
    pub id: String,
    pub severity: Level,
    pub comment: String,
}

impl HtmlTemplatingRender for Issue {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        //return
        "Issue".to_string()
    }
    /// Issue is never a full html file. It is always a sub-template.
    fn render_html_file(&self, _templates_folder_name: &str) -> String {
        //return
        String::new()
    }
    // html_templating boolean id the next node is rendered or not
    fn retain_next_node(&self, placeholder: &str) -> bool {
        // eprintln!("{}",&format!("retain_next_node: {}", &placeholder));
        match placeholder {
            _ => retain_next_node_match_else(&self.data_model_name(), placeholder),
        }
    }

    // html_templating functions that return a String
    #[allow(
        clippy::needless_return,
        clippy::integer_arithmetic,
        clippy::indexing_slicing
    )]
    fn replace_with_string(&self, placeholder: &str, _cursor_pos: usize) -> String {
        // eprintln!("{}",&format!("replace_with_string: {}", &placeholder));
        match placeholder {
            "t_issue_id" => self.id.to_string(),
            "t_issue_severity" => self.severity.to_string(),
            "t_issue_comment" => self.comment.to_string(),
            _ => replace_with_string_match_else(&self.data_model_name(), placeholder),
        }
    }
    // html_templating functions that return a vector of Nodes
    #[allow(clippy::needless_return)]
    fn replace_with_nodes(&self, placeholder: &str) -> Vec<Node> {
        // eprintln!("{}",&format!("replace_with_nodes: {}", &placeholder));
        match placeholder {
            _ => replace_with_nodes_match_else(&self.data_model_name(), placeholder),
        }
    }
    // html_templating for sub-template
    #[allow(clippy::needless_return)]
    fn render_sub_template(
        &self,
        template_name: &str,
        _sub_templates: &Vec<SubTemplate>,
    ) -> Vec<Node> {
        // eprintln!("{}",&format!("render_sub_template: {}", &placeholder));
        match template_name {
            _ => render_sub_template_match_else(&self.data_model_name(), template_name),
        }
    }
}
