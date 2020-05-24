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

//use std::fs;
use unwrap::unwrap;

#[derive(Debug)]
pub struct OnlyAuthor {
    pub author: String,
    pub author_id: String,
    pub author_url: String,
}
#[derive(Debug, Clone, Default)]
pub struct AuthorNew {
    pub author_url_author_name: String,
}

//use unwrap::unwrap;
#[derive(Debug, Default)]
pub struct ReservedFolder {
    pub list_fetched_author_id: Option<Vec<OnlyAuthor>>,
    pub reindex_after_fetch_new_reviews: Option<String>,
    pub list_new_author_id: Option<Vec<AuthorNew>>,
    pub add_author_url: Option<String>,
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
    pub async fn list_new_author_id(cached_review_index: CachedReviewIndex) -> Self {
        let mut vec_of_new = Vec::<AuthorNew>::new();
        use futures::future;
        // closure don't need the definition of the crazy return type. woohoo.
        let surf_get = |page_number: usize| {
            surf::get(&format!(
                "https://api.github.com/search/repositories?q=crev-proofs&page={}",
                page_number
            ))
            .recv_string()
        };
        // The github api response has pagination. It returns 30 items in one page.
        // The public api allows 10 request per minute. Enough for now.
        let mut is_last_page_empty = false;
        let mut page_number: usize = 1;
        // I will read the "total_count": 78, in the beginning of the first json
        // and be very exact about how many pages to fetch
        let mut total_count: usize = 999999999;
        while !is_last_page_empty && (page_number - 1) * 30 * 3 < total_count {
            // first make 3 requests concurrently
            let fut_1 = surf_get(page_number);
            page_number += 1;
            let fut_2 = surf_get(page_number);
            page_number += 1;
            let fut_3 = surf_get(page_number);
            page_number += 1;
            // await all 3 concurrently
            let vec_of_str = future::join_all(vec![fut_1, fut_2, fut_3]).await;

            // first I need the list of fetched authors
            // I cannot construct this before await, because await can take a lot of time
            // and reference lifetime is in question?
            // so I must do it after await.
            // probably the Mutex is available everywhere, anytime ?
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

            // the first time we loop around let's find the "total_count": 78,
            // so we can be precise to end the loop with no additional requests
            if page_number == 4 {
                let mut pos_cursor = 0;
                let resp_body = unwrap!(vec_of_str[0].as_ref());
                let range = unwrap!(find_range_between_delimiters(
                    resp_body,
                    &mut pos_cursor,
                    r#""total_count": "#,
                    r#","#
                ));
                total_count = unwrap!(resp_body[range].parse());
                // println!("total_count: {}", total_count);
            }

            for resp_body in vec_of_str.iter() {
                let resp_body = unwrap!(resp_body.as_ref());
                // unwrap!(fs::write("github_search.resp_body",&resp_body));
                // this is very big json vector, but I am interested in one single field: contents_url:
                // REST api is so terribly wasteful. GraphQl is theoretically much better.
                // I will also avoid the use of serde. Just to practice coding.
                let mut vec_of_urls: Vec<AuthorNew> = vec![];
                let mut pos_cursor = 0;

                // I need this format for author_url:
                // https://github.com/BurntSushi/crev-proofs
                // the contents_url return this format
                // https://api.github.com/repos/leo-lb/crev-proofs/contents",
                // the url must end with /crev_proofs/ else discard
                // the only valuable info is author_url_author_name 

                while let Some(pos_start) = find_pos_after_delimiter(
                    &resp_body,
                    pos_cursor,
                    r#""contents_url": "https://api.github.com/repos/"#,
                ) {
                    if let Some(pos_end) =
                        find_pos_before_delimiter(&resp_body, pos_start, r#"/crev_proofs/contents/{+path}""#)
                    {
                        let mut split_iterator = resp_body[pos_start..pos_end].split('/');
                        vec_of_urls.push(AuthorNew {
                            author_url_author_name: s!(unwrap!(split_iterator.next())),
                        });
                        pos_cursor = pos_end;
                    } else {
                        break;
                    }
                }
                // println!("vec_of_urls {}: {:#?}", vec_of_urls.len(), vec_of_urls);
                if vec_of_urls.is_empty() {
                    is_last_page_empty = true;
                // this will end the while loop
                } else {
                    for u in vec_of_urls.iter() {
                        //if already exists in index, I don't need it
                        let author_url = format!(
                            "https://github.com/{}/crev-proofs/",
                            u.author_url_author_name
                        );
                        //println!("author_url: {:#?}", author_url);
                        if !vec_of_author_url.iter().any(|v| v == &author_url) {
                            vec_of_new.push(AuthorNew {
                                author_url_author_name: s!(&u.author_url_author_name),
                            });
                        }
                    }
                }
            } // for resp_body
        } // loop
          //println!("vec_of_new: {}: {:#?}", vec_of_new.len(), &vec_of_new);
          // return
        ReservedFolder {
            list_new_author_id: Some(vec_of_new),
            ..Default::default()
        }
    }
    pub async fn add_author_url(
        author_url_fragment: String,
        _cached_review_index: CachedReviewIndex,
    ) -> Self {
        // in this fragment are 2 parts delimited with /
        // let split it and use parts one by one
        println!("author_url_fragment: {}", author_url_fragment);
        let mut split_iterator = author_url_fragment.split('/');
        let author_new = AuthorNew {
            author_url_author_name: s!(unwrap!(split_iterator.next())),
        };
        // find github content
        let gh_content_url = format!(
            "https://api.github.com/repos/{}/crev_proofs/contents",
            author_new.author_url_author_name
        );
        println!("gh_content_url: {}", &gh_content_url);
        let resp_body = unwrap!(surf::get(&gh_content_url).recv_string().await);
        // the new format of proof
        // "name": "5XSQsMDSEeY_uFOh9UOkkUiq8nt8ThA5ZJCHaxcuhjM",
        // "size": 0,
        let mut author_id = s!("");
        let mut pos_cursor: usize = 0;
        println!("resp_body: {}", &resp_body);
        loop {
            let range_name =
                find_range_between_delimiters(&resp_body, &mut pos_cursor, r#""name": ""#, r#"""#);
            if let Some(range_name) = range_name {
                println!("range_name: {:?}", &range_name);
                let range_size =
                    find_range_between_delimiters(&resp_body, &mut pos_cursor, r#""size": ""#, r#","#);
                if let Some(range_size) = range_size {
                    println!("range_size: {:?}", &range_size);
                    if &resp_body[range_size] == "0" {
                        author_id = s!(&resp_body[range_name]);
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        println!("author_id: {}", &author_id);
        // the old format of review
        // "name": "5XSQsMDSEeY_uFOh9UOkkUiq8nt8ThA5ZJCHaxcuhjM",
        // "size": 0,
        let mut author_id = s!("");
        let mut pos_cursor: usize = 0;
        println!("resp_body: {}", &resp_body);
        loop {
            let range_name =
                find_range_between_delimiters(&resp_body, &mut pos_cursor, r#""name": ""#, r#"""#);
            if let Some(range_name) = range_name {
                println!("range_name: {:?}", &range_name);
                let range_size =
                    find_range_between_delimiters(&resp_body, &mut pos_cursor, r#""size": ""#, r#","#);
                if let Some(range_size) = range_size {
                    println!("range_size: {:?}", &range_size);
                    if &resp_body[range_size] == "0" {
                        author_id = s!(&resp_body[range_name]);
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        println!("author_id: {}", &author_id);

        // return
        ReservedFolder {
            add_author_url: Some(format!("Add author finished. {}", &gh_content_url)),
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
    fn render_html_file(&self, templates_folderange_name: &str) -> String {
        let template_file_name = format!(
            "{}reserved_folder/reserved_folder_template.html",
            templates_folderange_name
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
                self.reindex_after_fetch_new_reviews.is_some()
            }
            "sb_list_new_author_id" => self.list_new_author_id.is_some(),
            "sb_add_author_url" => self.add_author_url.is_some(),
            _ => retain_next_node_match_else(&self.data_model_name(), placeholder),
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
        subtemplate: &str,
        pos_cursor: usize,
    ) -> String {
        // eprintln!("{}",&format!("replace_with_string: {}", &placeholder));
        // list_fetched_author_id is Option and can be None or Some
        let mut item_at_cursor_1 = &OnlyAuthor {
            author: String::new(),
            author_id: String::new(),
            author_url: String::new(),
        };
        if subtemplate == "stmplt_authors" {
            if let Some(list) = &self.list_fetched_author_id {
                item_at_cursor_1 = &list[pos_cursor];
            }
        }
        let mut item_at_cursor_2 = &AuthorNew::default();
        if subtemplate == "stmplt_authors_new" {
            if let Some(list) = &self.list_new_author_id {
                item_at_cursor_2 = &list[pos_cursor];
            }
        }
        match placeholder {
            // the href for css is good for static data. For dynamic route it must be different.
            "st_css_route" => s!("/cargo_crev_web/css/cargo_crev_web.css"),
            "st_favicon_route" => s!("/cargo_crev_web/favicon.png"),
            "st_ordinal_number" => (pos_cursor + 1).to_string(),
            "st_author" => s!(&item_at_cursor_1.author),
            "st_author_route" => format!(
                "/cargo_crev_web/author/{}/",
                url_encode(&item_at_cursor_1.author_id)
            ),
            "st_author_id" => s!(&item_at_cursor_1.author_id),
            // same name from different data model is not allowed
            "st_author_url" => s!(&item_at_cursor_1.author_url),
            "st_author_name" => s!(&item_at_cursor_2.author_url_author_name),
            "st_author_url_2" => format!(
                "https://github.com/{}/crev-proofs/",
                &item_at_cursor_2.author_url_author_name,
            ),
            "st_add_author_url_route" => format!(
                "/cargo_crev_web/reserved_folder/add_author_url/{}/",
                url_encode(&item_at_cursor_2.author_url_author_name)),
            "st_reindex_after_fetch_new_reviews" => {
                s!(unwrap!(self.reindex_after_fetch_new_reviews.as_ref()))
            }
            "st_add_author_url" => s!(unwrap!(self.add_author_url.as_ref())),
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
                            "list_fetched_author_id",
                            cursor_for_vec,
                        ));
                        nodes.extend_from_slice(&vec_node);
                    }
                }
                // return
                nodes
            }
            "stmplt_authors_new" => {
                // eprintln!("stmplt_authors_new: {}", "");
                let mut nodes = vec![];
                if let Some(list) = &self.list_new_author_id {
                    let sub_template = unwrap!(sub_templates
                        .iter()
                        .find(|&template| template.name == template_name));
                    // sub-template repeatable
                    for cursor_for_vec in 0..list.len() {
                        let vec_node = unwrap!(self.render_template_raw_to_nodes(
                            &sub_template.template,
                            HtmlOrSvg::Html,
                            template_name,
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
