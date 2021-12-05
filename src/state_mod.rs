//use crate::*;
//use crate::cargo_registry_index_mod;

pub struct StateGlobal {
    pub review_index: crate::review_index_mod::ReviewIndex,
    pub reviewer_index: crate::reviewer_index_mod::ReviewerIndex,
    // I don't have a solution yet. I'm waiting if maybe they make an api for minimal data.
    //pub crate_index : cargo_registry_index_mod::CrateIndex,
}
