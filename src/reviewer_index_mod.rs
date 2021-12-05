//! reviewer_index_mod.rs

use serde_derive::{Deserialize, Serialize};
use std::path::Path;
use std::path::PathBuf;

use crate::data_file_scan_mod::*;
use crate::*;

/// iterating in the original file format is not performant
/// it is better to read the files once and make an index of all
/// and then mostly use this index from memory.
/// this index is created every time the web app is initialized
/// or manually when the new and updated files are fetched
//use crate::*;
use unwrap::unwrap;

/// one item in the index
#[derive(Clone, Debug)]
pub struct ReviewerIndexItem {
    pub id: String,
    pub name: String,
    pub url: String,
}
/// only one field with a generic name vec
#[derive(Debug)]
pub struct ReviewerIndex {
    pub vec: Vec<ReviewerIndexItem>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TrustYaml {
    pub ids: Vec<IdYaml>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct IdYaml {
    id: String,
    url: Option<String>,
}

pub fn path_of_my_trust_folder() -> PathBuf {
    // original cache crev folder: /home/luciano/.config/crev/proofs
    // local webfolder example "../sample_data/.config/crev/proofs"
    let path = unwrap!(dirs::home_dir());
    let path = path.join(".config/crev/proofs");
    // dbg!(&path);
    //return
    path
}

/// all file names from trust directories
pub fn trust_crev_files(folder_path: &str) -> Vec<String> {
    // return
    unwrap!(traverse_dir_with_exclude_dir(
        Path::new(folder_path),
        "/*.crev",
        // avoid big folders and other folders with *.crev
        &vec![s!("/.git"), s!("/reviews")]
    ))
}

impl ReviewerIndex {
    /// prepares the data
    pub fn new() -> Self {
        let ns_start = ns_start("ReviewerIndex");
        let mut reviewer_index = ReviewerIndex { vec: vec![] };

        let path_of_my_trust_folder = path_of_my_trust_folder();
        let path_of_my_trust_folder = path_of_my_trust_folder.to_string_lossy() + "/";
        //fill from all the files all the reviews
        for file_name in trust_crev_files(&path_of_my_trust_folder).iter() {
            // dbg!(file_name);
            // iterator for reviews return &str
            let proofs_in_one_file = ProofsInOneFile::new(file_name);
            for proof_string in proofs_in_one_file {
                // use only data for index
                Self::push_reviewer_index(&proof_string, &mut reviewer_index);
            }
        }
        // sort by file_path
        reviewer_index
            .vec
            .sort_by(|a, b| b.name.to_lowercase().cmp(&a.name.to_lowercase()));
        ns_print("ReviewerIndex.new()", ns_start);
        // return
        reviewer_index
    }

    /// mutates review_index
    fn push_reviewer_index(review_string: &str, reviewer_index: &mut ReviewerIndex) {
        // deserialize one reviewer
        // dbg!(review_string);
        let trust_yaml: TrustYaml = unwrap!(serde_yaml::from_str(review_string));
        if trust_yaml.ids[0].url.is_none() {
            println!("url is none: {}", &trust_yaml.ids[0].id);
        } else {
            // one reviewer can have many ids
            // for example: kornelski
            for trust_id in trust_yaml.ids.iter() {
                // there can be many proofs for the same reviewer and id. They are chronologically only added in crev.
                // delete the old and push the new
                reviewer_index.vec.retain(|x| x.id != trust_id.id);
                let reviewer_index_item = ReviewerIndexItem {
                    name: reviewer_name_from_url(&trust_id.url.clone().unwrap()),
                    id: s!(&trust_id.id),
                    url: s!(&trust_id.url.clone().unwrap()),
                };
                reviewer_index.vec.push(reviewer_index_item);
            }
        }
        // dbg!("reviewer_index: {:?}", reviewer_index);
    }
}
