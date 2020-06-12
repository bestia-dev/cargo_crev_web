// SADLY, it is just a cache of used crates. Most of the crates is not here.
// and it would be very big if I include all the crates, because there is a lot of data
// I don't need. I wrote to them, but I don't expect much reaction.

// cargo stores a registry index cache of all the crates and versions on the local disk
// all the versions are here, and the dependencies, It is big, too big to cache all on start.
// I will cache only the crates name, that is file names. The versions will be lazy cached.
// When needed I will check if I have the versions cached. If not, I will get them from file.
// For now on linux only.
// The folder of the cache is this:  `~\.cargo\registry\index\github.com-1ecc6299db9ec823\.cache\`
// today 2020-06-11: index/cache: 17 MB
// registry/ is big 1.6 GB, index/ 82 MB, cache 242 MB, source 1,3 GB
/*
use crate::utils_mod::*;
use crate::*;

use regex::Regex;
//use serde_derive::Deserialize;
use std::fs;
use std::path::Path;
use unwrap::unwrap;

#[derive(Clone, Debug)]
pub struct Version {
    version: String,
    version_for_sorting: String,
}

#[derive(Clone, Debug)]
pub struct Crate {
    crate_name: String,
    last_version: String,
    versions: Vec<Version>,
}

#[derive(Clone, Debug)]
pub struct CrateIndex {
    crates: Vec<Crate>,
}

impl CrateIndex {
    /// prepares the data. Only crate_names on start
    pub fn new() -> Self {
        println!("CrateIndex::new()");
        let mut crates = vec![];
        // Find data from crates.io registry index in local cache.
        let mut folder = dirs::home_dir().expect("no home dir");
        folder.push(".cargo/registry/index/github.com-1ecc6299db9ec823/.cache");
        let folder_path = Path::new(&folder);

        let files = unwrap!(traverse_dir_with_exclude_dir(
            Path::new(folder_path),
            there is no space here, correct it: "/ *",
            // avoid big folders
            &vec![]
        ));
        for file_path in files.iter() {
            //println!("file: {}", file_path);
            // only the filename
            let spl: Vec<&str> = file_path.split('/').collect();
            let crate_name = spl.last().unwrap();
            //println!("crate_name: {}", crate_name);
            crates.push(Crate {
                crate_name: crate_name.to_string(),
                last_version: s!(""),
                versions: vec![],
            })
        }
        println!("crates.len(): {}", crates.len());
        //return
        CrateIndex { crates }
    }

    /// if is empty, then read from file
    pub fn get_last_version(&mut self, crate_name: &str) -> String {
        // the crate_name must exist
        let cursor_pos = self.crates.iter().position(|x| x.crate_name == crate_name);

        if let Some(cursor_pos) =  cursor_pos{
            if self.crates[cursor_pos].last_version.is_empty() {
                // this is the first time, read it from file and store it
                // the linux shell home dir symbol ~ or HOME is not expanded in raw rust
                // I must use the dirs crate
                let mut file_path = dirs::home_dir().expect("Not home dir");
                file_path.push(".cargo/registry/index/github.com-1ecc6299db9ec823/.cache");
                //interesting rules for the file_path structure
                if crate_name.len() == 1 {
                    file_path.push("1");
                } else if crate_name.len() == 2 {
                    file_path.push("2");
                } else if crate_name.len() == 3 {
                    file_path.push("3");
                } else {
                    file_path.push(&crate_name[0..2]);
                    file_path.push(&crate_name[2..4]);
                }
                file_path.push(crate_name);
                println!("file_path: {:?}", &file_path);
                //read the content and find versions
                let file_content = unwrap!(fs::read_to_string(file_path));
                dbg!(&file_content.len());
                //I will use regex to find the last "vers": "0.3.3",
                let re = unwrap!(Regex::new(r#""vers":"(.*?)""#));
                let last_version = unwrap!(re.captures_iter(&file_content).last());
                let last_version = last_version[1].to_string();
                self.crates[cursor_pos].last_version = last_version.to_string();
            }
            return self.crates[cursor_pos].last_version.to_string();
        } else {
            return s!("");
        }
    }
}
*/
