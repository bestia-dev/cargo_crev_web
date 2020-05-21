// region: (collapsed) lmake_readme include "readme.md" //! A
//! # cargo_crev_web
//!
//! version: 2020.515.2307  date: 2020-05-15 authors: Luciano Bestia  
//! **web server to query reviews from cargo-crev**
//!
//!
//! ## cargo-crev
//!
//! Cargo-crev is a system of review for rust crates in crates.io.  
//! <https://github.com/crev-dev/cargo-crev>  
//! Originally it is a CLI that programmers use on their local machines while developing.  
//! I would like to make a public cargo-crev web app to query reviews globally.  
//! The installation of cargo-crev is complicated and involving.  
//! Having a web app will be very good for promoting the system.  
//! The basis of cargo-crev is a list of trusted individuals.  
//! For the web it would be a broader number of people to achieve more understanding in the community.  
//! The same machine will have the web server and the git repository for cargo-crev.  
//!
//! ## crates.io and lib.rs
//!
//! A similar web page is also created by @Kornelski at <https://lib.rs/crates/num-traits/crev>.  
//! lib.rs is an alternative index to crates.io.  
//! Crates.io is official rust-lang server, focused more on the trusted storage of crates. It does near to nothing for searching a crate.  
//! Lib.rs is more focused on making easier to find a crate in a category. The code is still stored on crates.io. So the trust of authenticity of the code is high.  
//!
//! ## warp
//!
//! Warp is a web server written in rust.  
//! <https://github.com/seanmonstar/warp>  
//! It will listen on port 8051 listens to http.  
//!
//! ## Google vm
//!
//! One beta working server is installed on my google vm.  
//! There is a nginx server reverse proxy that accepts https http2 on 443 and relay to internal 8051.
//! Nginx also redirects all http 80 to https 443.  
//! In sites-available/default I added this lines:
//!
//! ```nginx
//! #region cargo_crev_web
//!     #without the trailing / it is not a directory (for the server and for the browser)
//!     #do the browser redirect 301
//!     location = /cargo_crev_web {
//!       return 301 /cargo_crev_web/;
//!     }
//!     #folder name only, pass to index.html on port 8051
//!     location = /cargo_crev_web/ {
//!       proxy_pass http://127.0.0.1:8051/index.html;
//!       proxy_buffering off;
//!     }
//!     # the trailing / after both of these lines means this route is not appended to the forwarding
//!     location /cargo_crev_web/ {
//!       proxy_pass http://127.0.0.1:8051/;
//!       proxy_buffering off;
//!     }
//!   #endregion
//! ```
//!
//! The application will be in background with the command "screen" with a session_name.  
//! So I can see all the stdout of the application easily.  
//! create a new session  
//! `screen -S cargo_crev_web_8051`,  
//! connect to an existing session  
//! `screen -r cargo_crev_web_8051`,  
//! start the web server  
//! `cd /var/www/webapps/cargo_crev_web; ./cargo_crev_web`
//! detach the session
//! `ctrl+a d`
//!
//! ## install cargo-crev to fetch reviews
//!
//! On my web server I want to fetch the cargo-crev reviews from GitHub in regular intervals.  
//! I need to install cargo-crev.  
//! My first attempt was to install rust and cargo with rustup with minimal profile.
//! `curl https://sh.rustup.rs -sSf | sh -s -- --profile minimal`  
//! Then I tried to install cargo-crev with cargo:  
//! `cargo install cargo-crev`  
//! It was a disaster. I have the smallest, tiniest possible VM and it looks that compiling the source code of cargo-crev is too much for it. I tried 3 times, waited for a few hours and it didn't succeed.  
//! Fortunately there is a binary release already compiled here:  
//! `https://github.com/crev-dev/cargo-crev/releases/download/v0.16.1/cargo-crev-v0.16.1-x86_64-unknown-linux-musl.tar.gz`  
//! I unzip it twice and saved the binary file `cargo-crev` in:  
//! `~/.cargo/bin`  
//! I could use it already and fetch all the repos, but that is not super safe. Better is to fetch only the trusted repos.  
//! For this I need to create a crev Id and for that I need to have a GitHub repo.  
//! Size of .cache/crev
//! On my local machine is 7 MB
//! On web server 2 MB
//! It looks that it is not extremly big.
//!
//! ## GitHub crev-proofs
//!
//! I followed the instructions <https://github.com/crev-dev/cargo-crev/blob/master/cargo-crev/src/doc/getting_started.md>  
//! I created a new GitHub user: `cargo-crev-web`. I wanted cargo_crev_web, but I couldn't. So I have inconsistent name here.  
//! I used my second email, because my first email is used for my personal GitHub LucianoBestia.  
//! On the google vm web server I created an SSH key and copied the key to GitHub to have SSH access.  
//! I forked the template <https://github.com/crev-dev/crev-proofs>.  
//! For fetch I will open a new screen session:  
//! `screen -S cargo_crev_web_git`  
//! to reconnect later: `screen -r cargo_crev_web_git`  
//! I will need the credentials for ssh for GitHub:  
//! `eval $(ssh-agent -s)`  
//! `ssh-add ~/.ssh/bestia2_for_github`  
//! create new crev id with my new github repo:  
//! `cargo crev id new --url https://github.com/cargo-crev-web/crev-proofs`  
//! add a trusted user:  
//! `crev id trust <hash>`  
//! example for dpc - Dawid Ciężarkiewicz, the author of cargo-crev. I trust him:  
//! `cargo crev id trust FYlr8YoYGVvDwHQxqEIs89reKKDy-oWisoO0qXXEfHE`  
//! it is possible also to trust a repo:  
//! `cargo crev trust <url of someone's crev-proofs repo>`  
//! At the end of editing the local data push:  
//! `cargo crev repo publish`  
//!
//! ## Linux scheduler
//!
//! I need to call every hour:  
//! `cargo crev repo fetch trusted`
//! to have fresh reviews available locally in `~/.cache/crev/`.  
//! The Linux scheduler `crontab` is ok, but I miss something more visual.  
//! I wrote <https://github.com/LucianoBestia/foreground_scheduler> to do this.  
//! It is a normal CLI and it is easy to see the output on the screen.  
//! To make this run indefinitely in another terminal session I use `screen`.
//!
//! ## testing .cache/crev
//!
//! Not all data is required in every review, so I need to test examples that contains different data.  
//! <https://bestia.dev/cargo_crev_web/query/btoi>  alternatives  
//! <https://bestia.dev/cargo_crev_web/query/num-traits>  issues  
//! <https://bestia.dev/cargo_crev_web/query/protobuf>  advisory old  
//! <https://bestia.dev/cargo_crev_web/query/inventory>   advisories
//!
//! Locally in development is the same, just the server is 127.0.0.1:8051/.  
//!
//! ## cargo crev reviews and advisory
//!
//! It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
//! to verify the trustworthiness of each of your dependencies.  
//! Please, spread this info.  
//! On the web use this url to read crate reviews. Example:  
//! <https://bestia.dev/cargo_crev_web/query/num-traits>  
//!
//! ## html templating
//!
//! Like many developers I also suffer from "graphical designitis".  
//! It is very hard for me to find the exact color variations and shape proportions and subtle font differences to make a web page beautiful. It is not lack of knowledge of html and css. It is lack of style and taste.  
//! Simply unrepairable!  
//! So I created a simple html templating system to separate the graphical desiner work from the developer work. As much as possible.  
//! First the graphical designer prepares a nice html+css with static data, that looks awesome.  
//! The 2 files are on his local disk and don't need any server or configuration. The static data must be as realistic as possible.  
//! Then I add comments that are commands where to insert the dynamic data. This comments don't destroy the original html. That html can still be visualized statically from the disk. It is easy to add to or modify the design. Just avoid to touch the comments.  
//! On the web server the HtmlTemplating trait takes the template and inserts the dynamic data.  
//! The result is normal html and is sent to the browser.
//!
//! ## CodeTour
//!
//! I like very much the VSCode extension CodeTour.  
//! It makes a special kind of documentation that shows the code flow.  
//! No other type of documentation is so useful as this.  
//! It works only in VSCode. I made an export to md utility because is so much easier to distribute the md file around.  
//!
//! ## ideas
//!
//! The same web server can easily run on the local machine of the developer.  
//! It is just one single binary executable file.  
//! It will read only the trusted reviews specific to that developer.  
//! So now we have a local web server and a browser. It means we have now the possibility to make a beautiful GUI for cargo-crev that works on any OS and remotely also. Good.  
//! Maybe it will be possible to add the functionality to write new reviews in the browser.  
//! I am terrible in VIM, I confess.  
//!
//! ## TODO
//!
//! - cached results  - cached templates?  
//! - filtered by version, rating,... from cached  
//! - summary per author, because there will be a lot of duplicates
//!
//! ## References
//!
//! <https://github.com/rustomax/rust-iterators>  
// endregion: (collapsed) lmake_readme include "readme.md" //! A

// region: (collapsed) Clippy
#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    // variable shadowing is idiomatic to Rust, but unnatural to me.
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::shadow_unrelated,
)]
#![allow(
    // library from dependencies have this clippy warnings. Not my code.
    clippy::cargo_common_metadata,
    clippy::multiple_crate_versions,
    clippy::wildcard_dependencies,
    // Rust is more idiomatic without return statement
    clippy::implicit_return,
    // I have private function inside a function. Self does not work there.
    // clippy::use_self,
    // Cannot add #[inline] to the start function with #[wasm_bindgen(start)]
    // because then wasm-pack build --st_arget no-modules returns an error: export `run` not found 
    // clippy::missing_inline_in_public_items
    // Why is this bad : Doc is good. rustc has a MISSING_DOCS allowed-by-default lint for public members, but has no way to enforce documentation of private items. This lint fixes that.
    clippy::doc_markdown,
)]
// endregion: (collapsed) Clippy

// region: (collapsed) use statements
mod crate_version_summary_mod;
mod author_reviews_mod;
mod crate_reviews_mod;
mod data_file_scan_mod;
mod duration_mod;
mod html_server_template_mod;
mod info_group_by_author_mod;
mod info_group_by_crate_mod;
mod issue_mod;
mod review_index_mod;
mod review_index_summary_mod;
mod review_mod;
mod utils_mod;
mod version_summary_mod;

// I must put the trait in scope
use crate::html_server_template_mod::*;

use clap::App;
use env_logger::Env;
//use futures::{sync::mpsc, Future, Stream};
#[allow(unused_imports)]
use ansi_term::Colour::{Blue, Green, Red, Yellow};
use log::info;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
//use unwrap::unwrap;
use warp::Filter;
// region: (collapsed) use statements

/// main function of the binary
#[tokio::main]
async fn main() {
    // region: env_logger log text to stdout depend on ENV variable
    // in Linux : RUST_LOG=info ./cargo_crev_web.exe
    // in Windows I don't know yet.
    // default for env variable info
    let mut builder = env_logger::from_env(Env::default().default_filter_or("info"));
    // nanoseconds in the logger
    builder.format_timestamp_nanos();
    builder.init();
    // endregion

    // region: cmdline parameters
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .get_matches();

    let local_ip = IpAddr::V4("127.0.0.1".parse::<Ipv4Addr>().expect("not an ip address"));
    let local_port = u16::from_str_radix("8051", 10).expect("not a number");
    let local_addr = SocketAddr::new(local_ip, local_port);

    info!(
        "cargo_crev_web http server listening on {} ",
        Red.paint(local_addr.to_string())
    );
    // endregion

    // websites are mostly always made of more separate web-apps
    // it is good for web-apps to NOT start from the website root
    // this webapp starts with the route website_url/cargo_crev_web/
    // example: bestia.dev/cargo_crev_web/query/num-traits
    //   or : 127.0.0.1:8051/cargo_crev_web/query/num-traits
    // that way is easy to publish it on different websites.
    // if they have this route not taken.

    // region: prepare routes

    // info dynamic content info
    let info = warp::path!("cargo_crev_web" / "info")
        .map(|| {
            let data_model = review_index_summary_mod::ReviewIndexSummary::new();
            let html_file = data_model.render_html_file("templates/");
            warp::reply::html(html_file)
        })
        .or(
            warp::path!("cargo_crev_web" / "info" / "group_by_crate").map(|| {
                let data_model = info_group_by_crate_mod::ReviewIndexByCrate::new();
                let html_file = data_model.render_html_file("templates/");
                warp::reply::html(html_file)
            }),
        )
        .or(
            warp::path!("cargo_crev_web" / "info" / "group_by_author").map(|| {
                let data_model = info_group_by_author_mod::ReviewIndexByAuthor::new();
                let html_file = data_model.render_html_file("templates/");
                warp::reply::html(html_file)
            }),
        );

    // query_crate dynamic content query
    let query_crate = warp::path!("cargo_crev_web" / "query" / String)
        .map(|crate_name: String| {
            let data_model = crate_reviews_mod::CrateReviews::new(&crate_name, "", "");
            let html_file = data_model.render_html_file("templates/");
            warp::reply::html(html_file)
        })
        .or(
            warp::path!("cargo_crev_web" / "query" / String / String).map(
                |crate_name: String, version: String| {
                    let data_model = crate_reviews_mod::CrateReviews::new(&crate_name, &version, "");
                    let html_file = data_model.render_html_file("templates/");
                    warp::reply::html(html_file)
                },
            ),
        )
        .or(
            warp::path!("cargo_crev_web" / "query" / String / String / String).map(
                |crate_name: String, version: String, kind: String| {
                    let data_model =
                        crate_reviews_mod::CrateReviews::new(&crate_name, &version, &kind);
                    let html_file = data_model.render_html_file("templates/");
                    warp::reply::html(html_file)
                },
            ),
        );

    // static file server (starts at cargo_crev_web)
    // route /cargo_crev_web/ get files from folder ./web_content_folder/
    let fileserver =
        warp::path("cargo_crev_web").and(warp::fs::dir("./web_content_folder/"));
    // endregion: prepare routes

    // combine all routes
    let routes = info.or(query_crate.or(fileserver));
    warp::serve(routes).run(local_addr).await;
}
