# crev_query
## server route  
The web server recognizes the route /query/ and calls html_for_crev_query().

##### step 1 of 23 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/main.rs#L219)
```rust
mod html_template_mod;
mod issue_mod;
mod proof_mod;
mod utils_mod;
mod version_summary_mod;

use clap::App;
use env_logger::Env;
//use futures::{sync::mpsc, Future, Stream};
#//---------------------- selection start ----------------------
use log::info;
//use serde_derive::{Deserialize, Serialize};
#//----------------------- selection end -----------------------
```
## data model  
Prepare CrevQueryData. This is the data model with all the data for templating in one place.

##### step 2 of 23 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/crev_query_mod.rs#L32)
```rust
        crate_name
    );

    //first fill a vector with proofs, because I need to filter and sort them
#//---------------------- selection start ----------------------
    let proofs = proofs_crev_query(crate_name);
    let all_summaries = all_summary_mod::calculate_all_summary_for_proofs(crate_name, &proofs);
    // put all data needed for this template in one place
    let crev_query_data = CrevQueryData {
        proofs,
        all_summaries,
    };
#//----------------------- selection end -----------------------
```
## template on disk  
Read the template from the disk and start the rendering.

##### step 3 of 23 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/crev_query_mod.rs#L37)
```rust
    // put all data needed for this template in one place
    let crev_query_data = CrevQueryData {
        proofs,
        all_summaries,
    };

    // now I have the data and I render the html from the template
    // the folders hierarchy for templates is similar like the routes
    // so to retain the same relative folders like css
#//---------------------- selection start ----------------------
    let template_file_name = format!("{}query/crev_query_template.html", templates_folder_name);
    let html_template_raw = template_raw_from_file(&template_file_name);
#//----------------------- selection end -----------------------
```
## render_template_raw_to_string  
This default trait method for rendering has no special knowledge about the data. Only about html templates. The final result is a string - html.

##### step 4 of 23 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L185)
```rust
                        // this tagname changes to html for children, not for this element
                        html_or_svg_local = HtmlOrSvg::Html;
                    }
                    //recursion
                    child_element = self.fill_element_node(
                        reader_for_microxml,
                        child_element,
                        html_or_svg_local,
                        dom_path,
                        sub_templates,
#//---------------------- selection start ----------------------
                    )?;
#//----------------------- selection end -----------------------
```
## must implement methods
In the same trait we have specific functions that must be implemented for every data model separately.

##### step 5 of 23 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L171)
```rust
                        // this is the
                        // svg elements have this namespace
#//---------------------- selection start ----------------------
                        child_element.namespace = Some("http://www.w3.org/2000/svg".to_string());
                    }
                    if tag_name == "foreignObject" {
                        // this tagname changes to html for children, not for this element
                        html_or_svg_local = HtmlOrSvg::Html;
                    }
                    //recursion
                    child_element = self.fill_element_node(
                        reader_for_microxml,
#//----------------------- selection end -----------------------
```
## render in nodes
Trait default method for render. All the rendering is processed as Nodes. Just at the end is exported to string.

##### step 6 of 23 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L238)
```rust
                        replace_string = None;
                        decode_5_xml_control_characters(&repl)
                    } else {
                        decode_5_xml_control_characters(txt)
                    };
                    // here accepts only utf-8.
#//---------------------- selection start ----------------------
                    // only minimum html entities are decoded
                    element.children.push(Node {
                        node_enum: NodeEnum::Text(txt),
                    });
                }
#//----------------------- selection end -----------------------
```
## extract children subtemplates  
The template can contain sub-templates. Here extract only the children (depth level 1).
The parent template is drained from subtemplates. Only a placeholder is retained for later replacement.

##### step 7 of 23 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L241)
```rust
                        decode_5_xml_control_characters(txt)
                    };
                    // here accepts only utf-8.
                    // only minimum html entities are decoded
                    element.children.push(Node {
                        node_enum: NodeEnum::Text(txt),
                    });
                }
#//---------------------- selection start ----------------------
                Event::Comment(txt) => {
                    // the main goal of comments is to change the value of the next text node
                    // with the result of a function
#//----------------------- selection end -----------------------
```
## read template events
The reader_for_microxml moves event by event sequentialy. For different types of events there is different code. Here we transform the input String into a Vec\<Node\> for easy manipulation.

##### step 8 of 23 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L251)
```rust
                    // with the result of a function
                    // it must look like <!--t_get_text-->

#//---------------------- selection start ----------------------
                    if txt.starts_with("t_") {
                        let repl_txt = self.call_fn_string(txt);
                        replace_string = Some(repl_txt);
                    } else if txt.starts_with("b_") {
                        // boolean if this is true than render the next node, else don't render
                        replace_boolean = Some(self.call_fn_boolean(txt));
                    } else if txt.starts_with("template_") {
                        // replace exactly this placeholder for a sub-template
#//----------------------- selection end -----------------------
```
## new node
A new html node/element/tag. We create a new Node with only the basic data.

##### step 9 of 23 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L259)
```rust
                        replace_boolean = Some(self.call_fn_boolean(txt));
                    } else if txt.starts_with("template_") {
                        // replace exactly this placeholder for a sub-template
                        let template_name = txt.trim_end_matches(" start");
                        let repl_vec_nodes = self.render_sub_template(template_name, sub_templates);
#//---------------------- selection start ----------------------
                        for repl_node in repl_vec_nodes {
                            element.children.push(repl_node.clone());
                        }
                    } else if txt.starts_with("n_") {
                        // nodes  (in a vector)
                        let repl_vec_nodes = self.call_fn_vec_nodes(txt);
#//----------------------- selection end -----------------------
```
## Svg namespace
Svg inside Html must be specially adorned with a namespace. Very annoying.

##### step 10 of 23 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L266)
```rust
                        }
                    } else if txt.starts_with("n_") {
                        // nodes  (in a vector)
                        let repl_vec_nodes = self.call_fn_vec_nodes(txt);
#//---------------------- selection start ----------------------
                        replace_vec_nodes = Some(repl_vec_nodes);
                    } else {
                        // it is really a comment
                        element.children.push(Node {
                            node_enum: NodeEnum::Comment(txt.to_string()),
                        });
                    }
#//----------------------- selection end -----------------------
```
## fill node recursively
The new node we created will be filled in this method. This goes recursive.

##### step 11 of 23 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L275)
```rust
                        });
                    }
                }
                Event::EndElement(name) => {
#//---------------------- selection start ----------------------
                    let last_name = unwrap!(dom_path.pop());
                    // it can be also auto-closing element
                    if last_name == name || name == "" {
                        return Ok(element);
                    } else {
                        return Err(format!(
                            "End element not correct: starts <{}> ends </{}>",
#//----------------------- selection end -----------------------
```
## fill_element_node()  
This is the recursive method. It accepts a newly created ElementNode and fills it with attributes and children. Most of the template is just copied. Special Comments and data- attributes are points in the template to replace with dynamic content.

##### step 12 of 23 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L302)
```rust
        /// https://www.tutorialspoint.com/html5/html5_entities.htm  
        fn decode_5_xml_control_characters(input: &str) -> String {
            // The standard library replace() function makes allocation,
#//---------------------- selection start ----------------------
            //but is probably fast enough for my use case.
            input
                .replace("&quot;", "\"")
                .replace("&apos;", "'")
                .replace("&amp;", "&")
                .replace("&lt;", "<")
                .replace("&gt;", ">")
        }
#//----------------------- selection end -----------------------
```
## static html template
The template's life starts as static content. The graphic designer can copy the html file to his disk and open it with the browser. He can use a text editor to change html and css design. The template contains static sample data similar to the dynamic data. So the designer has the visual clue how all will look at the end.

##### step 13 of 23 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/webfolder/templates/query/crev_query_template.html#L28)
```html
            <h3>query reviews from <a href="https://github.com/crev-dev/cargo-crev" target="_blank">cargo-crev</a></h3>
            <h3>crate: <a data-t-href="t_crate_link" href="https://crates.io/crates/num-traits" target="_blank"><!--t_crate_name-->num-traits</a></h3>
        </div>
        <div class="crate_summary">
            <div class="crate_summary_cell bold"></div>
#//---------------------- selection start ----------------------
            <div class="crate_summary_cell bold" title="reviews count">c</div>
            <div class="crate_summary_cell bold greener" title="rating strong">S
            </div>
            <div class="crate_summary_cell bold green " title="rating positive">
                P</div>
            <div class="crate_summary_cell bold" title="rating neutral">E</div>
#//----------------------- selection end -----------------------
```
## dynamic content
Modifying the visuals of a web page is an eternal task. Let's separate as much as possible the work of the graphic designer and of the (data) developer.
Once the graphic design is ready, we need to add placeholders for dynamic data. This placeholders will be replaced with dynamic data while rendering. The placeholders must not destroy the capability of the html file to be viewed statically. I choosed to use html comments, for example \<!--t_number--\> 

##### step 14 of 23 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/webfolder/templates/query/crev_query_template.html#L42)
```html
            <div class="crate_summary_cell yellow" title="alternatives">a</div>
            <div class="crate_summary_cell orange" title="issues">i</div>
            <div class="crate_summary_cell red" title="advisories">a</div>
            <div class="crate_summary_cell" title="thoroughness">t</div>
            <div class="crate_summary_cell" title="understanding">u</div>

            <div class="crate_summary_cell bold">crate</div>
            <div class="crate_summary_cell bold" title="reviews count">
                <!--t_crate_review_number-->7</div>
            <div class="crate_summary_cell bold greener" title="rating strong">
                <!--t_crate_rating_strong-->2</div>
```
## replace text  
The static text "1" for this text node is used for the graphic preview. 
To replace it with dynamic data, we add before it a comment with the special syntax \<!--t_name--\>. 

##### step 15 of 23 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/webfolder/templates/query/crev_query_template.html#L48)
```html
            <div class="crate_summary_cell bold">crate</div>
            <div class="crate_summary_cell bold" title="reviews count">
                <!--t_crate_review_number-->7</div>
            <div class="crate_summary_cell bold greener" title="rating strong">
                <!--t_crate_rating_strong-->2</div>
            <div class="crate_summary_cell bold green " title="rating positive">
                <!--t_crate_rating_positive-->3</div>
            <div class="crate_summary_cell bold" title="rating neutral">
                <!--t_crate_rating_neutral-->2</div>
            <div class="crate_summary_cell bold red" title="rating negative">
#//---------------------- selection start ----------------------
                <!--t_crate_rating_negative-->1</div>
#//----------------------- selection end -----------------------
```
## text placeholder
The rendering finds the special comment. It calls `call_fn_string` and temporarily saves the result. 
It does not push the placeholder comment to the html nodes, because is not needed in the result html.

##### step 16 of 23 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L408)
```rust
    fn element_node_to_html(html: &mut String, element_node: &ElementNode) {
        html.push_str("<");
        html.push_str(&element_node.tag_name);
        html.push_str(" ");
        for attr in &element_node.attributes {
            html.push_str(&attr.name);
            html.push_str(" = \"");
            html.push_str(&attr.value);
#//---------------------- selection start ----------------------
            html.push_str("\" ");
        }
        html.push_str(">");
#//----------------------- selection end -----------------------
```
## dynamic data
Every placeholder has code that returns dynamic data as a string. This method is implemented on the data model, so it has access to all the data it needs.

##### step 17 of 23 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/all_summary_mod.rs#L146)
```rust
        clippy::needless_return,
        clippy::integer_arithmetic,
        clippy::indexing_slicing
    )]
    fn call_fn_string(&self, placeholder: &str) -> String {
        // eprintln!("{}",&format!("call_fn_string: {}", &placeholder));
        match placeholder {
            "t_crate_name" => self.crate_name.to_string(),
            "t_crate_link" => format!("https://crates.io/crates/{}", self.crate_name),
            "t_crate_review_number" => to_string_zero_to_empty(self.crate_summary.review_number),
            "t_crate_rating_strong" => to_string_zero_to_empty(self.crate_summary.rating_strong),
```
## next TextNode
When the rendering goes to the next TextNode it does not use the static content. 
It uses the dynamic content temporarily saved.

##### step 18 of 23 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L391)
```rust
}
// region: utility fn only for the root template
/// only the root template comes from a file
/// all sub-templates later are raw strings
pub fn template_raw_from_file(file_name: &str) -> String {
    let mut template_raw = unwrap!(fs::read_to_string(file_name));
    // find node <html >, jump over <!DOCTYPE html> because it is not microXml compatible
#//---------------------- selection start ----------------------
    // I will add <!DOCTYPE html> when the rendering ends.
    let pos_html = unwrap!(template_raw.find("<html"));
    template_raw.drain(..pos_html);
    //return
#//----------------------- selection end -----------------------
```
## push to parent node
Then this dynamic TextNode is pushed to the parent node.

##### step 19 of 23 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L398)
```rust
    let pos_html = unwrap!(template_raw.find("<html"));
    template_raw.drain(..pos_html);
    //return
    template_raw
}
/// default implementation - render template to string
pub fn element_node_to_string(element_node: &ElementNode) -> Result<String, String> {
    // region: private fn
#//---------------------- selection start ----------------------
    /// sub element to html
    fn element_node_to_html(html: &mut String, element_node: &ElementNode) {
        html.push_str("<");
#//----------------------- selection end -----------------------
```
## boolean placeholder
The special comment \<!--b_...--\> can result in true or false. 
It leaves or removes the next node completely.

##### step 20 of 23 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/webfolder/templates/query/crev_query_template.html#L105)
```html
            <div class="review_header_cell green bold" title="rating">
                <!--t_review_rating-->positive</div>
            <div class="review_header_cell">
                <!--t_review_date-->2020-01-18</div>
            <div class="review_header_cell white">
                <a target="_blank" data-t-href="t_review_author_link" href="https://github.com/niklasf/crev-proofs">
                    <!--t_review_author-->author_name</a></div>
            <div class="review_header_cell" title="thoroughness understanding">
                <!--t_crate_thoroughness_understanding-->none high</div>
        </div>
#//---------------------- selection start ----------------------
        <!--b_has_alternatives-->
#//----------------------- selection end -----------------------
```
## boolean
The rendering finds the placeholder and calls the implementation method.
It saves temporarily the result.

##### step 21 of 23 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L422)
```rust
                    //recursion
                    element_node_to_html(html, sub_element);
                }
                NodeEnum::Text(text) => html.push_str(&text),
                NodeEnum::Comment(text) => html.push_str(&format!("<!--{}-->", &text)),
            }
        }
        //end tag
#//---------------------- selection start ----------------------
        html.push_str("</");
        html.push_str(&element_node.tag_name);
        html.push_str(">");
#//----------------------- selection end -----------------------
```
## next node
Before rendering the next node we look at the temporary value replace_boolean.
If it is false, then we don't render the next node. Just jump over it.

##### step 22 of 23 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L346)
```rust
                    {
                        exist_template = true;
                        // special name for template that will not be used at all.
                        // this happens when the graphic designer need more repetition of the
                        // same sub-template only for visual effect while editing.
                        if sub_template_name == "template_not_for_render" {
                            // eprintln!("template_not_for_render {} {}",pos_start, pos_end_after_tag);
                            //remove all the template
                            sub_templates[0]
                                .template
#//---------------------- selection start ----------------------
                                .drain(pos_start..pos_end_after_tag);
#//----------------------- selection end -----------------------
```
## boolean method
The implemented method returns true or false for the placeholder according to the data.

##### step 23 of 23 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/proof_mod.rs#L125)
```rust
    //return
    author
}
impl HtmlTemplating for Proof {
    /// html_templating boolean id the next node is rendered or not
    fn call_fn_boolean(&self, placeholder: &str) -> bool {
        // eprintln!("{}",&format!("call_fn_boolean: {}", &placeholder));
#//---------------------- selection start ----------------------
        match placeholder {
            "b_not_for_render" => false,
            "b_has_alternatives" => self.alternatives.is_some(),
            "b_has_issues" => self.issues.is_some(),
#//----------------------- selection end -----------------------
```
