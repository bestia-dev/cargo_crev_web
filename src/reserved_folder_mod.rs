//! reserved_folder_mod
//! This is only one module/html page, but can execute different actions.
//! The data model must have fields for every action as Option<>.
//! Because only this data can influence the html render.
//! There are different "new" functions for different actions, to prepare adequate data.
//! If field is is_some(), then render the html part dedicated to this action.

use crate::review_index_mod;
use crate::*;

use log::debug;
use serde_derive::{Deserialize, Serialize};
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
    pub reindex_after_fetch_new_reviews: Option<String>,
    pub fetch_new_reviews: Option<String>,
    pub blocklisted_repos: Option<Vec<(String, String)>>,
    pub list_new_reviewer_id: Option<Vec<OnlyReviewer>>,
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
    pub fn reindex_after_fetch_new_reviews(state_global: ArcMutStateGlobal) -> Self {
        unwrap!(state_global.lock()).review_index = review_index_mod::ReviewIndex::new();
        unwrap!(state_global.lock()).reviewer_index = reviewer_index_mod::ReviewerIndex::new();
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

    /// The command `cargo crev id query all` returns a list of repos found in
    /// all trust crev files: mine (/home/me/.config/crev) and from all the remotes (/home/me/.cache/crev/remotes).
    /// I used for some time the repo <https://gitlab.com/crev-dev/auto-crev-proofs.git>.
    /// It finds new repos automatically, but it looks that it is not working any more.
    /// I will do it myself: find all forked crev-proofs on github and gitlab and `cargo crev id query all`.
    /// There are also some manually added repos in:
    /// <https://github.com/crev-dev/cargo-crev/wiki/List-of-Proof-Repositories>
    /// But it looks that this list is obsolete.
    /// I will add all the new repos with `cargo crev trust --level low <url>`.
    /// Ajooj: this automatically fetches the proofs with all transient. I don't want that.
    /// Warning: `cargo crev id trust --level low <hash>` sounds very similar, but it accepts only hash, not url!
    /// And without url it is not possible to find the hash.
    /// For list only directly trusted identities:
    /// `cargo crev id query trusted --high-cost 1 --medium-cost 1 --low-cost 1 --depth 1`
    /// This way I can then use `cargo crev repo fetch trusted --high-cost 1 --medium-cost 1 --low-cost 1 --depth 1`
    ///
    /// Warning: never put passwords, passphrases ot tokens inside the code and publish it to Github !
    /// If you wrote some secret in the bash like `export PASS=my_secret`, delete it from the history with this command:
    /// history | tac | grep export | cut -d ' ' -f 2 | awk '{printf "history -d " $1 "; "}'
    ///
    /// Warning: for `cargo crev trust` to work I need the crev passphrase.
    /// I will paste the passphrase from the clipboard to the env variable before starting the server:
    /// `export CREV_PASSPHRASE=$(pbpaste)`
    /// To use pbpaste and pbcopy in Debian on WSL2 use this instructions: <https://www.techtronic.us/pbcopy-pbpaste-for-wsl/>.
    /// But if `clip.exe` does not work you need to do this first to your /etc/profile file: <https://github.com/microsoft/WSL/issues/5779#issuecomment-675574471>
    /// Use `sudo nano /etc/profile`. I rather comment the lines with #. And after that, restart wsl in Administrative mode Cmd in Win10: `wsl -shutdown`.
    /// Aaaand it worked locally in my WSL, but it didn't work on my google vm over SSH. So I used this other technique:  
    /// Add a space before the linux command to avoid it to be stored in the bash history.  
    ///
    /// For unauthenticated requests on Github, the rate limit allows for up to 60 requests per hour.
    /// So I need to be authenticated for Github api with [github PAT (personal access token)](https://docs.github.com/en/github/authenticating-to-github/keeping-your-account-and-data-secure/creating-a-personal-access-token)
    /// Before running the web server I store it in the environment variable:  
    /// `export GITHUB_TOKEN=$(pbpaste)`  or add a space before the command to avoid it being stored in the bash history.  
    pub async fn list_new_reviewer_id(state_global: ArcMutStateGlobal) -> Self {
        fn check_repo_on_github(
            forked_repo: &ForkedRepo,
            client: &reqwest::blocking::Client,
            url_for_content: &str,
            blocklisted_repos: &mut Vec<(String, String)>,
            vec_of_new_repo: &mut Vec<OnlyReviewer>,
        ) {
            // dbg!(&forked_repo.html_url);
            if !forked_repo.html_url.starts_with("https://github.com") {
                println!("Not on github: {}", &forked_repo.html_url);
                vec_of_new_repo.push(OnlyReviewer {
                    reviewer_name: "".to_string(),
                    reviewer_id: "".to_string(),
                    reviewer_url: forked_repo.html_url.clone(),
                });
            } else {
                let response = client
                    .get(url_for_content)
                    .header("User-Agent", "cargo_crev_web (github.com/LucianoBestia/cargo_crev_web)")
                    .header("authorization", &format!("Bearer {}", unwrap!(std::env::var("GITHUB_TOKEN"))))
                    .send()
                    .unwrap();
                let response_text = response.text().unwrap_or("".to_string());
                if response_text.is_empty() {
                    // add this url to blocklist.json
                    blocklisted_repos.push((forked_repo.html_url.to_string(), "url not exist".to_string()));
                    println!("Error for call to url: {}", &url_for_content);
                } else {
                    let rsl = serde_json::from_str::<Vec<RepoContent>>(&response_text);
                    match rsl {
                        Err(_err) => debug!("Cannot deserialize: {:?}", &response_text),
                        Ok(vec_repo_content) => {
                            let mut count_ids = 0;
                            for content in vec_repo_content.iter() {
                                // "name": "24YKeuThJDNFSlJyxcl5diSZcKcRbh-0zXM0YxTOFJw",
                                // "type": "dir",
                                if content.name.len() == 43 && content.r#type == "dir" {
                                    count_ids += 1;
                                    // dbg!("    {} {}", content.name, forked_repo.html_url);
                                    vec_of_new_repo.push(OnlyReviewer {
                                        reviewer_name: reviewer_name_from_url(&forked_repo.html_url),
                                        reviewer_id: content.name.clone(),
                                        reviewer_url: forked_repo.html_url.clone(),
                                    });
                                }
                            }
                            // if there is no id in the repo then add it to blocklisted
                            if count_ids == 0 {
                                blocklisted_repos.push((forked_repo.html_url.to_string(), "no id".to_string()));
                            }
                        }
                    }
                }
            }
        }
        let mut reserved_folder = ReservedFolder { ..Default::default() };
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
        #[derive(Serialize, Deserialize, Debug)]
        struct ForkedRepo {
            html_url: String,
            contents_url: String,
        }
        #[derive(Serialize, Deserialize, Debug)]
        struct VecOfForkedRepos {
            vec: Vec<ForkedRepo>,
        }
        #[derive(Serialize, Deserialize, Debug)]
        struct RepoContent {
            name: String,
            r#type: String,
        }

        reserved_folder.fill_blocklisted_repos();
        let mut blocklisted_repos = reserved_folder.blocklisted_repos.as_ref().unwrap().clone();

        let client = reqwest::blocking::Client::new();
        let mut vec_of_new_repo = Vec::<OnlyReviewer>::new();
        let mut page = 1;
        loop {
            // there can be more pages. Max per_page is 100
            let url_for_page = &format!("https://api.github.com/repos/crev-dev/crev-proofs/forks?per_page=100&page={}", page);

            let response = client
                .get(url_for_page)
                .header("User-Agent", "cargo_crev_web (github.com/LucianoBestia/cargo_crev_web)")
                .header("authorization", &format!("Bearer {}", unwrap!(std::env::var("GITHUB_TOKEN"))))
                .send()
                .unwrap();
            let response_text = response.text().unwrap_or("".to_string());
            if response_text.is_empty() {
                println!("Error for call to url: {}", &url_for_page);
                break;
            } else {
                let vec_forked_repo: Vec<ForkedRepo> = serde_json::from_str(&response_text).unwrap();
                for forked_repo in vec_forked_repo.iter() {
                    // "html_url": "https://github.com/dcsommer/crev-proofs",
                    // "contents_url": "https://api.github.com/repos/dcsommer/crev-proofs/contents/{+path}",
                    let url_for_content = forked_repo.contents_url.trim_end_matches("/{+path}");

                    // control in reviewer_index and blocklist to avoid futile calls to api
                    if unwrap!(state_global.lock()).reviewer_index.vec.iter().any(|x| x.url == forked_repo.html_url) {
                        // println!("Reviewer_index already contains: {}", forked_repo.html_url);
                    } else if blocklisted_repos.iter().any(|x| x.0 == forked_repo.html_url) {
                        //println!("Blocklisted already contains: {}", forked_repo.html_url);
                    } else {
                        check_repo_on_github(forked_repo, &client, url_for_content, &mut blocklisted_repos, &mut vec_of_new_repo);
                    }
                }
                // the last page has less then 100 items
                if vec_forked_repo.len() < 100 {
                    break;
                }
                page += 1;
            }
        }

        let mut query_vec: Vec<String> = vec![];
        let output = unwrap!(std::process::Command::new("cargo").args(["crev", "id", "query", "all"]).output());
        let query_all = output.stdout;
        let query_all = unwrap!(String::from_utf8(query_all));
        for line in query_all.lines() {
            let splitted: Vec<&str> = line.split_whitespace().collect();
            query_vec.push(splitted[3].to_string());
        }
        for html_url in query_vec.iter() {
            let url_for_content = format!("https://api.github.com/repos/{}/contents/", html_url.trim_start_matches("https://github.com/"));
            let query_all_repo = ForkedRepo {
                html_url: html_url.to_string(),
                contents_url: url_for_content.to_string(),
            };
            // "html_url": "https://github.com/dcsommer/crev-proofs",
            // "contents_url": "https://api.github.com/repos/dcsommer/crev-proofs/contents/",

            // control in reviewer_index and blocklist to avoid futile calls to api
            if unwrap!(state_global.lock()).reviewer_index.vec.iter().any(|x| x.url == query_all_repo.html_url) {
                // println!("Reviewer_index already contains: {}", forked_repo.html_url);
            } else if blocklisted_repos.iter().any(|x| x.0 == query_all_repo.html_url) {
                // println!("Blocklisted already contains: {}", forked_repo.html_url);
            } else {
                dbg!(&query_all_repo.html_url);
                check_repo_on_github(&query_all_repo, &client, &url_for_content, &mut blocklisted_repos, &mut vec_of_new_repo);
            }
        }

        // save the blocklist.json
        blocklisted_repos.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
        let blocklisted_repos_json = unwrap!(serde_json::to_string_pretty(&blocklisted_repos));
        unwrap!(fs::write("blocklisted_repos.json", &blocklisted_repos_json));

        vec_of_new_repo.sort_by(|a, b| a.reviewer_name.to_lowercase().cmp(&b.reviewer_name.to_lowercase()));

        reserved_folder.list_new_reviewer_id = Some(vec_of_new_repo);
        // return
        reserved_folder
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

    fn item_at_cursor_2(&self, subtemplate: &str, pos_cursor: usize) -> Option<&OnlyReviewer> {
        if subtemplate == "stmplt_reviewers_new" {
            if let Some(list) = &self.list_new_reviewer_id {
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
            "sb_is_fetch_new_reviews" => self.fetch_new_reviews.is_some(),
            "sb_is_reindex_after_fetch_new_reviews" => self.reindex_after_fetch_new_reviews.is_some(),
            "sb_blocklisted_repos" => self.blocklisted_repos.is_some() && self.list_new_reviewer_id.is_none(),
            "sb_list_new_reviewer_id" => self.list_new_reviewer_id.is_some(),
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
        let item_at_cursor_2 = self.item_at_cursor_2(subtemplate, pos_cursor).unwrap_or(&only_reviewer_empty);
        let item_at_cursor_3 = self.item_at_cursor_3(subtemplate, pos_cursor).unwrap_or(&daily_visitors_empty);
        match placeholder {
            "st_cargo_crev_web_version" => s!(env!("CARGO_PKG_VERSION")),
            "st_ordinal_number" => s!(pos_cursor + 1),
            "st_reviewer_name_1" => s!(&item_at_cursor_1.reviewer_name),
            "st_reviewer_id" => s!(item_at_cursor_1.reviewer_id),
            // same name from different data model is not allowed
            "st_reviewer_name_2" => s!(item_at_cursor_2.reviewer_name),
            "st_reviewer_url_2" => s!(item_at_cursor_2.reviewer_url),
            "st_reviewer_id_2" => s!(item_at_cursor_2.reviewer_id),
            "st_reindex_after_fetch_new_reviews" => {
                s!(unwrap!(self.reindex_after_fetch_new_reviews.as_ref()))
            }
            "st_fetch_new_reviews" => s!(unwrap!(self.fetch_new_reviews.as_ref())),
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
        let item_at_cursor_2 = self.item_at_cursor_2(subtemplate, pos_cursor).unwrap_or(&only_reviewer_empty);
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
            "su_reviewer_url_2" => {
                let x = url_u!(&item_at_cursor_2.reviewer_url, "");
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
            "stmplt_reviewers_new" => {
                let mut nodes = vec![];
                if let Some(list) = &self.list_new_reviewer_id {
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
