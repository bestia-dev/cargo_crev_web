//! review_mod

use crate::html_server_template_mod::*;
use crate::issue_mod::Issue;
use crate::utils_mod::*;
use crate::*;

use serde_derive::{Deserialize, Serialize};
use strum_macros;
use unwrap::unwrap;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ReviewFrom {
    #[serde(rename = "id-type")]
    pub id_type: String,
    pub id: String,
    pub url: String,
}
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ReviewPackage {
    pub source: String,
    pub name: String,
    pub version: String,
    pub digest: String,
    pub version_for_sorting: Option<String>,
}
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ReviewReview {
    pub thoroughness: Level,
    pub understanding: Level,
    pub rating: Rating,
}
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Alternative {
    pub source: String,
    pub name: String,
}
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Advisory {
    pub ids: Vec<String>,
    pub severity: Level,
    pub range: Option<String>,
    pub comment: String,
}
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct AdvisoryOld {
    pub affected: String,
    pub critical: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Review {
    pub kind: Option<String>,
    pub version: String,
    pub date: String,
    pub from: ReviewFrom,
    pub package: ReviewPackage,
    pub review: Option<ReviewReview>,
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

impl Default for Rating {
    fn default() -> Self {
        Rating::None
    }
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
impl Default for Level {
    fn default() -> Self {
        Level::None
    }
}

impl Review {
    /// naive method to extract author_name
    pub fn get_author_name(&self) -> String {
        let author_name = author_name_from_url(&self.from.url);

        // return
        author_name
    }
    /// version for sorting
    pub fn version_for_sorting(&self) -> String {
        let (major, minor, patch) = parse_semver(&self.package.version);
        let version_for_sorting = format!(
            "{:09}.{:09}.{:09}-{}",
            major,
            minor,
            patch,
            self.get_author_name(),
        );
        // return
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

impl HtmlServerTemplateRender for Review {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        // return
        s!("Review")
    }
    /// renders the complete html file. Not a sub-template/fragment.
    fn render_html_file(&self, _templates_folder_name: &str) -> String {
        // return
        String::new()
    }
    /// boolean : is the next node rendered or not
    fn retain_next_node_or_attribute(&self, placeholder: &str) -> bool {
        // dbg!(&placeholder);
        match placeholder {
            "sb_not_for_render" => false,
            "sb_has_alternatives" => self.alternatives.is_some(),
            "sb_has_issues" => self.issues.is_some(),
            "sb_has_advisories" => self.advisories.is_some(),
            "sb_has_old_advisory" => self.advisory.is_some(),
            _ => retain_next_node_or_attribute_match_else(&self.data_model_name(), placeholder),
        }
    }

    /// returns a String to replace the next text-node
    #[allow(
        clippy::needless_return,
        clippy::integer_arithmetic,
        clippy::indexing_slicing
    )]
    fn replace_with_string(
        &self,
        placeholder: &str,
        _subtemplate: &str,
        _pos_cursor: usize,
    ) -> String {
        // dbg!( &placeholder);
        match placeholder {
            "st_crate_name_version" => format!("{} {}", self.package.name, self.package.version),
            "st_crate_route" => {
                format!("/cargo_crev_web/crate/{}/", url_encode(&self.package.name))
            }
            "st_review_rating" => {
                if let Some(review) = &self.review {
                    review.rating.to_string()
                } else {
                    s!("")
                }
            }
            "st_rating_class_color" => format!(
                "review_header0_cell {} bold",
                color_from_rating(if let Some(review) = &self.review {
                    Some(&review.rating)
                } else {
                    None
                })
            ),
            "st_review_date" => s!(&self.date[..10]),
            "st_review_author" => {
                // naive method to extract author_name
                self.get_author_name()
            }
            "st_author_url" => s!(&self.from.url),
            "st_author_route" => format!("/cargo_crev_web/author/{}/", url_encode(&self.from.id)),
            "st_crate_thoroughness_understanding" => {
                if let Some(review) = &self.review {
                    format!(
                        "{} {}",
                        review.thoroughness.to_string(),
                        review.understanding.to_string()
                    )
                } else {
                    s!("")
                }
            }
            "st_review_comment" => {
                // dbg!(&self.comment);
                if let Some(comment) = &self.comment {
                    comment.clone()
                } else {
                    s!("")
                }
            }
            "st_alternatives_source" => {
                if let Some(alternatives) = &self.alternatives {
                    s!(&alternatives[0].source)
                } else {
                    s!("")
                }
            }
            "st_alternative_name" => {
                if let Some(alternatives) = &self.alternatives {
                    s!(&alternatives[0].name)
                } else {
                    s!("")
                }
            }
            "st_issue_id" => {
                if let Some(issues) = &self.issues {
                    s!(&issues[0].id)
                } else {
                    s!("")
                }
            }
            "st_issue_severity" => {
                if let Some(issues) = &self.issues {
                    issues[0].severity.to_string()
                } else {
                    s!("")
                }
            }
            "st_issue_comment" => {
                if let Some(issues) = &self.issues {
                    s!(&issues[0].comment)
                } else {
                    s!("")
                }
            }
            "st_advisories_ids" => {
                if let Some(advisories) = &self.advisories {
                    s!(&advisories[0].ids[0])
                } else {
                    s!("")
                }
            }
            "st_advisories_severity" => {
                if let Some(advisories) = &self.advisories {
                    advisories[0].severity.to_string()
                } else {
                    s!("")
                }
            }
            "st_advisories_range" => {
                if let Some(advisories) = &self.advisories {
                    if let Some(range) = &advisories[0].range {
                        s!(range)
                    } else {
                        s!("")
                    }
                } else {
                    s!("")
                }
            }
            "st_advisory_affected" => {
                if let Some(advisory) = &self.advisory {
                    s!(&advisory.affected)
                } else {
                    s!("")
                }
            }
            "st_advisory_critical" => {
                if let Some(advisory) = &self.advisory {
                    s!(&advisory.critical)
                } else {
                    s!("")
                }
            }
            _ => replace_with_string_match_else(&self.data_model_name(), placeholder),
        }
    }
    /// returns a vector of Nodes to replace the next Node
    #[allow(clippy::needless_return)]
    fn replace_with_nodes(&self, placeholder: &str) -> Vec<Node> {
        // dbg!( &placeholder);
        match placeholder {
            _ => replace_with_nodes_match_else(&self.data_model_name(), placeholder),
        }
    }
    /// renders sub-template
    #[allow(clippy::needless_return)]
    fn render_sub_template(
        &self,
        template_name: &str,
        sub_templates: &Vec<SubTemplate>,
    ) -> Vec<Node> {
        // dbf!( &placeholder);
        match template_name {
            "stmplt_issues" => {
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
                            template_name,
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
            Rating::Strong => s!("greener"),
            Rating::Positive => s!("green"),
            Rating::Neutral => s!(""),
            Rating::Negative => s!("red"),
            Rating::None => s!(""),
        }
    } else {
        s!("")
    }
}
