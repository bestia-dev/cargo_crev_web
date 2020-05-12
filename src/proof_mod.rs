//! proof_mod

use crate::html_template_mod::*;
use crate::issue_mod::Issue;
use serde_derive::{Deserialize, Serialize};
//use unwrap::unwrap;
use strum_macros;

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
    pub thoroughness: Level,
    pub understanding: Level,
    pub rating: Rating,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Alternative {
    pub source: String,
    pub name: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Advisory {
    pub ids: Vec<String>,
    pub severity: Level,
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

#[derive(
    strum_macros::EnumString,
    strum_macros::Display,
    Clone,
    Debug,
    Serialize,
    Deserialize,
    PartialOrd,
    Ord,
    PartialEq,
    Eq,
)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Rating {
    #[serde(alias = "dangerous")] // for backward compatibility with some previous versions
    Negative,
    Neutral,
    Positive,
    Strong,
}

#[derive(
    strum_macros::EnumString,
    strum_macros::Display,
    Debug,
    Copy,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Level {
    None,
    Low,
    Medium,
    High,
}

/// naive method to extract author
pub fn get_author(proof: &Proof) -> String {
    let author = proof
        .from
        .url
        .replace("https://github.com/", "")
        .replace("/crev-proofs", "");
    //return
    author
}
impl HtmlTemplating for Proof {
    /// html_templating boolean id the next node is rendered or not
    fn call_fn_boolean(&self, placeholder: &str) -> bool {
        // eprintln!("{}",&format!("call_fn_boolean: {}", &placeholder));
        match placeholder {
            "b_not_for_render" => false,
            "b_has_alternatives" => self.alternatives.is_some(),
            "b_has_issues" => self.issues.is_some(),
            "b_has_advisories" => self.advisories.is_some(),
            "b_has_old_advisory" => self.advisory.is_some(),
            _ => {
                eprintln!(
                    "Error: Unrecognized proof_mod call_fn_boolean: \"{}\"",
                    placeholder
                );
                true
            }
        }
    }

    /// html_templating functions that return a String
    #[allow(
        clippy::needless_return,
        clippy::integer_arithmetic,
        clippy::indexing_slicing
    )]
    fn call_fn_string(&self, placeholder: &str) -> String {
        // eprintln!("{}",&format!("call_fn_string: {}", &placeholder));
        match placeholder {
            "t_crate_name_version" => format!("{} {}", self.package.name, self.package.version),
            "t_review_rating" => {
                if let Some(review) = &self.review {
                    review.rating.to_string()
                } else {
                    "".to_string()
                }
            }
            "t_review_date" => self.date[..10].to_string(),
            "t_review_author" => {
                // naive method to extract author
                get_author(self)
            }
            "t_review_author_link" => self.from.url.to_string(),
            "t_crate_thoroughness_understanding" => {
                if let Some(review) = &self.review {
                    format!(
                        "{} {}",
                        review.thoroughness.to_string(),
                        review.understanding.to_string()
                    )
                } else {
                    "".to_string()
                }
            }
            "t_review_comment" => {
                if let Some(comment) = &self.comment {
                    comment.clone()
                } else {
                    "".to_string()
                }
            }
            "t_alternative_source" => {
                if let Some(alternatives) = &self.alternatives {
                    alternatives[0].source.to_string()
                } else {
                    "".to_string()
                }
            }
            "t_alternative_name" => {
                if let Some(alternatives) = &self.alternatives {
                    alternatives[0].name.to_string()
                } else {
                    "".to_string()
                }
            }
            "t_issue_id" => {
                if let Some(issues) = &self.issues {
                    issues[0].id.to_string()
                } else {
                    "".to_string()
                }
            }
            "t_issue_severity" => {
                if let Some(issues) = &self.issues {
                    issues[0].severity.to_string()
                } else {
                    "".to_string()
                }
            }
            "t_issue_comment" => {
                if let Some(issues) = &self.issues {
                    issues[0].comment.to_string()
                } else {
                    "".to_string()
                }
            }
            "t_advisories_ids" => {
                if let Some(advisories) = &self.advisories {
                    advisories[0].ids[0].to_string()
                } else {
                    "".to_string()
                }
            }
            "t_advisories_severity" => {
                if let Some(advisories) = &self.advisories {
                    advisories[0].severity.to_string()
                } else {
                    "".to_string()
                }
            }
            "t_advisories_range" => {
                if let Some(advisories) = &self.advisories {
                    advisories[0]
                        .range
                        .as_ref()
                        .unwrap_or(&String::new())
                        .to_string()
                } else {
                    "".to_string()
                }
            }
            "t_advisory_affected" => {
                if let Some(advisory) = &self.advisory {
                    advisory.affected.to_string()
                } else {
                    "".to_string()
                }
            }
            "t_advisory_critical" => {
                if let Some(advisory) = &self.advisory {
                    advisory.critical.to_string()
                } else {
                    "".to_string()
                }
            }
            _ => {
                let err_msg = format!(
                    "Error: Unrecognized proof_mod call_fn_string: \"{}\"",
                    placeholder
                );
                eprintln!("{}", &err_msg);
                err_msg
            }
        }
    }
    /// html_templating functions that return a vector of Nodes
    #[allow(clippy::needless_return)]
    fn call_fn_vec_nodes(&self, placeholder: &str) -> Vec<Node> {
        // eprintln!("{}",&format!("call_fn_vec_nodes: {}", &placeholder));
        match placeholder {
            _ => {
                // so much boilerplate
                let err_msg = format!(
                    "Error: Unrecognized proof_mod call_fn_vec_nodes: \"{}\"",
                    placeholder
                );
                eprintln!("{}", &err_msg);
                let node = Node {
                    node_enum: NodeEnum::Element(ElementNode {
                        tag_name: "h2".to_string(),
                        attributes: vec![],
                        children: vec![Node {
                            node_enum: NodeEnum::Text(err_msg),
                        }],
                        namespace: None,
                    }),
                };
                return vec![node];
            }
        }
    }
    /// html_templating for sub-template
    #[allow(clippy::needless_return)]
    fn render_sub_template(
        &self,
        template_name: &str,
        _sub_templates: &Vec<SubTemplate>,
    ) -> Vec<Node> {
        // eprintln!("{}",&format!("render_sub_template: {}", &placeholder));
        match template_name {
            _ => {
                // so much boilerplate
                let err_msg = format!(
                    "Error: Unrecognized proof_mod render_sub_template: \"{}\"",
                    template_name
                );
                eprintln!("{}", &err_msg);
                let node = Node {
                    node_enum: NodeEnum::Element(ElementNode {
                        tag_name: "h2".to_string(),
                        attributes: vec![],
                        children: vec![Node {
                            node_enum: NodeEnum::Text(err_msg),
                        }],
                        namespace: None,
                    }),
                };
                return vec![node];
            }
        }
    }
}
