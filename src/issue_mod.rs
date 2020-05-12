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
    fn call_fn_boolean(&self, placeholder: &str) -> bool {
        // eprintln!("{}",&format!("call_fn_boolean: {}", &placeholder));
        match placeholder {
            _ => {
                eprintln!(
                    "Error: Unrecognized issue_mod call_fn_boolean: \"{}\"",
                    placeholder
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
    fn call_fn_string(&self, placeholder: &str) -> String {
        // eprintln!("{}",&format!("call_fn_string: {}", &placeholder));
        match placeholder {
            "t_issue_id" => self.id.to_string(),
            "t_issue_severity" => self.severity.to_string(),
            "t_issue_comment" => self.comment.to_string(),
            _ => {
                let err_msg = format!(
                    "Error: Unrecognized issue_mod call_fn_string: \"{}\"",
                    placeholder
                );
                eprintln!("{}", &err_msg);
                err_msg
            }
        }
    }
    /// html_templating functions that return a vector of Nodes
    #[allow(clippy::needless_return)]
    fn call_fn_vec_nodes(&self, placeholder: &str) -> Vec<Node> {
        // eprintln!("{}",&format!("call_fn_vec_nodes: {}", &placeholder));
        match placeholder {
            _ => {
                // so much boilerplate
                let err_msg = format!(
                    "Error: Unrecognized issue_mod call_fn_vec_nodes: \"{}\"",
                    placeholder
                );
                eprintln!("{}", &err_msg);
                let node = Node {
                    node_enum: NodeEnum::Element(ElementNode {
                        tag_name: "h2".to_string(),
                        attributes: vec![],
                        children: vec![Node {
                            node_enum: NodeEnum::Text(err_msg),
                        }],
                        namespace: None,
                    }),
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
        _sub_templates: &Vec<SubTemplate>,
    ) -> Vec<Node> {
        // eprintln!("{}",&format!("render_sub_template: {}", &placeholder));
        match template_name {
            _ => {
                // so much boilerplate
                let err_msg = format!(
                    "Error: Unrecognized issue_mod render_sub_template: \"{}\"",
                    template_name
                );
                eprintln!("{}", &err_msg);
                let node = Node {
                    node_enum: NodeEnum::Element(ElementNode {
                        tag_name: "h2".to_string(),
                        attributes: vec![],
                        children: vec![Node {
                            node_enum: NodeEnum::Text(err_msg),
                        }],
                        namespace: None,
                    }),
                };
                return vec![node];
            }
        }
    }
}
