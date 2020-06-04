//! router_mod

// region: (collapsed) use statements
use crate::*;

// I must put the trait in scope
use crate::url_utf8_mod::*;
use crate::utils_mod::*;

//use futures::{sync::mpsc, Future, Stream};
#[allow(unused_imports)]
use ansi_term::Colour::{Blue, Green, Red, Yellow};
use log::info;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
//use unwrap::unwrap;
use warp::{http::Response, Filter};

// end region: (collapsed) use statements

pub async fn start_routes(
    cached_review_index: Arc<Mutex<review_index_mod::ReviewIndex>>,
    local_addr: SocketAddr,
) {
    // websites are mostly always made of more separate web-apps
    // it is good for web-apps to NOT start from the website root
    // this webapp starts with the route website_url/rust-reviews/
    // example: web.crev.dev/rust-reviews/crate/num-traits
    //   or : 127.0.0.1:8051/rust-reviews/crate/num-traits
    // that way is easy to publish it on different websites.
    // if they have this route not taken.

    // region: prepare routes

    // Turn our "state" into a new Filter...
    let cached_review_index = warp::any().map(move || cached_review_index.clone());

    // static files and folders:
    // /rust-reviews/css/*  - static css file
    // /rust-reviews/favicon.png  - static file

    // dynamic content:
    // /rust-reviews/
    // /rust-reviews/index.html
    // /rust-reviews/author/{author_id}/
    // /rust-reviews/badge/crev_count/{crate_name}.svg
    // /rust-reviews/crate/{crate_name}/
    // /rust-reviews/crate/{crate_name}/{version}/
    // /rust-reviews/crate/{crate_name}/{version}/{kind}/
    // /rust-reviews/crates/
    // /rust-reviews/authors/
    // /rust-reviews/review_new/
    // /rust-reviews/review_new_to_yaml/
    // /rust-reviews/reserved_folder/
    // /rust-reviews/reserved_folder/reindex_after_fetch_new_reviews/
    // /rust-reviews/reserved_folder/list_new_author_id/
    // /rust-reviews/reserved_folder/add_author_url/
    // /rust-reviews/reserved_folder/list_fetched_author_id/

    // the crate_name must finish with .svg
    let badge_route = warp::path!("rust-reviews" / "badge" / "crev_count" / UrlPartUtf8Decoded)
        .and(cached_review_index.clone())
        .map(|crate_name: UrlPartUtf8Decoded, cached_review_index| {
            let ns_start = ns_start("badge");
            let crate_name = crate_name.to_string();
            // remove suffix .svg
            let trimmed_str: &str = crate_name.trim_end_matches(".svg");
            let data_model = badge_mod::Badge::crev_count(trimmed_str, cached_review_index);
            dbg!(&data_model);
            let ns_new = ns_print("new()", ns_start);
            let html_file = data_model.render_html_file("templates/");
            ns_print("render_html_file()", ns_new);
            let reply = Response::builder()
                .header("content-type", "image/svg+xml")
                // for github I allow 1 hour caching of img/badge.
                // Because I fetch new data every hour.
                .header("Cache-Control", "max-age=3600, public")
                .body(html_file);
            //return
            reply
        });

    let review_new_route = warp::path!("rust-reviews" / "review_new")
        .map(|| {
            let ns_start = ns_start("review_new");
            //let data_model = review_new_mod::ReviewNew::new();
            let data_model = review_new_mod::ReviewNew::read_review("review_1.yaml");
            let ns_new = ns_print("new()", ns_start);
            let html_file = data_model.render_html_file("templates/");
            ns_print("render_html_file()", ns_new);
            warp::reply::html(html_file)
        })
        .or(warp::path!("rust-reviews" / "review_new_to_yaml")
            .and(warp::body::content_length_limit(1024 * 32))
            .and(warp::body::form())
            .map(|form_data: HashMap<String, String>| {
                let ns_start = ns_start("review_new_to_yaml");
                let data_model = review_new_mod::ReviewNew::from_form_data(form_data);
                let ns_new = ns_print("new()", ns_start);
                let html_file = data_model.render_html_file("templates/");
                ns_print("render_html_file()", ns_new);
                warp::reply::html(html_file)
            }));

    let reserved_folder_route =
        warp::path!("rust-reviews" / "reserved_folder" / "reindex_after_fetch_new_reviews")
            .and(cached_review_index.clone())
            .map(|cached_review_index| {
                let ns_start = ns_start("reindex_after_fetch_new_reviews");
                let data_model =
                    reserved_folder_mod::ReservedFolder::reindex_after_fetch_new_reviews(
                        cached_review_index,
                    );
                let ns_new = ns_print("new()", ns_start);
                let html_file = data_model.render_html_file("templates/");
                ns_print("render_html_file()", ns_new);
                warp::reply::html(html_file)
            })
            .or(
                warp::path!("rust-reviews" / "reserved_folder" / "list_new_author_id")
                    .and(cached_review_index.clone())
                    .and_then(|cached_review_index| async move {
                        let ns_start = ns_start("list_new_author_id");
                        let data_model = reserved_folder_mod::ReservedFolder::list_new_author_id(
                            cached_review_index,
                        )
                        .await;
                        let ns_new = ns_print("new()", ns_start);
                        let html_file = data_model.render_html_file("templates/");
                        ns_print("render_html_file()", ns_new);
                        // return crazy types
                        let result: Result<Box<dyn warp::Reply>, warp::Rejection> =
                            Ok(Box::new(warp::reply::html(html_file)) as Box<dyn warp::Reply>);
                        result
                    }),
            )
            .or(warp::path!(
                "rust-reviews" / "reserved_folder" / "add_author_url" / UrlPartUtf8Decoded
            )
            .and(cached_review_index.clone())
            .and_then(
                |author_name: UrlPartUtf8Decoded, cached_review_index| async move {
                    let ns_start = ns_start("add_author_url");
                    let author_name = author_name.to_string();
                    // in this fragment are 2 parts delimited with /, so it must be encoded
                    // after decoding looks like "scott-wilson/crev-proofs"
                    dbg!(&author_name);
                    let data_model = reserved_folder_mod::ReservedFolder::add_author_url(
                        author_name,
                        cached_review_index,
                    )
                    .await;
                    let ns_new = ns_print("new()", ns_start);
                    let html_file = data_model.render_html_file("templates/");
                    ns_print("render_html_file()", ns_new);
                    // return crazy types
                    let result: Result<Box<dyn warp::Reply>, warp::Rejection> =
                        Ok(Box::new(warp::reply::html(html_file)) as Box<dyn warp::Reply>);
                    result
                },
            ))
            .or(
                warp::path!("rust-reviews" / "reserved_folder" / "list_fetched_author_id")
                    .and(cached_review_index.clone())
                    .map(|cached_review_index| {
                        let ns_start = ns_start("list_fetched_author_id");
                        let data_model =
                            reserved_folder_mod::ReservedFolder::list_fetched_author_id(
                                cached_review_index,
                            );
                        let ns_new = ns_print("new()", ns_start);
                        let html_file = data_model.render_html_file("templates/");
                        ns_print("render_html_file()", ns_new);
                        warp::reply::html(html_file)
                    }),
            )
            .or(warp::path!("rust-reviews" / "reserved_folder")
                .and(cached_review_index.clone())
                .map(|cached_review_index| {
                    let ns_start = ns_start("reserved_folder");
                    let data_model = reserved_folder_mod::ReservedFolder::new(cached_review_index);
                    // dbg!( data_model);
                    let ns_new = ns_print("new()", ns_start);
                    let html_file = data_model.render_html_file("templates/");
                    ns_print("render_html_file()", ns_new);
                    warp::reply::html(html_file)
                }));

    let root_route = warp::path!("rust-reviews")
        .and(cached_review_index.clone())
        .map(|cached_review_index| {
            let ns_start = ns_start("ReviewIndexSummary");
            let data_model = review_index_summary_mod::ReviewIndexSummary::new(cached_review_index);
            let ns_new = ns_print("new()", ns_start);
            let html_file = data_model.render_html_file("templates/");
            ns_print("render_html_file()", ns_new);
            warp::reply::html(html_file)
        })
        .or(warp::path!("rust-reviews" / "index.html")
            .and(cached_review_index.clone())
            .map(|cached_review_index| {
                let ns_start = ns_start("ReviewIndexSummary");
                let data_model =
                    review_index_summary_mod::ReviewIndexSummary::new(cached_review_index);
                let ns_new = ns_print("new()", ns_start);
                let html_file = data_model.render_html_file("templates/");
                ns_print("render_html_file()", ns_new);
                warp::reply::html(html_file)
            }))
        .or(warp::path!("rust-reviews" / "crates")
            .and(cached_review_index.clone())
            .map(|cached_review_index| {
                let ns_start = ns_start("ReviewIndexByCrate");
                let data_model = crates_mod::ReviewIndexByCrate::new(cached_review_index);
                let ns_new = ns_print("new()", ns_start);
                let html_file = data_model.render_html_file("templates/");
                ns_print("render_html_file()", ns_new);
                warp::reply::html(html_file)
            }))
        .or(warp::path!("rust-reviews" / "authors")
            .and(cached_review_index.clone())
            .map(|cached_review_index| {
                let ns_start = ns_start("ReviewIndexByAuthor");
                let data_model = authors_mod::ReviewIndexByAuthor::new(cached_review_index);
                let ns_new = ns_print("new()", ns_start);
                let html_file = data_model.render_html_file("templates/");
                ns_print("render_html_file()", ns_new);
                warp::reply::html(html_file)
            }));

    let author_route = warp::path!("rust-reviews" / "author" / UrlPartUtf8Decoded)
        .and(cached_review_index.clone())
        .map(|author_id: UrlPartUtf8Decoded, cached_review_index| {
            let author_id = author_id.to_string();
            let ns_start = ns_start(&format!(
                "AuthorReviews author_name: '{}'",
                Yellow.paint(&author_id),
            ));
            let data_model =
                author_reviews_mod::AuthorReviews::new(cached_review_index, &author_id);
            let ns_new = ns_print("new()", ns_start);
            let html_file = data_model.render_html_file("templates/");
            ns_print("render_html_file()", ns_new);
            warp::reply::html(html_file)
        });

    let crate_route = warp::path!("rust-reviews" / "crate" / UrlPartUtf8Decoded)
        .and(cached_review_index.clone())
        .map(|crate_name: UrlPartUtf8Decoded, cached_review_index| {
            let crate_name = crate_name.to_string();
            let ns_start = ns_start(&format!(
                "CrateReviews crate_name: '{}'",
                Yellow.paint(&crate_name),
            ));

            let data_model =
                crate_reviews_mod::CrateReviews::new(cached_review_index, &crate_name, "", "");
            let ns_new = ns_print("new()", ns_start);
            let html_file = data_model.render_html_file("templates/");
            ns_print("render_html_file()", ns_new);
            warp::reply::html(html_file)
        })
        .or(
            warp::path!("rust-reviews" / "crate" / UrlPartUtf8Decoded / UrlPartUtf8Decoded)
                .and(cached_review_index.clone())
                .map(
                    |crate_name: UrlPartUtf8Decoded,
                     version: UrlPartUtf8Decoded,
                     cached_review_index| {
                        let crate_name = crate_name.to_string();
                        let version = version.to_string();
                        let ns_start = ns_start(&format!(
                            "CrateReviews crate_name: '{}', version '{}'",
                            Yellow.paint(&crate_name),
                            Yellow.paint(&version),
                        ));

                        let data_model = crate_reviews_mod::CrateReviews::new(
                            cached_review_index,
                            &crate_name,
                            &version,
                            "",
                        );
                        let ns_new = ns_print("new()", ns_start);
                        let html_file = data_model.render_html_file("templates/");
                        ns_print("render_html_file()", ns_new);
                        warp::reply::html(html_file)
                    },
                ),
        )
        .or(warp::path!(
            "rust-reviews" / "crate" / UrlPartUtf8Decoded / UrlPartUtf8Decoded / UrlPartUtf8Decoded
        )
        .and(cached_review_index.clone())
        .map(
            |crate_name: UrlPartUtf8Decoded,
             version: UrlPartUtf8Decoded,
             kind: UrlPartUtf8Decoded,
             cached_review_index| {
                let crate_name = crate_name.to_string();
                let version = version.to_string();
                let kind = kind.to_string();
                let ns_start = ns_start(&format!(
                    "CrateReviews crate_name: '{}', version '{}', kind '{}'",
                    Yellow.paint(&crate_name),
                    Yellow.paint(&version),
                    Yellow.paint(&kind)
                ));

                let data_model = crate_reviews_mod::CrateReviews::new(
                    cached_review_index,
                    &crate_name,
                    &version,
                    &kind,
                );
                let ns_new = ns_print("new()", ns_start);
                let html_file = data_model.render_html_file("templates/");
                ns_print("render_html_file()", ns_new);
                warp::reply::html(html_file)
            },
        ));

    // static file server (starts at rust-reviews)
    // route /rust-reviews/ get files from folder ./web_content_folder/
    let fileserver = warp::path("rust-reviews").and(warp::fs::dir("./web_content_folder/"));
    // endregion: prepare routes

    // combine all routes with or
    let routes = crate_route
        .or(author_route)
        .or(root_route)
        .or(reserved_folder_route)
        .or(review_new_route)
        .or(badge_route)
        .or(fileserver);

    info!(
        "Entry point sub-directory: {} ",
        Red.paint(s!("/rust-reviews/"))
    );

    warp::serve(routes).run(local_addr).await;
}
