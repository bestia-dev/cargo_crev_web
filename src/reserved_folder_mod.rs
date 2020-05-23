//! reserved_folder_mod
//! This is only one module/html page, but can execute different actions.
//! The data model must have fields for every action as Option<>.
//! Because only this data can influence the html render.
//! There are different "new" functions for different actions, to prepare adequate data.
//! If field is is_some(), then render the html part dedicated to this action.

use crate::html_server_template_mod::*;
use crate::review_index_mod;
use crate::utils_mod::*;
use crate::CachedReviewIndex;
use crate::*;

use unwrap::unwrap;

#[derive(Debug)]
pub struct OnlyAuthor {
    pub author: String,
    pub author_id: String,
    pub author_url: String,
}
//use unwrap::unwrap;
#[derive(Debug, Default)]
pub struct ReservedFolder {
    pub list_fetched_author_id: Option<Vec<OnlyAuthor>>,
    pub reindex_after_fetch_new_reviews: Option<String>,
}

impl ReservedFolder {
    /// prepares the data
    pub fn new(_cached_review_index: CachedReviewIndex) -> Self {
        //let review_index = cached_review_index.lock().expect("error cached_review_index.lock()");
        // return
        ReservedFolder {
            ..Default::default()
        }
    }
    pub fn list_fetched_author_id(cached_review_index: CachedReviewIndex) -> Self {
        // fills the field list_fetched_author_id
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
            list_fetched_author_id: Some(only_author),
            ..Default::default()
        }
    }
    pub fn reindex_after_fetch_new_reviews(cached_review_index: CachedReviewIndex) -> Self {
        let mut review_index = cached_review_index
            .lock()
            .expect("error cached_review_index.lock()");
        *review_index = review_index_mod::ReviewIndex::new();
        // return
        ReservedFolder {
            reindex_after_fetch_new_reviews: Some(s!("Reindex finished.")),
            ..Default::default()
        }
    }
    pub fn list_new_author_id(cached_review_index: CachedReviewIndex) -> Self {
        // first I need the list of fetched authors
        let review_index = cached_review_index
        .lock()
        .expect("error cached_review_index.lock()");
    use itertools::Itertools;
    let mut vec_of_author_url: Vec<String> = review_index
        .vec
        .iter()
        .unique_by(|rev| &rev.author_url)
        .map(|rev| rev.author_url.clone())
        .collect();
        vec_of_author_url.sort_by(|a, b| a.cmp(&b));
        println!("vec_of_author_url: {:#?}", vec_of_author_url);
        
        use tokio::task;
        task::spawn(async move {
            let json = unwrap!(
                surf::get("https://api.github.com/search/repositories?q=crev-proofs")
                    .recv_string()
                    .await
            );
            //this is json vector, find url:
            let mut vec_of_urls: Vec<String> = vec![];
            let mut cursor_pos = 0;

            // I need this format for author_url:
            // https://github.com/BurntSushi/crev-proofs
            // the contents_url return this format
            // https://api.github.com/repos/leo-lb/crev-proofs/contents",
            // i will transform it with replace()

            while let Some(pos_start) =
                find_pos_after_delimiter(&json, cursor_pos, r#""contents_url": ""#)
            {
                if let Some(pos_end) =
                    find_pos_before_delimiter(&json, pos_start, r#"content/{+path}""#)
                {
                    vec_of_urls.push(format!("{}/", &json[pos_start..pos_end]));
                    cursor_pos = pos_end;
                } else {
                    break;
                }
            }

            let mut vec_of_new = vec![];
            for url in vec_of_urls.iter() {
                //if exists in index, I don't need it
                let author_url = url.replace("/api.github.com/repos/", "/github.com/");
                println!("author_url: {:#?}", author_url);
                if !vec_of_author_url.iter().any(|v| v == &author_url) {
                    vec_of_new.push(author_url);
                }
            }
            println!("vec_of_new: {:#?}", vec_of_new);
        });
        // return
        ReservedFolder {
            ..Default::default()
        }
    }
}

impl HtmlServerTemplateRender for ReservedFolder {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        //return
        s!("ReservedFolder")
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
            "sb_is_list_fetched_author_id" => self.list_fetched_author_id.is_some(),
            "sb_is_reindex_after_fetch_new_reviews" => {
                println!(
                    "reindex_after_fetch_new_reviews {:?}",
                    self.reindex_after_fetch_new_reviews.is_some()
                );
                self.reindex_after_fetch_new_reviews.is_some()
            }
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
        // list_fetched_author_id is Option and can be None or Some
        let only_author;
        let item_at_cursor = if let Some(list) = &self.list_fetched_author_id {
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
            "st_css_route" => s!("/cargo_crev_web/css/cargo_crev_web.css"),
            "st_favicon_route" => s!("/cargo_crev_web/favicon.png"),
            "st_ordinal_number" => (cursor_pos + 1).to_string(),
            "st_author" => s!(&item_at_cursor.author),
            "st_author_route" => format!(
                "/cargo_crev_web/author/{}/",
                url_encode(&item_at_cursor.author_id)
            ),
            "st_author_id" => s!(&item_at_cursor.author_id),
            "st_author_url" => s!(&item_at_cursor.author_url),
            "st_reindex_after_fetch_new_reviews" => {
                s!(unwrap!(self.reindex_after_fetch_new_reviews.as_ref()))
            }
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
                if let Some(list) = &self.list_fetched_author_id {
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
