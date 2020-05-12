//! crev_query_templating_mod

use crate::crev_query_mod::*;
use crate::html_template_mod::*;
//use unwrap::unwrap;

impl HtmlTemplating for CrevQueryData {
    /// html_templating boolean id the next node is rendered or not
    fn call_fn_boolean(&self, placeholder: &str) -> bool {
        // println!("{}",&format!("call_fn_boolean: {}", &placeholder));
        match placeholder {
            _ => {
                println!(
                    "Error: Unrecognized crev_query_templating_mod call_fn_boolean: \"{}\"",
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
        // println!("{}",&format!("call_fn_string: {}", &placeholder));
        match placeholder {
            _ => {
                let err_msg = format!(
                    "Error: Unrecognized crev_query_templating_mod call_fn_string: \"{}\"",
                    placeholder
                );
                println!("{}", &err_msg);
                err_msg
            }
        }
    }
    /// html_templating functions that return a vector of Nodes
    #[allow(clippy::needless_return)]
    fn call_fn_vec_nodes(&self, placeholder: &str) -> Vec<ElementNode> {
        // println!("{}",&format!("call_fn_vec_nodes: {}", &placeholder));
        match placeholder {
            _ => {
                // so much boilerplate
                let err_msg = format!(
                    "Error: Unrecognized crev_query_templating_mod call_fn_vec_nodes: \"{}\"",
                    placeholder
                );
                println!("{}", &err_msg);
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
        // println!("{}",&format!("render_sub_template: {}", &placeholder));
        match template_name {
            _ => {
                // so much boilerplate
                let err_msg = format!(
                    "Error: Unrecognized crev_query_templating_mod render_sub_template: \"{}\"",
                    template_name
                );
                println!("{}", &err_msg);
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
