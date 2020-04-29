// region: lmake_readme include "readme.md" //! A

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
