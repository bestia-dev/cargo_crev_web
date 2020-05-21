//! author_reviews_mod

//use crate::review_mod::*;
//use crate::review_index_mod::*;
//use crate::data_file_scan_mod::*;

/*
pub fn prepare_reviews_for_author() -> Vec<Review> {
    //fill ReviewIndex
    let review_index = ReviewIndex::new();

// pub fn new() -> ReviewIndex {

    //get_review_pk


    // file scan for reviews
    //fn get_vec_of_review_by_review_pk(path_name: &str, review_pks: Vec<ReviewPk>) -> Vec<Review> {

}

// from the index returns path_name and ReviewPk
pub fn get_review_pk_for_author(review_index: &ReviewIndex, author_url: &str) -> Vec<OneFileReviewsPk> {
    // if they are sorted by author_url, than is easy
    let vec_one_file_reviews_pk = vec![];
    let old_file_path = "";
    let ret = vec![];
    let myp;
    for x in review_index {
        if x.author_url == author_url {
            if x.file_path != old_file_path {
                //create new item and empty vector
                myp = vec_one_file_reviews_pk {
                    file_path: x.file_path,
                    reviews_pk: vec![],
                };
            }
            // push to vector
            myp.reviews_pk.push(ReviewPk {
                crate_name: x.crate_name,
                author_url: x.author_url,
                version: x.version,
            });
        }
    }
}
*/
