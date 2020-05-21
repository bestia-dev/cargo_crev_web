//! author_reviews_mod

use crate::review_mod::*;
//use crate::review_index_mod::*;
use crate::data_file_scan_mod::*;
use crate::*;

//use unwrap::unwrap;
pub struct AuthorReviews {
    pub author: String,
    pub author_url: String,
    pub author_id:String,
    pub reviews: Vec<Review>,
}

impl AuthorReviews {
    pub fn new(cached_review_index: CachedReviewIndex, author_id: &str) -> Self {
        let review_index = cached_review_index
            .lock()
            .expect("error cached_review_index.lock()");
        // sort data by file_path
        // the data is sorted by path_file in ReviewIndex.new()
        // nobody else should sort the data
        // search data in the index
        let mut many_file = ManyFileReviewsPk { vec: vec![] };
        let old_file_path = "".to_string();
        let mut one_file = OneFileReviewsPk {
            file_path: "don't push the first row".to_string(),
            reviews_pk: vec![],
        };
        let mut author="".to_string();
        let mut author_url="".to_string();
        for index_item in review_index.vec.iter() {
            if index_item.author_id == author_id {
                if index_item.file_path != old_file_path {
                    if one_file.file_path != "don't push the first row" {
                        //only once read the author
                        author = index_item.author.clone();
                        author_url = index_item.author_url.clone();
                        // push the old one before creating the new one
                        many_file.vec.push(one_file);
                    }
                    // create new OneFile
                    one_file = OneFileReviewsPk {
                        file_path: index_item.file_path.clone(),
                        reviews_pk: vec![],
                    };
                }
                // add data to reviews_pk
                one_file.reviews_pk.push(ReviewPk {
                    crate_name: index_item.crate_name.clone(),
                    author_id: index_item.author_id.clone(),
                    version: index_item.version.clone(),
                });
            }
        }
        let reviews = get_vec_of_review(many_file);
        println!("reviews.len(): {}", reviews.len());
        //return
        AuthorReviews {
            author: author.to_string(),
            author_url: author_url.to_string(),
            author_id: author_id.to_string(),
            reviews,
        }
    }
}
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
