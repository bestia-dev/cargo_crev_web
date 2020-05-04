//use glob::glob;

use serde_derive::{Deserialize, Serialize};
//use unwrap::unwrap;

#[derive(Serialize, Deserialize, Clone)]
pub struct ProofFrom {
    #[serde(rename = "id-type")]
    pub id_type: String,
    pub id: String,
    pub url: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct ProofPackage {
    pub source: String,
    pub name: String,
    pub version: String,
    pub digest: String,
    pub version_for_sorting: Option<String>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct ProofReview {
    pub thoroughness: String,
    pub understanding: String,
    pub rating: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Issue {
    pub id: String,
    pub severity: String,
    pub comment: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Alternative {
    pub source: String,
    pub name: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Advisory {
    pub ids: Vec<String>,
    pub severity: String,
    pub range: Option<String>,
    pub comment: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct AdvisoryOld {
    pub affected: String,
    pub critical: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Proof {
    pub kind: Option<String>,
    pub version: String,
    pub date: String,
    pub from: ProofFrom,
    pub package: ProofPackage,
    pub review: Option<ProofReview>,
    pub alternatives: Option<Vec<Alternative>>,
    pub issues: Option<Vec<Issue>>,
    pub advisory: Option<AdvisoryOld>,
    pub advisories: Option<Vec<Advisory>>,
    pub comment: Option<String>,
}
