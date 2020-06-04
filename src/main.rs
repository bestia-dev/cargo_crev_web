//! main.rs
//! Here will have only doc comments, because they are so very large.
//! It is then difficult to code with all this doc comments cluttering the text editor.
//! The actual code will go to a separate file/mod : main_code_mod.rs

// region: (collapsed) lmake_readme include "readme.md" //! A

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
    // because then wasm-pack build --target no-modules returns an error: export `run` not found 
    // clippy::missing_inline_in_public_items
    // Why is this bad : Doc is good. rustc has a MISSING_DOCS allowed-by-default lint for public members, but has no way to enforce documentation of private items. This lint fixes that.
    clippy::doc_markdown,
)]
// endregion: (collapsed) Clippy

// region: (collapsed) use statements
mod author_reviews_mod;
mod authors_mod;
mod badge_mod;
mod crate_reviews_mod;
mod crate_version_summary_mod;
mod crates_mod;
mod data_file_scan_mod;
mod html_server_template_mod;
mod issue_mod;
mod macros_mod;
mod main_code_mod;
mod reserved_folder_mod;
mod review_index_mod;
mod review_index_summary_mod;
mod review_mod;
mod review_new_mod;
mod router_mod;
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
