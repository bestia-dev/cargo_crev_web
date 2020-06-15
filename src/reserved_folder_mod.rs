//! reserved_folder_mod
//! This is only one module/html page, but can execute different actions.
//! The data model must have fields for every action as Option<>.
//! Because only this data can influence the html render.
//! There are different "new" functions for different actions, to prepare adequate data.
//! If field is is_some(), then render the html part dedicated to this action.

use crate::data_file_scan_mod::*;
use crate::review_index_mod;
use crate::*;

use serde_derive::{Deserialize, Serialize};
use std::fs;
use unwrap::unwrap;

#[derive(Debug, Default)]
pub struct OnlyAuthor {
    pub author_name: String,
    pub author_id: String,
    pub author_url: String,
}

//use unwrap::unwrap;
#[derive(Debug, Default)]
pub struct ReservedFolder {
    pub list_fetched_author_id: Option<Vec<OnlyAuthor>>,
    pub reindex_after_fetch_new_reviews: Option<String>,
    pub fetch_new_reviews: Option<String>,
    pub list_new_author_id: Option<Vec<OnlyAuthor>>,
    pub add_author_url: Option<String>,
}

impl ReservedFolder {
    /// prepares the data
    pub fn new(_state_global: ArcMutStateGlobal) -> Self {
        // return
        ReservedFolder {
            ..Default::default()
        }
    }
    pub fn list_fetched_author_id(state_global: ArcMutStateGlobal) -> Self {
        // fills the field list_fetched_author_id
        use itertools::Itertools;
        let mut only_author: Vec<OnlyAuthor> = unwrap!(state_global.lock())
            .review_index
            .vec
            .iter()
            .unique_by(|rev| &rev.author_name)
            .map(|rev| OnlyAuthor {
                author_name: rev.author_name.clone(),
                author_id: rev.author_id.clone(),
                author_url: rev.author_url.clone(),
            })
            .collect();
        only_author.sort_by(|a, b| {
            a.author_name
                .to_lowercase()
                .cmp(&b.author_name.to_lowercase())
        });
        // dbg!(only_author);

        // return
        ReservedFolder {
            list_fetched_author_id: Some(only_author),
            ..Default::default()
        }
    }
    pub fn reindex_after_fetch_new_reviews(state_global: ArcMutStateGlobal) -> Self {
        unwrap!(state_global.lock()).review_index = review_index_mod::ReviewIndex::new();
        // return
        ReservedFolder {
            reindex_after_fetch_new_reviews: Some(s!("Reindex finished.")),
            ..Default::default()
        }
    }
    pub fn fetch_new_reviews(_state_global: ArcMutStateGlobal) -> Self {
        unwrap!(std::process::Command::new("bash")
            .arg("/var/www/scripts/cargo_crev_web_fetch_reindex.sh")
            .spawn());
        // return
        ReservedFolder {
            fetch_new_reviews: Some(s!("Fetch will be done in a minute or so.")),
            ..Default::default()
        }
    }
    pub async fn list_new_author_id(state_global: ArcMutStateGlobal) -> Self {
        // The repo https://gitlab.com/crev-dev/auto-crev-proofs.git
        // is automated to have all the crev repos it can find. It is also
        // possible to add repos manually.
        // I will clone and fetch that repo periodically
        // I will extract the data for adding new repos to rust-reviews.
        // on my local disk it is cached as:
        // .cache/crev/remotes/gitlab_com_chrysn_auto-crev-proofs-SQMK-9lvFGG0TNopVnQ0uQ/W-RXYmWCrsXJWinxMMdjCjR9ywGlH9srvMi0cmYL2rI/trust/
        // in the sample folder it is:
        // sample_data/cache/crev/remotes/gitlab_com_chrysn_auto-crev-proofs-SQMK-9lvFGG0TNopVnQ0uQ/W-RXYmWCrsXJWinxMMdjCjR9ywGlH9srvMi0cmYL2rI/trust/

        /*
        ids:
          - id-type: crev
            id: 24YKeuThJDN_FSlJy_xcl5diSZcKcRbh-0zXM0YxTOFJw
            url: "https://github.com/LucianoBestia/crev-proofs"
        */
        #[derive(Serialize, Deserialize, Clone, Debug)]
        struct ReviewIdsShort {
            pub id: String,
            pub url: Option<String>,
        }
        #[derive(Serialize, Deserialize, Clone, Debug)]
        struct ReviewShort {
            pub ids: Vec<ReviewIdsShort>,
        }
        let mut vec_of_auto_crev = Vec::<OnlyAuthor>::new();
        let mut vec_of_new = Vec::<OnlyAuthor>::new();
        let path = path_of_remotes_folder().join("gitlab_com_chrysn_auto-crev-proofs-SQMK-9lvFGG0TNopVnQ0uQ/W-RXYmWCrsXJWinxMMdjCjR9ywGlH9srvMi0cmYL2rI/trust");
        let path = path.to_string_lossy();
        //fill from all the files all the reviews
        for file_name in crev_files(&path).iter() {
            // iterator for reviews return &str
            let reviews_in_one_file = ReviewsInOneFile::new(file_name);
            for review_string in reviews_in_one_file {
                //dbg!(review_string);
                //fn push_author(review_string:&str, vec_of_new:&mut Vec<ReviewIdsShort>){
                let review_short: ReviewShort = unwrap!(serde_yaml::from_str(&review_string));

                vec_of_auto_crev.push(OnlyAuthor {
                    author_name: if let Some(url) = &review_short.ids[0].url {
                        author_name_from_url(&url)
                    } else {
                        s!("")
                    },
                    author_id: review_short.ids[0].id.clone(),
                    author_url: if let Some(url) = &review_short.ids[0].url {
                        url.clone()
                    } else {
                        s!("")
                    },
                });
                //dbg!(&vec_of_new);
            }
        }
        //dbg!(&vec_of_new);

        // region: first I need the list of fetched authors
        // I cannot construct this before await, because await can take a lot of time
        // and reference lifetime is in question?
        // so I must do it after await.
        // probably the Mutex is available everywhere, anytime ?
        use itertools::Itertools;
        let mut fetched_author_url: Vec<String> = unwrap!(state_global.lock())
            .review_index
            .vec
            .iter()
            .unique_by(|rev| &rev.author_url)
            .map(|rev| rev.author_url.clone())
            .collect();
        fetched_author_url.sort_by(|a, b| a.cmp(&b));
        // endregion: first I need the list of fetched authors

        // read blacklist_author_url from json file
        // TODO: make this editable from web UI

        let blacklisted_author_url = unwrap!(fs::read_to_string("blacklist_author_url.json"));
        let mut blacklisted_author_url: Vec<String> =
            unwrap!(serde_json::from_str(&blacklisted_author_url));

        blacklisted_author_url.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

        for auto_crev in vec_of_auto_crev.iter() {
            // if author already exists in index, I don't need it.
            // if author repo is in the "incomplete" list, I don't need it
            if !fetched_author_url
                .iter()
                .any(|v| v == &auto_crev.author_url)
                && !blacklisted_author_url
                    .iter()
                    .any(|v| v == &auto_crev.author_url)
            {
                vec_of_new.push(OnlyAuthor {
                    author_name: auto_crev.author_name.clone(),
                    author_id: auto_crev.author_id.clone(),
                    author_url: auto_crev.author_url.clone(),
                });
            }
        }

        // dbg!(vec_of_new.len());
        // dbg!( &vec_of_new);
        // return
        ReservedFolder {
            list_new_author_id: Some(vec_of_new),
            ..Default::default()
        }
    }

    pub async fn add_author_url(
        // this type guarantee that it has been decoded
        author_name: String,
        _state_global: ArcMutStateGlobal,
    ) -> Self {
        // in this fragment are 2 parts delimited with /
        // let split it and use parts one by one
        // dbg!(&author_name);
        let author_new = OnlyAuthor {
            author_name: s!(author_name),
            ..OnlyAuthor::default()
        };
        let author_url = url_u!("https://github.com/{}/crev-proofs", &author_new.author_name);
        let author_url = author_url.to_string();

        // find github content
        let gh_content_url = url_u!(
            "https://api.github.com/repos/{}/crev-proofs/contents",
            &author_new.author_name
        );
        let gh_content_url = gh_content_url.to_string();
        // dbg!(&gh_content_url);
        let resp_body = unwrap!(surf::get(&gh_content_url).recv_string().await);
        // the new format of proof
        // "name": "5X5SQsMDSEeY_uFOh9UOkkUiq8nt8ThA5ZJCHax5cu3hjM",
        // "size": 0,
        let mut author_id = s!("");
        let mut pos_cursor: usize = 0;
        // dbg!(&resp_body);
        loop {
            // first get the name, then get the size
            let range_name =
                find_range_between_delimiters(&resp_body, &mut pos_cursor, r#""name": ""#, r#"""#);
            if let Some(range_name) = range_name {
                // dbg!(&range_name);
                let range_size = find_range_between_delimiters(
                    &resp_body,
                    &mut pos_cursor,
                    r#""size": "#,
                    r#","#,
                );
                if let Some(range_size) = range_size {
                    // dbg!(&range_size);
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
        let add_author_url = if author_id.is_empty() {
            format!(
                "add author with these commands:<br/>
            cargo crev repo fetch url {}<br/>
            cargo crev id trust {}<br/>",
                &author_url.to_string(),
                &author_id
            )
        } else {
            s!("This repo is incomplete.")
        };
        // return
        ReservedFolder {
            add_author_url: Some(add_author_url),
            ..Default::default()
        }
    }
    /// return the item at cursor or default
    fn item_at_cursor_1(&self, subtemplate: &str, pos_cursor: usize) -> Option<&OnlyAuthor> {
        if subtemplate == "stmplt_authors" {
            if let Some(list) = &self.list_fetched_author_id {
                Some(&list[pos_cursor])
            } else {
                None
            }
        } else {
            None
        }
    }

    fn item_at_cursor_2(&self, subtemplate: &str, pos_cursor: usize) -> Option<&OnlyAuthor> {
        if subtemplate == "stmplt_authors_new" {
            if let Some(list) = &self.list_new_author_id {
                Some(&list[pos_cursor])
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl HtmlServerTemplateRender for ReservedFolder {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        // return
        s!("ReservedFolder")
    }
    /// renders the complete html file. Not a sub-template/fragment.
    fn render_html_file(&self, templates_folder_name: &str) -> String {
        let template_file_name = format!("{}reserved_folder_template.html", templates_folder_name);
        let html = self.render_from_file(&template_file_name);

        // return
        html
    }
    /// boolean : is the next node rendered or not
    fn retain_next_node_or_attribute(&self, placeholder: &str) -> bool {
        // dbg!(&placeholder);
        match placeholder {
            "sb_is_list_fetched_author_id" => self.list_fetched_author_id.is_some(),
            "sb_is_fetch_new_reviews" => self.fetch_new_reviews.is_some(),
            "sb_is_reindex_after_fetch_new_reviews" => {
                self.reindex_after_fetch_new_reviews.is_some()
            }
            "sb_list_new_author_id" => self.list_new_author_id.is_some(),
            "sb_add_author_url" => self.add_author_url.is_some(),
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
        subtemplate: &str,
        pos_cursor: usize,
    ) -> String {
        // dbg!(&placeholder);
        // list_fetched_author_id is Option and can be None or Some
        let only_author_empty = OnlyAuthor::default();
        let item_at_cursor_1 = self
            .item_at_cursor_1(subtemplate, pos_cursor)
            .unwrap_or(&only_author_empty);
        let item_at_cursor_2 = self
            .item_at_cursor_2(subtemplate, pos_cursor)
            .unwrap_or(&only_author_empty);
        match placeholder {
            "st_cargo_crev_web_version" => s!(env!("CARGO_PKG_VERSION")),
            "st_ordinal_number" => s!(pos_cursor + 1),
            "st_author_name_1" => s!(&item_at_cursor_1.author_name),
            "st_author_id" => s!(item_at_cursor_1.author_id),
            // same name from different data model is not allowed
            "st_author_name_2" => s!(item_at_cursor_2.author_name),
            "st_reindex_after_fetch_new_reviews" => {
                s!(unwrap!(self.reindex_after_fetch_new_reviews.as_ref()))
            }
            "st_fetch_new_reviews" => s!(unwrap!(self.fetch_new_reviews.as_ref())),
            "st_add_author_url" => s!(unwrap!(self.add_author_url.as_ref())),
            _ => replace_with_string_match_else(&self.data_model_name(), placeholder),
        }
    }
    /// exclusive url encoded for href and src
    fn replace_with_url(
        &self,
        placeholder: &str,
        subtemplate: &str,
        pos_cursor: usize,
    ) -> UrlUtf8EncodedString {
        let only_author_empty = OnlyAuthor::default();
        let item_at_cursor_1 = self
            .item_at_cursor_1(subtemplate, pos_cursor)
            .unwrap_or(&only_author_empty);
        let item_at_cursor_2 = self
            .item_at_cursor_2(subtemplate, pos_cursor)
            .unwrap_or(&only_author_empty);
        // dbg!( &placeholder);
        match placeholder {
            // the href for css is good for static data. For dynamic route it must be different.
            "su_css_route" => url_u!("/rust-reviews/css/rust-reviews.css"),
            "su_favicon_route" => url_u!("/rust-reviews/favicon.png"),
            "su_img_src_logo" => url_u!("/rust-reviews/images/Logo_02.png"),
            "su_author_url" => url_u!(&item_at_cursor_1.author_url, ""),
            "su_author_route" => url_u!("/rust-reviews/author/{}/", &item_at_cursor_1.author_id),
            "su_add_author_url_route" => url_u!(
                "/rust-reviews/reserved_folder/add_author_url/{}/",
                &item_at_cursor_2.author_name
            ),
            "su_author_url_2" => {
                let x = url_u!(
                    "https://github.com/{}/crev-proofs/",
                    &item_at_cursor_2.author_name
                );
                //dbg!(&x);
                //return
                x
            }
            _ => replace_with_url_match_else(&self.data_model_name(), placeholder),
        }
    }
    /// returns a vector of Nodes to replace the next Node
    #[allow(clippy::needless_return)]
    fn replace_with_nodes(&self, placeholder: &str) -> Vec<Node> {
        // dbg!(&placeholder);
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
        // dbg!( &placeholder);
        match template_name {
            "stmplt_authors" => {
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
                            template_name,
                            cursor_for_vec,
                        ));
                        nodes.extend_from_slice(&vec_node);
                    }
                }
                // return
                nodes
            }
            "stmplt_authors_new" => {
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
