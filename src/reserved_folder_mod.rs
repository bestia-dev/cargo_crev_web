//! reserved_folder_mod

use crate::html_server_template_mod::*;
use crate::utils_mod::*;
use crate::CachedReviewIndex;

use unwrap::unwrap;

#[derive(Debug)]
pub struct OnlyAuthor {
    pub author: String,
    pub author_id: String,
    pub author_url: String,
}
//use unwrap::unwrap;
#[derive(Debug)]
pub struct ReservedFolder {
    pub list_trusted_author_id: Option<Vec<OnlyAuthor>>,
}

impl ReservedFolder {
    /// prepares the data
    pub fn new(_cached_review_index: CachedReviewIndex) -> Self {
        //let review_index = cached_review_index.lock().expect("error cached_review_index.lock()");
        // return
        ReservedFolder {
            list_trusted_author_id: None,
        }
    }
    pub fn list_trusted_author_id(cached_review_index: CachedReviewIndex) -> Self {
        // fills the field list_trusted_author_id
        let review_index = cached_review_index
            .lock()
            .expect("error cached_review_index.lock()");
        use itertools::Itertools;
        let mut only_author: Vec<OnlyAuthor> = review_index
            .vec
            .iter()
            .unique_by(|rev| &rev.author)
            .map(|rev| OnlyAuthor {
                author: rev.author.clone(),
                author_id: rev.author_id.clone(),
                author_url: rev.author_url.clone(),
            })
            .collect();
        only_author.sort_by(|a, b| a.author.cmp(&b.author));
        println!("only author: {:#?}", only_author);

        // return
        ReservedFolder {
            list_trusted_author_id: Some(only_author),
        }
    }
    pub fn reindex_after_fetch_new_reviews(_cached_review_index: CachedReviewIndex) -> Self {
        // return
        ReservedFolder {
            list_trusted_author_id: None,
        }
    }
}

impl HtmlServerTemplateRender for ReservedFolder {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        //return
        "ReservedFolder".to_string()
    }
    /// renders the complete html file. Not a sub-template/fragment.
    fn render_html_file(&self, templates_folder_name: &str) -> String {
        let template_file_name = format!(
            "{}reserved_folder/reserved_folder_template.html",
            templates_folder_name
        );
        let html = self.render_from_file(&template_file_name);

        // return
        html
    }
    /// boolean : is the next node rendered or not
    fn retain_next_node(&self, placeholder: &str) -> bool {
        // eprintln!("{}",&format!("retain_next_node: {}", &placeholder));
        match placeholder {
            "sb_is_list_trusted_author_id" => self.list_trusted_author_id.is_some(),
            _ => retain_next_node_match_else(&self.data_model_name(), placeholder),
        }
    }

    /// returns a String to replace the next text-node
    #[allow(
        clippy::needless_return,
        clippy::integer_arithmetic,
        clippy::indexing_slicing
    )]
    fn replace_with_string(&self, placeholder: &str, cursor_pos: usize) -> String {
        // eprintln!("{}",&format!("replace_with_string: {}", &placeholder));
        // list_trusted_author_id is Option and can be None or Some
        let only_author;
        let item_at_cursor = if let Some(list) = &self.list_trusted_author_id {
            &list[cursor_pos]
        } else {
            only_author = OnlyAuthor {
                author: String::new(),
                author_id: String::new(),
                author_url: String::new(),
            };
            //return
            &only_author
        };
        match placeholder {
            // the href for css is good for static data. For dynamic route it must be different.
            "st_css_route" => "/cargo_crev_web/css/cargo_crev_web.css".to_string(),
            "st_favicon_route" => "/cargo_crev_web/favicon.png".to_string(),
            "st_ordinal_number" => (cursor_pos + 1).to_string(),
            "st_author" => item_at_cursor.author.to_string(),
            "st_author_route" => format!(
                "/cargo_crev_web/author/{}/",
                url_encode(&item_at_cursor.author_id)
            ),
            "st_author_id" => item_at_cursor.author_id.to_string(),
            "st_author_url" => item_at_cursor.author_url.to_string(),
            _ => replace_with_string_match_else(&self.data_model_name(), placeholder),
        }
    }
    /// returns a vector of Nodes to replace the next Node
    #[allow(clippy::needless_return)]
    fn replace_with_nodes(&self, placeholder: &str) -> Vec<Node> {
        // eprintln!("{}",&format!("replace_with_nodes: {}", &placeholder));
        match placeholder {
            _ => replace_with_nodes_match_else(&self.data_model_name(), placeholder),
        }
    }
    /// renders sub-template
    #[allow(clippy::needless_return)]
    fn render_sub_template(
        &self,
        template_name: &str,
        sub_templates: &Vec<SubTemplate>,
    ) -> Vec<Node> {
        // eprintln!("{}",&format!("render_sub_template: {}", &placeholder));
        match template_name {
            "stmplt_authors" => {
                // eprintln!("stmplt_authors: {}", "");
                let mut nodes = vec![];
                if let Some(list) = &self.list_trusted_author_id {
                    let sub_template = unwrap!(sub_templates
                        .iter()
                        .find(|&template| template.name == template_name));
                    // sub-template repeatable
                    for cursor_for_vec in 0..list.len() {
                        let vec_node = unwrap!(self.render_template_raw_to_nodes(
                            &sub_template.template,
                            HtmlOrSvg::Html,
                            cursor_for_vec
                        ));
                        nodes.extend_from_slice(&vec_node);
                    }
                }
                // return
                nodes
            }
            _ => render_sub_template_match_else(&self.data_model_name(), template_name),
        }
    }
}
