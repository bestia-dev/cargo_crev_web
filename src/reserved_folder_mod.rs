//! reserved_folder_mod
//! This is only one module/html page, but can execute different actions.
//! The data model must have fields for every action as Option<>.
//! Because only this data can influence the html render.
//! There are different "new" functions for different actions, to prepare adequate data.
//! If field is is_some(), then render the html part dedicated to this action.

use crate::*;

use std::fs;
use unwrap::unwrap;

#[derive(Debug, Default)]
pub struct OnlyReviewer {
    pub reviewer_name: String,
    pub reviewer_id: String,
    pub reviewer_url: String,
}

#[derive(Debug, Default)]
pub struct DailyVisitors {
    pub date: String,
    pub visitors: String,
    pub requests: String,
}

//use unwrap::unwrap;
#[derive(Debug, Default)]
pub struct ReservedFolder {
    pub list_trusted_reviewer_id: Option<Vec<OnlyReviewer>>,
    pub blocklisted_repos: Option<Vec<(String, String)>>,
    pub daily_visitors: Option<Vec<DailyVisitors>>,
}

impl ReservedFolder {
    /// prepares the data
    pub fn new(_state_global: ArcMutStateGlobal) -> Self {
        // return
        ReservedFolder { ..Default::default() }
    }
    pub fn list_trusted_reviewer_id(state_global: ArcMutStateGlobal) -> Self {
        // dbg!(reviewer_index);
        let mut only_reviewer: Vec<OnlyReviewer> = unwrap!(state_global.lock())
            .reviewer_index
            .vec
            .iter()
            .map(|r| OnlyReviewer {
                reviewer_name: r.name.clone(),
                reviewer_id: r.id.clone(),
                reviewer_url: r.url.clone(),
            })
            .collect();
        only_reviewer.sort_by(|a, b| a.reviewer_name.to_lowercase().cmp(&b.reviewer_name.to_lowercase()));
        // return
        ReservedFolder {
            list_trusted_reviewer_id: Some(only_reviewer),
            ..Default::default()
        }
    }

    pub fn blocklisted_repos(_state_global: ArcMutStateGlobal) -> Self {
        let mut reserved_folder = ReservedFolder { ..Default::default() };
        reserved_folder.fill_blocklisted_repos();
        //return
        reserved_folder
    }
    /// read blocklisted_repos from json file
    fn fill_blocklisted_repos(&mut self) {
        let blocklisted_repos = unwrap!(fs::read_to_string("blocklisted_repos.json"));
        let mut blocklisted_repos: Vec<(String, String)> = unwrap!(serde_json::from_str(&blocklisted_repos));

        blocklisted_repos.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
        self.blocklisted_repos = Some(blocklisted_repos);
    }

    pub fn daily_visitors(_state_global: ArcMutStateGlobal) -> Self {
        // dbg!(reviewer_index);
        let daily_visitors = crate::daily_visitors_mod::read_nginx_log_and_fill_daily_visitors();
        // return
        ReservedFolder {
            daily_visitors: Some(daily_visitors),
            ..Default::default()
        }
    }

    /// return the item at cursor or default
    fn item_at_cursor_1(&self, subtemplate: &str, pos_cursor: usize) -> Option<&OnlyReviewer> {
        if subtemplate == "stmplt_reviewers" {
            if let Some(list) = &self.list_trusted_reviewer_id {
                Some(&list[pos_cursor])
            } else {
                None
            }
        } else {
            None
        }
    }

    fn item_at_cursor_3(&self, subtemplate: &str, pos_cursor: usize) -> Option<&DailyVisitors> {
        if subtemplate == "stmplt_daily_visitors" {
            if let Some(list) = &self.daily_visitors {
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
            "sb_is_list_trusted_reviewer_id" => self.list_trusted_reviewer_id.is_some(),
            "sb_blocklisted_repos" => self.blocklisted_repos.is_some(),
            "sb_daily_visitors" => self.daily_visitors.is_some(),
            _ => retain_next_node_or_attribute_match_else(&self.data_model_name(), placeholder),
        }
    }

    /// returns a String to replace the next text-node
    #[allow(clippy::needless_return, clippy::integer_arithmetic, clippy::indexing_slicing)]
    fn replace_with_string(&self, placeholder: &str, subtemplate: &str, pos_cursor: usize) -> String {
        // dbg!(&placeholder);
        // list_trusted_reviewer_id is Option and can be None or Some
        let only_reviewer_empty = OnlyReviewer::default();
        let daily_visitors_empty = DailyVisitors::default();
        let item_at_cursor_1 = self.item_at_cursor_1(subtemplate, pos_cursor).unwrap_or(&only_reviewer_empty);
        let item_at_cursor_3 = self.item_at_cursor_3(subtemplate, pos_cursor).unwrap_or(&daily_visitors_empty);
        match placeholder {
            "st_cargo_crev_web_version" => s!(env!("CARGO_PKG_VERSION")),
            "st_ordinal_number" => s!(pos_cursor + 1),
            "st_reviewer_name_1" => s!(&item_at_cursor_1.reviewer_name),
            "st_reviewer_id" => s!(item_at_cursor_1.reviewer_id),
            // same name from different data model is not allowed
            "st_repo_url" => s!(unwrap!(self.blocklisted_repos.as_ref())[pos_cursor].0),
            "st_blocklist_note" => s!(unwrap!(self.blocklisted_repos.as_ref())[pos_cursor].1),

            "st_date" => s!(item_at_cursor_3.date),
            "st_visitors" => s!(item_at_cursor_3.visitors),
            "st_requests" => s!(item_at_cursor_3.requests),
            _ => replace_with_string_match_else(&self.data_model_name(), placeholder),
        }
    }
    /// exclusive url encoded for href and src
    fn replace_with_url(&self, placeholder: &str, subtemplate: &str, pos_cursor: usize) -> UrlUtf8EncodedString {
        let only_reviewer_empty = OnlyReviewer::default();
        let item_at_cursor_1 = self.item_at_cursor_1(subtemplate, pos_cursor).unwrap_or(&only_reviewer_empty);
        // dbg!( &placeholder);
        match placeholder {
            // the href for css is good for static data. For dynamic route it must be different.
            "su_css_route" => url_u!("/rust-reviews/css/rust-reviews.css"),
            "su_favicon_route" => url_u!("/rust-reviews/favicon.png"),
            "su_img_src_logo" => url_u!("/rust-reviews/images/Logo_02.png"),
            "su_reviewer_url" => url_u!(&item_at_cursor_1.reviewer_url, ""),
            "su_repo_url" => url_u!(&unwrap!(self.blocklisted_repos.as_ref())[pos_cursor].0, ""),
            "su_reviewer_route" => {
                url_u!("/rust-reviews/reviewer/{}/", &item_at_cursor_1.reviewer_id)
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
    fn render_sub_template(&self, template_name: &str, sub_templates: &Vec<SubTemplate>) -> Vec<Node> {
        // dbg!( &placeholder);
        match template_name {
            "stmplt_reviewers" => {
                let mut nodes = vec![];
                if let Some(list) = &self.list_trusted_reviewer_id {
                    let sub_template = unwrap!(sub_templates.iter().find(|&template| template.name == template_name));
                    // sub-template repeatable
                    for cursor_for_vec in 0..list.len() {
                        let vec_node = unwrap!(self.render_template_raw_to_nodes(&sub_template.template, HtmlOrSvg::Html, template_name, cursor_for_vec,));
                        nodes.extend_from_slice(&vec_node);
                    }
                }
                // return
                nodes
            }
            "stmplt_blocklisted_repos" => {
                let mut nodes = vec![];
                if let Some(list) = &self.blocklisted_repos {
                    let sub_template = unwrap!(sub_templates.iter().find(|&template| template.name == template_name));
                    // sub-template repeatable
                    for cursor_for_vec in 0..list.len() {
                        let vec_node = unwrap!(self.render_template_raw_to_nodes(&sub_template.template, HtmlOrSvg::Html, template_name, cursor_for_vec));
                        nodes.extend_from_slice(&vec_node);
                    }
                }
                // return
                nodes
            }
            "stmplt_daily_visitors" => {
                let mut nodes = vec![];
                if let Some(list) = &self.daily_visitors {
                    let sub_template = unwrap!(sub_templates.iter().find(|&template| template.name == template_name));
                    // sub-template repeatable
                    for cursor_for_vec in 0..list.len() {
                        let vec_node = unwrap!(self.render_template_raw_to_nodes(&sub_template.template, HtmlOrSvg::Html, template_name, cursor_for_vec));
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
