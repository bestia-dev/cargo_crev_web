//use crate::*;
use crate::review_index_mod;
//use crate::cargo_registry_index_mod;

pub struct StateGlobal {
    pub review_index: review_index_mod::ReviewIndex,
    // I don't have a solution yet. I'm waiting if maybe they make an api for minimal data.
    //pub crate_index : cargo_registry_index_mod::CrateIndex,
}
