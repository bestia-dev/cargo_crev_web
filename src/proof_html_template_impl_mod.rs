//! proof_html_template_impl_mod  

use crate::html_template_mod::*;
use crate::proof_mod::Proof;
//use crate::*;

use std::fs;
use unwrap::unwrap;

pub fn push_review_to_html(html: &mut String, proof: &Proof) {
    //read template and then render
    let template = unwrap!(fs::read_to_string("crev/proof_template.html"));
    let template = between_body_tag(&template);
    let root_node = unwrap!(proof.render_template(&template, HtmlOrSvg::Html));
    //from Nodes to String
    *html = from_node_to_string(root_node);
    //println!("after: {}", "proof.render_template()");
    //println!("{:?}", html);
}

impl HtmlTemplating for Proof {
    /// html_templating boolean id the next node is rendered or not
    fn call_fn_boolean(&self, fn_name: &str) -> bool {
        // println!("{}",&format!("call_fn_boolean: {}", &fn_name));
        match fn_name {
            "b_not_visible" => false,
            "b_has_alternatives" => self.alternatives.is_some(),
            "b_has_issues" => self.issues.is_some(),
            "b_has_advisories" => self.advisories.is_some(),
            "b_has_old_advisory" => self.advisory.is_some(),
            _ => {
                let x = format!("Error: Unrecognized call_fn_boolean: \"{}\"", fn_name);
                println!("{}", &x);
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
            "t_crate_name_version" => format!("{} {}", self.package.name, self.package.version),
            "t_review_rating" => {
                if let Some(review) = &self.review {
                    review.rating.to_string()
                } else {
                    "".to_string()
                }
            }
            "t_review_date" => self.date[..10].to_string(),
            "t_review_author" => {
                // naive method to extract author
                let author = self
                    .from
                    .url
                    .replace("https://github.com/", "")
                    .replace("/crev-proofs", "");
                //return
                author
            }
            "t_crate_thoroughness_understanding" => {
                if let Some(review) = &self.review {
                    format!(
                        "{} {}",
                        review.thoroughness.to_string(),
                        review.understanding.to_string()
                    )
                } else {
                    "".to_string()
                }
            }
            "t_review_comment" => {
                if let Some(comment) = &self.comment {
                    comment.clone()
                } else {
                    "".to_string()
                }
            }
            _ => {
                let x = format!("Error: Unrecognized call_fn_string: \"{}\"", fn_name);
                println!("{}", &x);
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
                            let x = format!("Error: Unrecognized call_fn_listener: \"{}\"", fn_name);
                            println!("{}",&x);
                        }
                    }
                })
            }
    */
    /// html_templating functions that return a Node
    #[allow(clippy::needless_return)]
    fn call_fn_node(&self, fn_name: &str) -> Node {
        // println!("{}",&format!("call_fn_node: {}", &fn_name));
        match fn_name {
            _ => {
                let node = Node {
                    node_enum: NodeEnum::Element(ElementNode {
                        tag_name: "h2".to_string(),
                        attributes: vec![],
                        children: vec![Node {
                            node_enum: NodeEnum::Text(TextNode {
                                text: format!("Error: Unrecognized call_fn_node: \"{}\"", fn_name),
                            }),
                        }],
                        namespace: None,
                    }),
                };
                return node;
            }
        }
    }

    /// html_templating functions that return a vector of Nodes
    #[allow(clippy::needless_return)]
    fn call_fn_vec_nodes(&self, fn_name: &str) -> Vec<Node> {
        // println!("{}",&format!("call_fn_node: {}", &fn_name));
        match fn_name {
            _ => {
                let node = Node {
                    node_enum: NodeEnum::Element(ElementNode {
                        tag_name: "h2".to_string(),
                        attributes: vec![],
                        children: vec![Node {
                            node_enum: NodeEnum::Text(TextNode {
                                text: format!(
                                    "Error: Unrecognized call_fn_vec_nodes: \"{}\"",
                                    fn_name
                                ),
                            }),
                        }],
                        namespace: None,
                    }),
                };
                return vec![node];
            }
        }
    }
}

/// fn open new local page with #
/// does not push to history
pub fn open_new_local_page(hash: &str) {
    // I want to put the first url in history.
    // These are opened from outside my app and I cannot manage that differently.
    // There are 2 of them:
    // 1. if the players starts without hash
    // 2. if the player scanned the qrcode and opened the p3 with group_id
    // For links opened inside the app, I can call the open with or without history.
    // For example for menu p21 I want to have a back button.
    /*
    let (_old_location_href, old_href_hash) = websysmod::get_url_and_hash();
    if old_href_hash.is_empty() || old_href_hash.starts_with("#p03.") {
        websysmod::open_new_local_page_push_to_history(hash)
    } else {
        let _x = websysmod::window().location().replace(hash);
    }
    */
}
/*
/// update html_template and extract and saves html_sub_templates
#[allow(clippy::integer_arithmetic)]
#[allow(clippy::indexing_slicing)]
pub fn update_html_template_and_sub_templates(
    rrc: &mut RootRenderingComponent,
    resp_body_text: &str,
) {
    // only the html inside the <body> </body>
    let mut tm = between_body_tag(&resp_body_text);
    // parse and save sub_templates <template name="xxx"></template>
    rrc.web_data.html_sub_templates.clear();
    loop {
        let mut exist_template = false;

        let pos1 = tm.find("<template ");
        let del2 = "</template>";
        let pos2 = tm.find(del2);
        if let Some(pos_start) = pos1 {
            if let Some(pos_end) = pos2 {
                exist_template = true;
                // drain - extract a substring and remove it from the original
                let sub1: String = tm.drain(pos_start..pos_end + del2.len()).collect();

                let del3 = "name=\"";
                let pos_name_start = unwrap!(sub1.find(del3));
                let sub2 = &sub1[pos_name_start + del3.len()..];
                //println!("{}",sub2);

                let pos_name_end = unwrap!(sub2.find('"'));
                let name = &sub2[0..pos_name_end];
                //println!("{}",name);

                let del5 = '>';
                let pos_name_end_tag = unwrap!(sub1.find(del5));
                let pos6 = unwrap!(sub1.find(del2));
                let sub_template = &sub1[pos_name_end_tag + 1..pos6];
                //println!("{}",sub_template);

                rrc.web_data
                    .html_sub_templates
                    .push((name.to_string(), sub_template.to_string()));
            }
        }
        if !exist_template {
            break;
        }
    }
    rrc.web_data.html_template = tm;
}
*/
