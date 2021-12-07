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
//use unwrap::unwrap;
use warp::{
    //filters::BoxedFilter,
    http::Response,
    Filter,
};

// https://github.com/rs-ipfs/rust-ipfs/commit/ae3306686209afa5911b1ad02170c1ac3bacda7c
/// Helper to combine the multiple filters together with Filter::or, possibly boxing the types in
/// the process. This greatly helps the build times for `ipfs-http`.
macro_rules! combine {
    ($x:expr, $($y:expr),+) => {
        {
            let filter = boxed_on_debug!($x);
            $(
                let filter = boxed_on_debug!(filter.or($y));
            )+
            filter
        }
    }
}

#[cfg(debug_assertions)]
macro_rules! boxed_on_debug {
    ($x:expr) => {
        $x.boxed()
    };
}

#[cfg(not(debug_assertions))]
macro_rules! boxed_on_debug {
    ($x:expr) => {
        $x
    };
}

// end region: (collapsed) use statements

pub async fn start_routes(state_global: ArcMutStateGlobal, local_addr: SocketAddr) {
    // websites are mostly always made of more separate web-apps
    // it is good for web-apps to NOT start from the website root
    // this webapp starts with the route website_url/rust-reviews/
    // example: web.crev.dev/rust-reviews/crate/num-traits
    //   or : 127.0.0.1:8051/rust-reviews/crate/num-traits
    // that way is easy to publish it on different websites.
    // if they have this route not taken.

    // region: prepare routes

    // Turn our "state" into a new Filter...
    let state_global = warp::any().map(move || state_global.clone());

    // static files and folders:
    // /rust-reviews/css/*  - static css file
    // /rust-reviews/favicon.png  - static file

    // dynamic content:
    // /rust-reviews/
    // /rust-reviews/index.html
    // /rust-reviews/reviewer/{reviewer_id}/
    // /rust-reviews/badge/crev_count/{crate_name}.svg
    // /rust-reviews/crate/{crate_name}/
    // /rust-reviews/crate/{crate_name}/{version}/
    // /rust-reviews/crate/{crate_name}/{version}/{kind}/
    // /rust-reviews/crates/
    // /rust-reviews/last_reviews/
    // /rust-reviews/reviewers/
    // /rust-reviews/review_new/
    // /rust-reviews/review_new/{crate}/
    // /rust-reviews/review_new/{crate}/{version}/
    // /rust-reviews/people_of_rust/
    // /rust-reviews/reserved_folder/
    // /rust-reviews/reserved_folder/reindex_after_fetch_new_reviews/
    // /rust-reviews/reserved_folder/blocklisted_repos/
    // /rust-reviews/reserved_folder/list_new_reviewer_id/
    // /rust-reviews/reserved_folder/list_trusted_reviewer_id/
    // /rust-reviews/reserved_folder/daily_visitors/

    // this looks like a file and does not need ends_with_slash_or_redirect()
    let index_html_route = warp::path!("rust-reviews" / "index.html")
        .and(state_global.clone())
        .map(|state_global| {
            let ns_start = ns_start("ReviewIndexSummary");
            let data_model = review_index_summary_mod::ReviewIndexSummary::new(state_global);
            let ns_new = ns_print("new()", ns_start);
            let html_file = data_model.render_html_file("templates/");
            ns_print("render_html_file()", ns_new);
            warp::reply::html(html_file)
        });

    // the crate_name must finish with .svg
    let badge_route = warp::path!("rust-reviews" / "badge" / "crev_count" / UrlPartUtf8Decoded)
        .and(state_global.clone())
        .map(|crate_name: UrlPartUtf8Decoded, state_global| {
            let ns_start = ns_start("badge");
            let crate_name = crate_name.to_string();
            // remove suffix .svg
            let trimmed_str: &str = crate_name.trim_end_matches(".svg");
            let data_model = badge_mod::Badge::crev_count(trimmed_str, state_global);
            // dbg!(&data_model);
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
        .and(warp::body::form())
        .map(|form_data: HashMap<String, String>| {
            let ns_start = ns_start("review_new");
            let data_model = review_new_mod::ReviewNew::new(form_data);
            let ns_new = ns_print("new()", ns_start);
            let html_file = data_model.render_html_file("templates/");
            ns_print("render_html_file()", ns_new);
            warp::reply::html(html_file)
        })
        .or(
            warp::path!("rust-reviews" / "review_new" / UrlPartUtf8Decoded).map(
                |crate_name: UrlPartUtf8Decoded| {
                    let crate_name = crate_name.to_string();
                    let ns_start = ns_start("review_new");
                    let data_model = review_new_mod::ReviewNew::new_from_get(&crate_name, "");
                    let ns_new = ns_print("new()", ns_start);
                    let html_file = data_model.render_html_file("templates/");
                    ns_print("render_html_file()", ns_new);
                    warp::reply::html(html_file)
                },
            ),
        )
        .or(
            warp::path!("rust-reviews" / "review_new" / UrlPartUtf8Decoded / UrlPartUtf8Decoded)
                .map(
                    |crate_name: UrlPartUtf8Decoded, version: UrlPartUtf8Decoded| {
                        let crate_name = crate_name.to_string();
                        let version = version.to_string();
                        let ns_start = ns_start("review_new");
                        let data_model =
                            review_new_mod::ReviewNew::new_from_get(&crate_name, &version);
                        let ns_new = ns_print("new()", ns_start);
                        let html_file = data_model.render_html_file("templates/");
                        ns_print("render_html_file()", ns_new);
                        warp::reply::html(html_file)
                    },
                ),
        );

    let reserved_folder_route =
        warp::path!("rust-reviews" / "reserved_folder" / "reindex_after_fetch_new_reviews")
            .and(state_global.clone())
            .map(|state_global| {
                let ns_start = ns_start("reindex_after_fetch_new_reviews");
                let data_model =
                    reserved_folder_mod::ReservedFolder::reindex_after_fetch_new_reviews(
                        state_global,
                    );
                let ns_new = ns_print("new()", ns_start);
                let html_file = data_model.render_html_file("templates/");
                ns_print("render_html_file()", ns_new);
                warp::reply::html(html_file)
            })
            .or(
                warp::path!("rust-reviews" / "reserved_folder" / "fetch_new_reviews")
                    .and(state_global.clone())
                    .map(|state_global| {
                        let ns_start = ns_start("fetch_new_reviews");
                        let data_model =
                            reserved_folder_mod::ReservedFolder::fetch_new_reviews(state_global);
                        let ns_new = ns_print("new()", ns_start);
                        let html_file = data_model.render_html_file("templates/");
                        ns_print("render_html_file()", ns_new);
                        warp::reply::html(html_file)
                    }),
            )
            .or(
                warp::path!("rust-reviews" / "reserved_folder" / "blocklisted_repos")
                    .and(state_global.clone())
                    .and_then(|state_global| async move {
                        let ns_start = ns_start("blocklisted_repos");
                        let data_model =
                            reserved_folder_mod::ReservedFolder::blocklisted_repos(state_global);
                        let ns_new = ns_print("new()", ns_start);
                        let html_file = data_model.render_html_file("templates/");
                        ns_print("render_html_file()", ns_new);
                        // return crazy types
                        let result: Result<Box<dyn warp::Reply>, warp::Rejection> =
                            Ok(Box::new(warp::reply::html(html_file)) as Box<dyn warp::Reply>);
                        result
                    }),
            )
            .or(
                warp::path!("rust-reviews" / "reserved_folder" / "list_new_reviewer_id")
                    .and(state_global.clone())
                    .and_then(|state_global| async move {
                        let ns_start = ns_start("list_new_reviewer_id");
                        let data_model =
                            reserved_folder_mod::ReservedFolder::list_new_reviewer_id(state_global)
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
            .or(
                warp::path!("rust-reviews" / "reserved_folder" / "list_trusted_reviewer_id")
                    .and(state_global.clone())
                    .map(|state_global| {
                        let ns_start = ns_start("list_trusted_reviewer_id");
                        let data_model =
                            reserved_folder_mod::ReservedFolder::list_trusted_reviewer_id(
                                state_global,
                            );
                        let ns_new = ns_print("new()", ns_start);
                        let html_file = data_model.render_html_file("templates/");
                        ns_print("render_html_file()", ns_new);
                        warp::reply::html(html_file)
                    }),
            )
            .or(
                warp::path!("rust-reviews" / "reserved_folder" / "daily_visitors")
                    .and(state_global.clone())
                    .map(|state_global| {
                        let ns_start = ns_start("daily_visitors");
                        let data_model =
                            reserved_folder_mod::ReservedFolder::daily_visitors(state_global);
                        let ns_new = ns_print("new()", ns_start);
                        let html_file = data_model.render_html_file("templates/");
                        ns_print("render_html_file()", ns_new);
                        warp::reply::html(html_file)
                    }),
            )
            .or(warp::path!("rust-reviews" / "reserved_folder")
                .and(state_global.clone())
                .map(|state_global| {
                    let ns_start = ns_start("reserved_folder");
                    let data_model = reserved_folder_mod::ReservedFolder::new(state_global);
                    // dbg!( data_model);
                    let ns_new = ns_print("new()", ns_start);
                    let html_file = data_model.render_html_file("templates/");
                    ns_print("render_html_file()", ns_new);
                    warp::reply::html(html_file)
                }));

    let root_route = warp::path!("rust-reviews")
        .and(state_global.clone())
        .map(|state_global| {
            let ns_start = ns_start("ReviewIndexSummary");
            let data_model = review_index_summary_mod::ReviewIndexSummary::new(state_global);
            let ns_new = ns_print("new()", ns_start);
            let html_file = data_model.render_html_file("templates/");
            ns_print("render_html_file()", ns_new);
            warp::reply::html(html_file)
        })
        .or(warp::path!("rust-reviews" / "last_reviews")
            .and(state_global.clone())
            .map(|state_global| {
                let ns_start = ns_start("last_reviews");
                let data_model = last_reviews_mod::LastReviews::new(state_global);
                let ns_new = ns_print("new()", ns_start);
                let html_file = data_model.render_html_file("templates/");
                ns_print("render_html_file()", ns_new);
                warp::reply::html(html_file)
            }))
        .or(warp::path!("rust-reviews" / "crates")
            .and(state_global.clone())
            .map(|state_global| {
                let ns_start = ns_start("ReviewIndexByCrate");
                let data_model = crates_mod::ReviewIndexByCrate::new(state_global);
                let ns_new = ns_print("new()", ns_start);
                let html_file = data_model.render_html_file("templates/");
                ns_print("render_html_file()", ns_new);
                warp::reply::html(html_file)
            }))
        .or(warp::path!("rust-reviews" / "reviewers")
            .and(state_global.clone())
            .map(|state_global| {
                let ns_start = ns_start("ReviewIndexByReviewer");
                let data_model = reviewers_mod::ReviewIndexByReviewer::new(state_global);
                let ns_new = ns_print("new()", ns_start);
                let html_file = data_model.render_html_file("templates/");
                ns_print("render_html_file()", ns_new);
                warp::reply::html(html_file)
            }));

    let reviewer_route = warp::path!("rust-reviews" / "reviewer" / UrlPartUtf8Decoded)
        .and(state_global.clone())
        .map(|reviewer_id: UrlPartUtf8Decoded, state_global| {
            let reviewer_id = reviewer_id.to_string();
            let ns_start = ns_start(&format!(
                "ReviewerReviews reviewer_name: '{}'",
                Yellow.paint(&reviewer_id),
            ));
            let data_model = reviewer_reviews_mod::ReviewerReviews::new(state_global, &reviewer_id);
            let ns_new = ns_print("new()", ns_start);
            let html_file = data_model.render_html_file("templates/");
            ns_print("render_html_file()", ns_new);
            warp::reply::html(html_file)
        });

    let crate_route = warp::path!("rust-reviews" / "crate" / UrlPartUtf8Decoded)
        .and(state_global.clone())
        .map(|crate_name: UrlPartUtf8Decoded, state_global| {
            let crate_name = crate_name.to_string();
            let ns_start = ns_start(&format!(
                "CrateReviews crate_name: '{}'",
                Yellow.paint(&crate_name),
            ));

            let data_model =
                crate_reviews_mod::CrateReviews::new(state_global, &crate_name, "", "");
            let ns_new = ns_print("new()", ns_start);
            let html_file = data_model.render_html_file("templates/");
            ns_print("render_html_file()", ns_new);
            warp::reply::html(html_file)
        })
        .or(
            warp::path!("rust-reviews" / "crate" / UrlPartUtf8Decoded / UrlPartUtf8Decoded)
                .and(state_global.clone())
                .map(
                    |crate_name: UrlPartUtf8Decoded, version: UrlPartUtf8Decoded, state_global| {
                        let crate_name = crate_name.to_string();
                        let version = version.to_string();
                        let ns_start = ns_start(&format!(
                            "CrateReviews crate_name: '{}', version '{}'",
                            Yellow.paint(&crate_name),
                            Yellow.paint(&version),
                        ));

                        let data_model = crate_reviews_mod::CrateReviews::new(
                            state_global,
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
        .and(state_global.clone())
        .map(
            |crate_name: UrlPartUtf8Decoded,
             version: UrlPartUtf8Decoded,
             kind: UrlPartUtf8Decoded,
             state_global| {
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
                    state_global,
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

    let people_of_rust_route = warp::path!("rust-reviews" / "people_of_rust").map(|| {
        let ns_start = ns_start("PeopleOfRust");
        let data_model = people_of_rust_mod::PeopleOfRust::new();
        let ns_new = ns_print("new()", ns_start);
        let html_file = data_model.render_html_file("templates/");
        ns_print("render_html_file()", ns_new);
        warp::reply::html(html_file)
    });

    // static file server (starts at rust-reviews)
    // route /rust-reviews/ get files from folder ./web_content_folder/
    // static files must not have trailing slash, no need for ends_with_slash_or_redirect()
    let fileserver = warp::path("rust-reviews").and(warp::fs::dir("./web_content_folder/"));
    // endregion: prepare routes

    // combine all routes with or
    let routes = combine!(
        index_html_route,
        root_route,
        crate_route,
        reviewer_route,
        reserved_folder_route,
        review_new_route,
        badge_route,
        people_of_rust_route,
        fileserver
    );

    info!(
        "Entry point sub-directory: {} ",
        Yellow.paint(s!("/rust-reviews/"))
    );

    warp::serve(routes).run(local_addr).await;
}
