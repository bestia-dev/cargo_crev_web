// main.rs
// Here will have only doc comments, because they are so very large.
// It is then difficult to code with all this doc comments cluttering the text editor.
// The actual code will go to a separate file/mod : main_code_mod.rs

// region: lmake_md_to_doc_comments include README.md A //!
//! # Rust-reviews (working title cargo_crev_web)
//!
//! ***version: 2020.822.1757  date: 2020-08-22 reviewers: Luciano Bestia***  
//! **web server to query reviews from cargo-crev**
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-4270-green.svg)](https://github.com/LucianoBestia/cargo_crev_web/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-508-blue.svg)](https://github.com/LucianoBestia/cargo_crev_web/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-588-purple.svg)](https://github.com/LucianoBestia/cargo_crev_web/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/LucianoBestia/cargo_crev_web/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-57-orange.svg)](https://github.com/LucianoBestia/cargo_crev_web/)
//!
//! <img src="https://github.com/LucianoBestia/cargo_crev_web/blob/master/web_server_folder/web_content_folder/images/snip_01.png?raw=true" width="400"/><img src="https://github.com/LucianoBestia/cargo_crev_web/blob/master/web_server_folder/web_content_folder/images/snip_02.png?raw=true" width="400"/><img src="https://github.com/LucianoBestia/cargo_crev_web/blob/master/web_server_folder/web_content_folder/images/snip_03.png?raw=true" width="400"/><img src="https://github.com/LucianoBestia/cargo_crev_web/blob/master/web_server_folder/web_content_folder/images/snip_04.png?raw=true" width="400"/><img src="https://github.com/LucianoBestia/cargo_crev_web/blob/master/web_server_folder/web_content_folder/images/snip_05.png?raw=true" width="400"/>
//!
//! ## Try it out
//!
//! <https://web.crev.dev/rust-reviews/crate/num-traits/>  
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
//! ## CREV - Rust code reviews - Raise awareness
//!
//! Please, spread this info !\
//! Open source code needs a community effort to express trustworthiness.\
//! Start with reading the reviews of the crates you use. Example: [web.crev.dev/rust-reviews/crate/num-traits/](https://web.crev.dev/rust-reviews/crate/num-traits/) \
//! Than install the CLI [cargo-crev](https://github.com/crev-dev/cargo-crev)\. Read the [Getting Started guide](https://github.com/crev-dev/cargo-crev/blob/master/cargo-crev/src/doc/getting_started.md). \
//! On your Rust project, verify the trustworthiness of all dependencies, including transient dependencies with `cargo crev verify`\
//! Write a new review ! \
//! Describe the crates you trust. Or warn about the crate versions you think are dangerous.\
//! Help other developers, inform them and share your opinion.\
//! Use the helper on this webpage: [web.crev.dev/rust-reviews/review_new](https://web.crev.dev/rust-reviews/review_new)
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
//! #region rust-reviews
//!     #without the trailing / it is not a directory (for the server and for the browser)
//!     #do the browser redirect 301
//!     location = /rust-reviews {
//!       return 301 /rust-reviews/;
//!     }
//!     #folder name only, pass to index.html on port 8051
//!     location = /rust-reviews/ {
//!       proxy_pass http://127.0.0.1:8051/index.html;
//!       proxy_buffering off;
//!     }
//!     # the trailing / after both of these lines means this route is not appended to the forwarding
//!     location /rust-reviews/ {
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
//! It looks that it is not extremely big.
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
//! ## trusted reviewers
//!
//! For the purpose of showing most existing reviews, the cargo_rev_web will "trust" anybody.  
//! It is not really trusting, it is just showing their reviews.  
//! The repo <https://gitlab.com/crev-dev/auto-crev-proofs> contains all of the proof repos.  
//! It is automated and maintained by @chrysn.  
//! <https://github.com/crev-dev/cargo-crev/issues/336>  
//! Other reference is <https://github.com/crev-dev/cargo-crev/wiki/List-of-Proof-Repositories>  
//! For incomplete, obsolete or otherwise unwanted repos I will have an editable blacklist.  
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
//! The script is in /var/www/scripts/cargo_crev_web_fetch_reindex.sh\
//! I run it:  
//! `foreground_scheduler 05 /bin/bash "/var/www/scripts/cargo_crev_web_fetch_reindex.sh"`  
//!
//! ## testing .cache/crev
//!
//! Not all data is required in every review, so I need to test examples that contains different data.  
//! <https://web.crev.dev/rust-reviews/crate/btoi>  alternatives  
//! <https://web.crev.dev/rust-reviews/crate/num-traits/>  issues  
//! <https://web.crev.dev/rust-reviews/crate/protobuf>  advisory old  
//! <https://web.crev.dev/rust-reviews/crate/inventory>   advisories
//!
//! Locally in development is the same, just the server is 127.0.0.1:8051/.  
//!
//! ## html templating
//!
//! Like many developers I also suffer from "graphical designitis".  
//! It is very hard for me to find the exact color variations and shape proportions and subtle font differences to make a web page beautiful. It is not lack of knowledge of html and css. It is lack of style and taste.  
//! Simply unrepairable!  
//! So I created a simple html templating system to separate the graphical designer work from the developer work. As much as possible.  
//! First the graphical designer prepares a nice html+css with static data, that looks awesome.  
//! The 2 files are on his local disk and don't need any server or configuration. The static data must be as realistic as possible.  
//! Then I add comments that are commands where to insert the dynamic data. This comments don't destroy the original html. That html can still be visualized statically from the disk. It is easy to add to or modify the design. Just avoid to touch the comments.  
//! On the web server the HtmlTemplating trait takes the template and inserts the dynamic data.  
//! The result is normal html and is sent to the browser.
//!
//! ## Badges
//!
//! A service for SVG badges for `crev count of reviews for one crate` is on url:\
//! <https://web.crev.dev/rust-reviews/badge/crev_count/reader_for_microxml.svg>\
//! Example how it looks like on GitHub:\
//! <https://github.com/LucianoBestia/reader_for_microxml/>  
//! ```markdown
//! [![crev reviews](
//! https://web.crev.dev/rust-reviews/badge/crev_count/reader_for_microxml.svg
//! )](https://web.crev.dev/rust-reviews/crate/reader_for_microxml/)
//! ```
//!
//! ### xhtml validator
//!
//! Manually coded xhtml can be full of "bugs".\
//! The validator will quickly find them out.\
//! I would like to have this tool inside VsCode.\
//! <https://www.liquid-technologies.com/online-xml-validator>
//!
//! ## CodeTour
//!
//! I like very much the VSCode extension CodeTour.  
//! It makes a special kind of documentation that shows the code flow.  
//! No other type of documentation is so useful as this.  
//! It works only in VSCode. I made an export to md utility because is so much easier to distribute the md file around.  
//!
//! ## javascript Click to Copy
//!
//! I used <https://clipboardjs.com/> for Click to Copy.  
//! For tooltips I used <https://chrisbracco.com/a-simple-css-tooltip/>  
//!
//! ## colors
//!
//! Every person on this planet would like to have different colors. That is human nature. Or just nature's statistical dispersion to increase chance of survival when the environment changes dramatically...  
//! To build a website that satisfies everybody's taste for color is impossible.  
//! Even hoping to satisfy a few close friends is mission impossible.  
//! So there is this Chrome extension named User Css:  
//! <https://chrome.google.com/webstore/detail/user-css/okpjlejfhacmgjkmknjhadmkdbcldfcb>  
//! You write a css in your browser and while you are typing it immediately changes the look of the webpage.  
//! In the webpage <https://web.crev.dev/rust-reviews/> you can find a basic css for the color palette. Just Click-to-Copy and try it in User Css. Then change colors to your liking.  
//! At the end save the css for the next time you visit the website.  
//! If somebody is very happy with his/her user css, please send it to me. I will make it available to other users. They will be happy. Humans love colors - different colors.  
//!
//! ## ideas
//!
//! The same web server can easily run on the local machine of the developer.  
//! It is just one single binary executable file.  
//! It will read only the trusted reviews specific to that developer.  
//! So now we have a local web server and a browser. It means we have now the possibility to make a beautiful GUI for cargo-crev that works on any OS and remotely also. Good.  
//! This local server will ue crev-lib to access the crev functionality.  
//!
//! ## Stats
//!
//! <https://github.com/LucianoBestia/cargo_crev_web/issues/4>
//!
//! ## References
//!
//! <https://github.com/rustomax/rust-iterators>  
//!
// endregion: lmake_md_to_doc_comments include README.md A //!

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
    // because then wasm-pack build --target no-modules returns an error: export `run` not found 
    // clippy::missing_inline_in_public_items
    // Why is this bad : Doc is good. rustc has a MISSING_DOCS allowed-by-default lint for public members, but has no way to enforce documentation of private items. This lint fixes that.
    clippy::doc_markdown,
)]
// endregion: (collapsed) Clippy

// region: (collapsed) use statements
mod badge_mod;
mod cargo_registry_index_mod;
mod crate_reviews_mod;
mod crate_version_summary_mod;
mod crates_mod;
mod daily_visitors_mod;
mod data_file_scan_mod;
mod html_server_template_mod;
mod issue_mod;
mod last_reviews_mod;
mod macros_mod;
mod main_code_mod;
mod people_of_rust_mod;
mod reserved_folder_mod;
mod review_index_mod;
mod review_index_summary_mod;
mod review_mod;
mod review_new_mod;
mod reviewer_index_mod;
mod reviewer_reviews_mod;
mod reviewers_mod;
mod router_mod;
mod state_mod;
mod url_utf8_mod;
mod utils_mod;
mod version_summary_mod;

// `pub use` statements her are propagated to every other module
// when they use `crate::*;`
// just like they were all in the same file ;-)
pub use crate::html_server_template_mod::*;
pub use crate::macros_mod::*;
pub use crate::main_code_mod::*;
pub use crate::url_utf8_mod::*;
pub use crate::utils_mod::*;

// endregion: (collapsed) use statements

/// main function of the binary
#[tokio::main]
async fn main() {
    main_code_mod::main_code().await;
}
