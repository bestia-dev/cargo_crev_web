//! main.rs

// region: (collapsed) use statements
use crate::*;

use clap::App;
use env_logger::Env;
//use futures::{sync::mpsc, Future, Stream};
#[allow(unused_imports)]
use ansi_term::Colour::{Blue, Green, Red, Yellow};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Arc, Mutex};
//use unwrap::unwrap;

// end region: (collapsed) use statements

// Globally accessible object inside Arc-Mutex
pub type ArcMutStateGlobal = Arc<Mutex<state_mod::StateGlobal>>;
// review_index_mod::ReviewIndex>>;

pub async fn main_code() {
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

    let local_ip = IpAddr::V4("0.0.0.0".parse::<Ipv4Addr>().expect("not an ip address"));
    let local_port = u16::from_str_radix("8051", 10).expect("not a number");
    let local_addr = SocketAddr::new(local_ip, local_port);

    // endregion

    // I will cache the review index
    let review_index = review_index_mod::ReviewIndex::new();
    // I don't have a solution yet. I'm waiting if maybe they make an api for minimal data.
    //let crate_index = cargo_registry_index_mod::CrateIndex::new();
    let state_global = state_mod::StateGlobal {
        //crate_index,
        review_index,
    };
    let state_global = Arc::new(Mutex::new(state_global));

    router_mod::start_routes(state_global, local_addr).await;
}
