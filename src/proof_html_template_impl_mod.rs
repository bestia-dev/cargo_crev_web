//! proof_html_template_impl_mod  

//use crate::*;

//use std::fs;
//use unwrap::unwrap;

// fn open new local page with #
// does not push to history
//pub fn open_new_local_page(hash: &str) {
// I want to put the first url in history.
// These are opened from outside my app and I cannot manage that differently.
// There are 2 of them:
// 1. if the players starts without hash
// 2. if the player scanned the qrcode and opened the p3 with group_id
// For links opened inside the app, I can call the open with or without history.
// For example for menu p21 I want to have a back button.
/*
let (_old_location_href, old_href_hash) = websysmod::get_url_and_hash();
if old_href_hash.is_empty() || old_href_hash.starts_with("#p03.") {
    websysmod::open_new_local_page_push_to_history(hash)
} else {
    let _x = websysmod::window().location().replace(hash);
}
*/
//}
