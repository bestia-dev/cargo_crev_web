//use glob::glob;
use dirs;
use serde_derive::{Deserialize, Serialize};
use std::{fs, io, path::Path};
use unwrap::unwrap;

#[derive(Serialize, Deserialize, Clone)]
pub struct ProofFrom {
    #[serde(rename = "id-type")]
    id_type: String,
    id: String,
    url: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct ProofPackage {
    source: String,
    name: String,
    version: String,
    digest: String,
    version_for_sorting: Option<String>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct ProofReview {
    thoroughness: String,
    understanding: String,
    rating: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Issue {
    id: String,
    severity: String,
    comment: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Alternative {
    source: String,
    name: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Advisory {
    ids: Vec<String>,
    severity: String,
    range: Option<String>,
    comment: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct AdvisoryOld {
    affected: String,
    critical: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Proof {
    kind: Option<String>,
    version: String,
    date: String,
    from: ProofFrom,
    package: ProofPackage,
    review: Option<ProofReview>,
    alternatives: Option<Vec<Alternative>>,
    issues: Option<Vec<Issue>>,
    advisory: Option<AdvisoryOld>,
    advisories: Option<Vec<Advisory>>,
    comment: Option<String>,
}

/// crev query returns html
pub fn crev_query(crate_name: String) -> String {
    println!("crate_name: {}", crate_name);
    let mut html = String::with_capacity(4000);
    //first fill a vector with reviews, because I need to filter and sort them
    let mut reviews = vec![];
    // original cache crev folder: /home/luciano/.cache/crev/remotes
    // on the google vm bestia02: /home/luciano_bestia/.cache/crev/remotes
    // local webfolder example "crev/cache/crev/remotes"
    let path = unwrap!(dirs::home_dir());
    let path = path.join(".cache/crev/remotes");
    println!("path: {}", path.display());
    let mut count_files = 0;
    for filename_crev in &unwrap!(traverse_dir_with_exclude_dir(
        &path,
        "/*.crev",
        // avoid big folders and other folders with *.crev
        &vec!["/.git".to_string(), "/trust".to_string()]
    )) {
        count_files += 1;
        //println!("filename_crev: {}", filename_crev);
        // for filename_result in unwrap!(glob("/reviews/*.crev")) {
        // read crev file
        let crev_text = unwrap!(fs::read_to_string(filename_crev));
        for part1 in crev_text.split("----- END CREV PROOF -----") {
            let start_delimiter = "----- BEGIN CREV PROOF -----";
            if let Some(start_pos) = part1.find(start_delimiter) {
                let start_pos = start_pos + start_delimiter.len() + 1;
                if let Some(end_pos) = part1.find("----- SIGN CREV PROOF -----") {
                    let proof_string = &part1[start_pos..end_pos];
                    push_proof(proof_string, &mut reviews, &crate_name, filename_crev);
                }
            }
        }
        // older review has different delimiter. Everything else is the same.
        for part1 in crev_text.split("-----END CREV PACKAGE REVIEW-----") {
            let start_delimiter = "-----BEGIN CREV PACKAGE REVIEW-----";
            if let Some(start_pos) = part1.find(start_delimiter) {
                let start_pos = start_pos + start_delimiter.len() + 1;
                if let Some(end_pos) = part1.find("-----BEGIN CREV PACKAGE REVIEW SIGNATURE-----") {
                    let proof_string = &part1[start_pos..end_pos];
                    push_proof(proof_string, &mut reviews, &crate_name, filename_crev);
                }
            }
        }
    }
    println!("files queried: {}", count_files);
    //sort first by version desc, but semver version and then by date
    reviews.sort_by(|a, b| {
        b.package
            .version_for_sorting
            .cmp(&a.package.version_for_sorting)
    });
    for proof in &reviews {
        push_review_to_html(&mut html, proof);
    }

    let html_file = unwrap!(fs::read_to_string("crev/template.html"));
    let html_file = html_file.replace("<!-- content -->", &html);
    //return
    html_file
}

pub fn push_review_to_html(html: &mut String, proof: &Proof) {
    // naive method to extract author
    let author = proof
        .from
        .url
        .replace("https://github.com/", "")
        .replace("/crev-proofs", "");

    //look test.html to see the static html template
    html.push_str(r#"<div class="review_container">"#);
    html.push_str(r#"<div class="review_header">"#);
    html.push_str(&format!(
        r#"<div class="review_header_cell">{} {}</div>"#,
        proof.package.name, proof.package.version
    ));

    html.push_str(&format!(
        r#"<div class="review_header_cell green bold">{}</div>"#,
        if proof.review.is_some() {
            unwrap!(proof.review.as_ref()).rating.as_str()
        } else {
            ""
        }
    ));

    html.push_str(&format!(
        r#"<div class="review_header_cell">{}</div>"#,
        &proof.date[..10]
    ));
    html.push_str(&format!(
        r#"<div class="review_header_cell white">{}</div>"#,
        author
    ));
    html.push_str(&format!(
        r#"<div class="review_header_cell">{} {}</div>"#,
        if proof.review.is_some() {
            unwrap!(proof.review.as_ref()).thoroughness.as_str()
        } else {
            ""
        },
        if proof.review.is_some() {
            unwrap!(proof.review.as_ref()).understanding.as_str()
        } else {
            ""
        }
    ));
    html.push_str(r#"</div>"#);

    if let Some(alternatives) = &proof.alternatives {
        for alternative in alternatives {
            html.push_str(r#"<div class="review_alternative">"#);
            html.push_str(&format!(
                r#"<div class="review_header_cell">{}</div>"#,
                "alternatives:"
            ));
            html.push_str(&format!(
                r#"<div class="review_header_cell">{}</div>"#,
                &alternative.source
            ));
            html.push_str(&format!(
                r#"<div class="review_header_cell">{}</div>"#,
                alternative.name
            ));
            html.push_str(r#"</div>"#);
        }
    }

    if let Some(issues) = &proof.issues {
        for issue in issues {
            html.push_str(r#"<div class="review_issue">"#);
            html.push_str(&format!(
                r#"<div class="review_header_cell">{}</div>"#,
                "issues:"
            ));
            html.push_str(&format!(
                r#"<div class="review_header_cell">{}</div>"#,
                &issue.id
            ));
            html.push_str(&format!(
                r#"<div class="review_header_cell">{}</div>"#,
                &issue.severity
            ));
            html.push_str(&format!(
                r#"<div class="review_header_cell">{}</div>"#,
                &issue.comment
            ));
            html.push_str(r#"</div>"#);
        }
    }
    if let Some(advisories) = &proof.advisories {
        for advisory in advisories {
            html.push_str(r#"<div class="review_advisory">"#);
            html.push_str(&format!(
                r#"<div class="review_header_cell">{}</div>"#,
                "advisory:"
            ));
            let mut ids_string = String::with_capacity(300);
            for id in &advisory.ids {
                ids_string.push_str(id);
                ids_string.push_str(", ");
            }
            html.push_str(&format!(
                r#"<div class="review_header_cell">{}</div>"#,
                &ids_string
            ));
            html.push_str(&format!(
                r#"<div class="review_header_cell">{}</div>"#,
                &advisory.severity
            ));
            html.push_str(&format!(
                r#"<div class="review_header_cell">{}</div>"#,
                advisory.range.as_ref().unwrap_or(&String::new())
            ));
            html.push_str(r#"</div>"#);
        }
    }

    if let Some(advisory) = &proof.advisory {
        html.push_str(r#"<div class="review_advisory">"#);
        html.push_str(&format!(
            r#"<div class="review_header_cell">{}</div>"#,
            "advisory:"
        ));
        html.push_str(&format!(
            r#"<div class="review_header_cell">{}</div>"#,
            &advisory.affected
        ));
        html.push_str(&format!(
            r#"<div class="review_header_cell">{}</div>"#,
            &advisory.critical
        ));
        html.push_str(r#"</div>"#);
    }

    html.push_str(r#"<div class="review_comment">"#);
    if let Some(comment) = &proof.comment {
        html.push_str(comment);
    }
    html.push_str(r#"</div>"#);
    html.push_str(r#"</div>"#);
}

/// parse semver ex. 12.99.88alpha
fn parse_semver(text: &str) -> (usize, usize, usize) {
    let pos = 0;
    let (major, pos) = parse_next_number(&text, pos);
    //jump over dot
    let pos = pos + 1;
    let (minor, pos) = parse_next_number(&text, pos);
    //jump over dot
    let pos = pos + 1;
    let (patch, _pos) = parse_next_number(&text, pos);
    //return
    (major, minor, patch)
}
/// parse next characters until is numeric or end
fn parse_next_number(text: &str, pos: usize) -> (usize, usize) {
    let mut pos = pos;
    let mut number = "".to_string();
    let mut one_char = text[pos..pos + 1].chars().next().unwrap();
    while one_char.is_numeric() {
        number.push(one_char);
        pos += 1;
        if pos > text.len() - 1 {
            break;
        }
        one_char = text[pos..pos + 1].chars().next().unwrap();
    }
    let number: usize = unwrap!(number.parse());
    //return
    (number, pos)
}

/// traverse dir (sub-dir) with exclude dir
/// the find_file and the exclude dir strings must start with /
fn traverse_dir_with_exclude_dir(
    dir: &Path,
    find_file: &str,
    exclude_dirs: &Vec<String>,
) -> io::Result<Vec<String>> {
    // if the parameter is /*.rs, I can eliminate /*
    let find_file = &find_file.replace("/*", "");

    let mut v = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let str_path = unwrap!(path.to_str());
            if path.is_dir() {
                let mut is_excluded = false;
                for excl in exclude_dirs {
                    if str_path.ends_with(excl) {
                        is_excluded = true;
                        break;
                    }
                }
                if !is_excluded {
                    let mut sub_v = traverse_dir_with_exclude_dir(&path, find_file, exclude_dirs)?;
                    v.append(&mut sub_v);
                }
            } else {
                if str_path.ends_with(find_file) {
                    v.push(str_path.to_string());
                }
            }
        }
    }
    Ok(v)
}

fn push_proof(
    proof_string: &str,
    reviews: &mut Vec<Proof>,
    crate_name: &str,
    _filename_crev: &str,
) {
    let mut proof: Proof = unwrap!(serde_yaml::from_str(proof_string));
    //filter: only one crate_name
    if &proof.package.name == crate_name {
        // reviews without review are not important
        //version for sorting
        let (major, minor, patch) = parse_semver(&proof.package.version);
        proof.package.version_for_sorting = Some(format!("{:09}.{:09}.{:09}", major, minor, patch));
        reviews.push(proof);
    }
}
