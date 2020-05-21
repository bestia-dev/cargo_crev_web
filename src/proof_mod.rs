//! proof_mod

use crate::html_template_mod::*;
use crate::issue_mod::Issue;
use crate::utils_mod::*;
use serde_derive::{Deserialize, Serialize};
use strum_macros;
use unwrap::unwrap;

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
    None,
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

impl Proof {
    /// naive method to extract author
    pub fn get_author(&self) -> String {
        let author = self
            .from
            .url
            .replace("https://github.com/", "")
            .replace("/crev-proofs", "");
        // return
        author
    }
    /// version for sorting
    pub fn version_for_sorting(&self) -> String {
        let (major, minor, patch) = parse_semver(&self.package.version);
        let version_for_sorting = format!(
            "{:09}.{:09}.{:09}-{}",
            major,
            minor,
            patch,
            self.get_author(),
        );
        //return
        version_for_sorting
    }
    /// get rating even when review in none
    pub fn get_rating(&self) -> Rating {
        if let Some(review) = &self.review {
            review.rating.clone()
        } else {
            Rating::None
        }
    }
}

impl HtmlTemplatingRender for Proof {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        //return
        "Proof".to_string()
    }
    /// This struct is never a full html file. It is always a sub-template.
    fn render_html_file(&self, _templates_folder_name: &str) -> String {
        //return
        String::new()
    }
    // html_templating boolean id the next node is rendered or not
    fn call_fn_boolean(&self, placeholder: &str) -> bool {
        // eprintln!("{}",&format!("call_fn_boolean: {}", &placeholder));
        match placeholder {
            "b_not_for_render" => false,
            "b_has_alternatives" => self.alternatives.is_some(),
            "b_has_issues" => self.issues.is_some(),
            "b_has_advisories" => self.advisories.is_some(),
            "b_has_old_advisory" => self.advisory.is_some(),
            _ => call_fn_boolean_match_else(&self.data_model_name(), placeholder),
        }
    }

    // html_templating functions that return a String
    #[allow(
        clippy::needless_return,
        clippy::integer_arithmetic,
        clippy::indexing_slicing
    )]
    fn call_fn_string(&self, placeholder: &str, _cursor_pos: usize) -> String {
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
            "t_rating_class_color" => format!(
                "review_header_cell {} bold",
                color_from_rating(if let Some(review) = &self.review {
                    Some(&review.rating)
                } else {
                    None
                })
            ),
            "t_review_date" => self.date[..10].to_string(),
            "t_review_author" => {
                // naive method to extract author
                self.get_author()
            }
            "t_review_author_url" => self.from.url.to_string(),
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
            _ => call_fn_string_match_else(&self.data_model_name(), placeholder),
        }
    }
    // html_templating functions that return a vector of Nodes
    #[allow(clippy::needless_return)]
    fn call_fn_vec_nodes(&self, placeholder: &str) -> Vec<Node> {
        // eprintln!("{}",&format!("call_fn_vec_nodes: {}", &placeholder));
        match placeholder {
            _ => call_fn_vec_nodes_match_else(&self.data_model_name(), placeholder),
        }
    }
    // html_templating for sub-template
    #[allow(clippy::needless_return)]
    fn render_sub_template(
        &self,
        template_name: &str,
        sub_templates: &Vec<SubTemplate>,
    ) -> Vec<Node> {
        // eprintln!("{}",&format!("render_sub_template: {}", &placeholder));
        match template_name {
            "template_issues" => {
                // eprintln!("template_issues: {}", "");
                let sub_template = unwrap!(sub_templates
                    .iter()
                    .find(|&template| template.name == template_name));
                let mut nodes = vec![];
                // sub-template repeatable
                if let Some(issues) = &self.issues {
                    for issue in issues {
                        let vec_node = unwrap!(issue.render_template_raw_to_nodes(
                            &sub_template.template,
                            HtmlOrSvg::Html,
                            0
                        ));
                        nodes.extend_from_slice(&vec_node);
                    }
                }
                // return
                nodes
            }
            _ => render_sub_template_match_else(&self.data_model_name(), template_name),
        }
    }
}

pub fn color_from_rating(rating: Option<&Rating>) -> String {
    if let Some(rating) = rating {
        match rating {
            Rating::Strong => "greener".to_string(),
            Rating::Positive => "green".to_string(),
            Rating::Neutral => "".to_string(),
            Rating::Negative => "red".to_string(),
            Rating::None => "".to_string(),
        }
    } else {
        "".to_string()
    }
}
