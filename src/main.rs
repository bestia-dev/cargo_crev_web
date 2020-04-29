// region: lmake_readme include "readme.md" //! A
//! # cargo_crev_web
//! 
//! version: 2020.429.2144  date: 2020-04-29 authors: Luciano Bestia  
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
//! ## warp
//! 
//! Warp is a web server written in rust.  
//! <https://github.com/seanmonstar/warp>  
//! It will listen on port 8051 listens to http.  
//! 
//! ## Google vm
//! 
//! One working server is installed on my google vm.  
//! There is a nginx server reverse proxy that accepts https http2 on 443 and relay to internal 8051.
//! Nginx also redirects all http 80 to https 443.  
//! -- to start the application in background with the command "screen" with a session_name  
//! `screen -S cargo_crev_web_8051`,  
//! `screen -r cargo_crev_web_8051`,  
//! 
//! ## testing my .cache/crev
//! 
//! in development:  
//! <http://127.0.0.1:8051/query/btoi>    alternatives  
//! <http://127.0.0.1:8051/query/num-traits>   issues  
//! <http://127.0.0.1:8051/query/protobuf>   advisory  
//! 
//! on the web:  
//! <https://bestia.dev/cargo_crev_web/query/btoi>  alternatives  
//! <https://bestia.dev/cargo_crev_web/query/num-traits>  issues  
//! <https://bestia.dev/cargo_crev_web/query/protobuf>  advisory  
// endregion: lmake_readme include "readme.md" //! A

// region: Clippy
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
    // because then wasm-pack build --target no-modules returns an error: export `run` not found 
    // clippy::missing_inline_in_public_items
    // Why is this bad : Doc is good. rustc has a MISSING_DOCS allowed-by-default lint for public members, but has no way to enforce documentation of private items. This lint fixes that.
    clippy::doc_markdown,
)]
// endregion

// region: use statements
mod crev_query_mod;

use clap::App;
use env_logger::Env;
//use futures::{sync::mpsc, Future, Stream};
use log::info;
//use serde_derive::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
//use unwrap::unwrap;
use warp::Filter;
// endregion

// region: enum, structs, const,...
// endregion

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
        "cargo-crev http server listening on {} ",
        ansi_term::Colour::Red.paint(local_addr.to_string())
    );
    // endregion

    // dynamic content
    let query = warp::path!("query" / String).map(|crate_name| {
        let html_file = crev_query_mod::crev_query(crate_name);
        warp::reply::html(html_file)
    });
    // static file server
    // GET files of route / -> are from folder ./crev/
    let fileserver = warp::fs::dir("./crev/");
    let routes = query.or(fileserver);

    warp::serve(routes).run(local_addr).await;
}
