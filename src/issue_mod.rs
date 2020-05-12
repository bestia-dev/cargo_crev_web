//! issue_mod

use crate::html_template_mod::*;
use crate::proof_mod::Level;
use serde_derive::{Deserialize, Serialize};
//use unwrap::unwrap;
//use strum_macros;
#[derive(Serialize, Deserialize, Clone)]
pub struct Issue {
    pub id: String,
    pub severity: Level,
    pub comment: String,
}

impl HtmlTemplating for Issue {
    /// html_templating boolean id the next node is rendered or not
    fn call_fn_boolean(&self, fn_name: &str) -> bool {
        // println!("{}",&format!("call_fn_boolean: {}", &fn_name));
        match fn_name {
            _ => {
                println!(
                    "Error: Unrecognized issue_mod call_fn_boolean: \"{}\"",
                    fn_name
                );
                true
            }
        }
    }

    /// html_templating functions that return a String
    #[allow(
        clippy::needless_return,
        clippy::integer_arithmetic,
        clippy::indexing_slicing
    )]
    fn call_fn_string(&self, fn_name: &str) -> String {
        // println!("{}",&format!("call_fn_string: {}", &fn_name));
        match fn_name {
            "t_issue_id" => self.id.to_string(),
            "t_issue_severity" => self.severity.to_string(),
            "t_issue_comment" => self.comment.to_string(),
            _ => {
                let err_msg = format!(
                    "Error: Unrecognized issue_mod call_fn_string: \"{}\"",
                    fn_name
                );
                println!("{}", &err_msg);
                err_msg
            }
        }
    }
    /// html_templating functions that return a vector of Nodes
    #[allow(clippy::needless_return)]
    fn call_fn_vec_nodes(&self, fn_name: &str) -> Vec<ElementNode> {
        // println!("{}",&format!("call_fn_vec_nodes: {}", &fn_name));
        match fn_name {
            _ => {
                // so much boilerplate
                let err_msg = format!(
                    "Error: Unrecognized issue_mod call_fn_vec_nodes: \"{}\"",
                    fn_name
                );
                eprintln!("{}", &err_msg);
                let node = ElementNode {
                    tag_name: "h2".to_string(),
                    attributes: vec![],
                    children: vec![Node {
                        node_enum: NodeEnum::Text(err_msg),
                    }],
                    namespace: None,
                };
                return vec![node];
            }
        }
    }
    /// html_templating for sub-template
    #[allow(clippy::needless_return)]
    fn render_sub_template(
        &self,
        template_name: &str,
        sub_templates: &Vec<SubTemplate>,
    ) -> Vec<ElementNode> {
        // println!("{}",&format!("render_sub_template: {}", &fn_name));
        match template_name {
            _ => {
                // so much boilerplate
                let err_msg = format!(
                    "Error: Unrecognized issue_mod render_sub_template: \"{}\"",
                    template_name
                );
                eprintln!("{}", &err_msg);
                let node = ElementNode {
                    tag_name: "h2".to_string(),
                    attributes: vec![],
                    children: vec![Node {
                        node_enum: NodeEnum::Text(err_msg),
                    }],
                    namespace: None,
                };
                return vec![node];
            }
        }
    }
}
