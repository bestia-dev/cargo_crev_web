//! all_summary_mod

// region: use
use crate::*;
//use serde_derive::{Deserialize, Serialize};
//use std::fs;
//use unwrap::unwrap;
// endregion: use

#[derive(Clone, Debug)]
pub struct VersionSummary {
    pub version: String,
    pub version_for_sorting: String,
    pub review_number: usize,
    pub rating_strong: usize,
    pub rating_positive: usize,
    pub rating_neutral: usize,
    pub rating_negative: usize,
    pub alternatives: usize,
    pub issues: usize,
    pub advisories: usize,
    pub thoroughness: usize,
    pub understanding: usize,
}

impl VersionSummary {
    pub fn new() -> Self {
        VersionSummary {
            version: "".to_string(),
            version_for_sorting: "".to_string(),
            review_number: 0,
            rating_strong: 0,
            rating_positive: 0,
            rating_neutral: 0,
            rating_negative: 0,
            alternatives: 0,
            issues: 0,
            advisories: 0,
            thoroughness: 0,
            understanding: 0,
        }
    }
}

impl html_template_mod::HtmlTemplating for VersionSummary {
    /// html_templating boolean id the next node is rendered or not
    fn call_fn_boolean(&self, fn_name: &str) -> bool {
        // println!("{}",&format!("call_fn_boolean: {}", &fn_name));
        match fn_name {
            _ => {
                let x = format!("Unrecognized version_summary_mod call_fn_boolean: \"{}\"", fn_name);
                println!("Error: {}", &x);
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
        use html_template_mod::to_string_zero_to_empty;
        match fn_name {
            "t_version" => self.version.to_string(),
            "t_review_number" => to_string_zero_to_empty(self.review_number),
            "t_rating_strong" => to_string_zero_to_empty(self.rating_strong),
            "t_rating_positive" => to_string_zero_to_empty(self.rating_positive),
            "t_rating_neutral" => to_string_zero_to_empty(self.rating_neutral),
            "t_rating_negative" => to_string_zero_to_empty(self.rating_negative),
            "t_alternatives" => to_string_zero_to_empty(self.alternatives),
            "t_issues" => to_string_zero_to_empty(self.issues),
            "t_advisories" => to_string_zero_to_empty(self.advisories),
            "t_thoroughness" => to_string_zero_to_empty(self.thoroughness),
            "t_understanding" => to_string_zero_to_empty(self.understanding),
            _ => {
                let x = format!("Unrecognized version_summary_mod call_fn_string: \"{}\"", fn_name);
                println!("Error: {}", &x);
                x
            }
        }
    }
    /*
            /// return a closure for the listener.
            #[allow(clippy::too_many_lines, clippy::type_complexity)]
            fn call_fn_listener(
                &self,
                fn_name: String,
            ) -> Box<dyn Fn(&mut dyn RootRender, VdomWeak, Event) + 'static> {
                Box::new(move |root, vdom, event| {
                    let fn_name = fn_name.clone();
                    let fn_name = fn_name.as_str();
                    let rrc = root.unwrap_mut::<RootRenderingComponent>();
                    //println!("{}",&format!("call_fn_listener: {}", &fn_name));
                    match fn_name {

                        "open_youtube" => {
                            // randomly choose a link from rrc.videos
                            let num = websysmod::get_random(0, rrc.game_data.videos.len());
                            #[allow(clippy::indexing_slicing)]
                            // cannot panic:the num is 0..video.len
                            websysmod::open_new_tab(&format!(
                                "https://www.youtube.com/watch?v={}",
                                rrc.game_data.videos[num]
                            ));
                        }
                        _ => {
                            let x = format!("Unrecognized version_summary_mod call_fn_listener: \"{}\"", fn_name);
                            println!("Error: {}",&x);
                        }
                    }
                })
            }
    */
    /// html_templating functions that return a Node
    #[allow(clippy::needless_return)]
    fn call_fn_node(&self, fn_name: &str) -> html_template_mod::Node {
        // println!("{}",&format!("call_fn_node: {}", &fn_name));
        match fn_name {
            _ => {
                let node = html_template_mod::Node {
                    node_enum: html_template_mod::NodeEnum::Element(
                        html_template_mod::ElementNode {
                            tag_name: "h2".to_string(),
                            attributes: vec![],
                            children: vec![html_template_mod::Node {
                                node_enum: html_template_mod::NodeEnum::Text(format!(
                                    "Error: Unrecognized version_summary_mod call_fn_node: \"{}\"",
                                    fn_name
                                )),
                            }],
                            namespace: None,
                        },
                    ),
                };
                return node;
            }
        }
    }

    /// html_templating functions that return a vector of Nodes
    #[allow(clippy::needless_return)]
    fn call_fn_vec_nodes(&self, fn_name: &str) -> Vec<html_template_mod::Node> {
        // println!("{}",&format!("call_fn_node: {}", &fn_name));
        match fn_name {
            _ => {
                let node = html_template_mod::Node {
                    node_enum: html_template_mod::NodeEnum::Element(
                        html_template_mod::ElementNode {
                            tag_name: "h2".to_string(),
                            attributes: vec![],
                            children: vec![html_template_mod::Node {
                                node_enum: html_template_mod::NodeEnum::Text(format!(
                                    "Error: Unrecognized version_summary_mod call_fn_vec_nodes: \"{}\"",
                                    fn_name
                                )),
                            }],
                            namespace: None,
                        },
                    ),
                };
                return vec![node];
            }
        }
    }
}
