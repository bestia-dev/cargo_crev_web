// cargo stores a cache of all the crates and versions on the local disk
// let's use it.

use crate::utils_mod::*;

use unwrap::unwrap;
use std::path::{Path};
use std::fs;
use serde_derive::{Deserialize};

#[derive(Clone, Debug)]
pub struct Version{
    version:String,
    version_for_sorting:String,
}

#[derive(Clone, Debug)]
pub struct Crate{
    crate_name:String,
    versions:Vec<Version>
}

#[derive(Clone, Debug)]
pub struct CrateIndex{
    crates : Vec<Crate>
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeserializeInto{
    name: String,
    vers: String,
    yanked: bool,
}

impl CrateIndex {
    /// prepares the data
    pub fn new() -> Self {
        let mut crates = vec![];
        // Find data from crates.io registry index in local cache.  
        // For now on linux only. 
        // The folder of the cache is this:  `~\.cargo\registry\index\github.com-1ecc6299db9ec823\.cache\`  
        // today 2020-06-11: index/cache: 17 MB 
        // registry/ is big 1.6 GB, index/ 82 MB, cache 242 MB, source 1,3 GB 
        let mut folder = dirs::home_dir().expect("no home dir");
        folder.push(".cargo/registry/index/github.com-1ecc6299db9ec823/.cache");
        let folder_path = Path::new(&folder);

        let files = unwrap!(traverse_dir_with_exclude_dir(
            Path::new(folder_path),
            "/*.*",
            // avoid big folders 
            &vec![]
        ));
        for file_path in files.iter(){
            let file_content = unwrap!( fs::read_to_string(file_path));
            // the start is something strange, I will ignore it until the first {
            let pos_start = file_content.find("{");
            if let Some(pos_start) = pos_start{
                let json = &file_content[pos_start..];
                let short: DeserializeInto = unwrap!(serde_yaml::from_str(json));
                if short.yanked == false{
                    println!("{} {}",short.name, short.vers );
                }
            }
        }
        //return
        CrateIndex{
            crates
        }
    }
}