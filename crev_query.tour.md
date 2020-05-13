# crev_query
## server route  
The web server recognizes the route /query/ and calls html_for_crev_query().

##### step 1 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/main.rs#L266)
```rust
        Red.paint(local_addr.to_string())
    );
    // endregion

    // this webapp will start with the route cargo_crev_web
    // the website does not matter.
    // example: bestia.dev/cargo_crev_web/query/num-traits
    //   or : 127.0.0.1:8051/cargo_crev_web/query/num-traits

#//---------------------- selection start ----------------------
    // dynamic content
    let query_crate_name = warp::path!("cargo_crev_web" / "query" / String)
#//----------------------- selection end -----------------------
```
## data model  
Prepare CrevQueryData. This is the data model with all the data for templating in one place.

##### step 2 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/crev_query_mod.rs#L32)
```rust
    version: &str,
    kind: &str,
) -> String {
    let start = duration_mod::start_ns();
#//---------------------- selection start ----------------------
    eprintln!(
        "{}: crate_name: '{}', version '{}', kind '{}'",
        &Local::now().format("%Y-%m-%d %H:%M:%S"),
        Green.paint(crate_name),
        Green.paint(version),
        Green.paint(kind)
    );
#//----------------------- selection end -----------------------
```
## template on disk  
Read the template from the disk and start the rendering.

##### step 3 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/crev_query_mod.rs#L37)
```rust
        &Local::now().format("%Y-%m-%d %H:%M:%S"),
        Green.paint(crate_name),
        Green.paint(version),
        Green.paint(kind)
    );

    // first fill a vector with proofs, because I need to filter and sort them
    let mut proofs = proofs_crev_query(crate_name);
    let before_sum_and_filter =
#//---------------------- selection start ----------------------
        duration_mod::eprint_duration_ns("  after proofs_crev_query()", start);

#//----------------------- selection end -----------------------
```
## render_template_raw_to_nodes  
This default trait method for rendering has no special knowledge about the data. Only about html templates. The templateing works with Nodes.

##### step 4 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L77)
```rust
        &self,
        template_name: &str,
        sub_templates: &Vec<SubTemplate>,
    ) -> Vec<Node>;
    // endregion: methods to be implemented for a specific project

    // region: the only true public method - default implementation code
    // endregion: default implementation
    // region: this methods should be private somehow, but I don't know in Rust how to do it
    // / extract sub_templates and get root element Node.
#//---------------------- selection start ----------------------
    fn render_template_raw_to_nodes(
#//----------------------- selection end -----------------------
```
## must implement methods
In the same trait we have specific functions that must be implemented for every data model separately.

##### step 5 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L66)
```rust
    // plumbing between trait declaration and implementation
    // while rendering, cannot mut rrc
#//---------------------- selection start ----------------------
    fn call_fn_string(&self, placeholder: &str) -> String;
    fn call_fn_boolean(&self, placeholder: &str) -> bool;
    // this is also for sub-templates
    fn call_fn_vec_nodes(&self, placeholder: &str) -> Vec<Node>;
    fn render_sub_template(
        &self,
        template_name: &str,
        sub_templates: &Vec<SubTemplate>,
    ) -> Vec<Node>;
#//----------------------- selection end -----------------------
```
## extract children subtemplates  
The template can contain sub-templates. Here extract only the children (depth level 1).
The parent template is drained from subtemplates. Only a placeholder is retained for later replacement.

##### step 6 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L81)
```rust
    // region: this methods should be private somehow, but I don't know in Rust how to do it
    // / extract sub_templates and get root element Node.
    fn render_template_raw_to_nodes(
        &self,
        html_template_raw: &str,
        html_or_svg_parent: HtmlOrSvg,
    ) -> Result<Vec<Node>, String> {
        // html_template_raw can be a fragment. I add the root, that will later be removed.
#//---------------------- selection start ----------------------
        let html_template_raw = &format!("<template>{}</template>", html_template_raw);
        // extract sub_templates. Only one level deep.
        let sub_templates = Self::extract_children_sub_templates(html_template_raw);
#//----------------------- selection end -----------------------
```
## read template events
The reader_for_microxml moves event by event sequentialy. For different types of events there is different code. Here we transform the input String into a Vec\<Node\> for easy manipulation.

##### step 7 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L153)
```rust
        let mut replace_node: Option<Node> = None;
        let mut replace_vec_nodes: Option<Vec<Node>> = None;
        let mut replace_boolean: Option<bool> = None;
        let mut html_or_svg_local;
        // loop through all the siblings in this iteration
#//---------------------- selection start ----------------------
        loop {
            // the children inherits html_or_svg from the parent, but cannot change the parent
            html_or_svg_local = html_or_svg_parent;
            match reader_for_microxml.read_event() {
                Event::StartElement(tag_name) => {
                    dom_path.push(tag_name.to_owned());
#//----------------------- selection end -----------------------
```
## new node
A new html node/element/tag. We create a new Node with only the basic data.

##### step 8 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L160)
```rust
            html_or_svg_local = html_or_svg_parent;
            match reader_for_microxml.read_event() {
                Event::StartElement(tag_name) => {
                    dom_path.push(tag_name.to_owned());
                    // construct a child element and fill it (recursive)
#//---------------------- selection start ----------------------
                    let mut child_element = ElementNode {
                        tag_name: String::from(tag_name),
                        attributes: vec![],
                        children: vec![],
                        namespace: None,
                    };
#//----------------------- selection end -----------------------
```
## Svg namespace
Svg inside Html must be specially adorned with a namespace. Very annoying.

##### step 9 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L169)
```rust
                        namespace: None,
                    };
#//---------------------- selection start ----------------------
                    if tag_name == "svg" {
                        // this tagname changes to svg now
                        html_or_svg_local = HtmlOrSvg::Svg;
                    }
                    if let HtmlOrSvg::Svg = html_or_svg_local {
                        // this is the
                        // svg elements have this namespace
                        child_element.namespace = Some("http://www.w3.org/2000/svg".to_string());
                    }
#//----------------------- selection end -----------------------
```
## fill node recursively
The new node we created will be filled in this method. This goes recursive.

##### step 10 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L181)
```rust
                        // this tagname changes to html for children, not for this element
                        html_or_svg_local = HtmlOrSvg::Html;
                    }
                    // recursion
#//---------------------- selection start ----------------------
                    child_element = self.fill_element_node(
                        reader_for_microxml,
                        child_element,
                        html_or_svg_local,
                        dom_path,
                        sub_templates,
                    )?;
#//----------------------- selection end -----------------------
```
## fill_element_node()  
This is the recursive method. It accepts a newly created ElementNode and fills it with attributes and children. Most of the template is just copied. Special Comments and data- attributes are points in the template to replace with dynamic content.

##### step 11 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L141)
```rust
    // / Recursive function to fill the Element with attributes
    // / and sub-nodes(Element, Text, Comment).
    #[allow(clippy::too_many_lines, clippy::type_complexity)]
#//---------------------- selection start ----------------------
    fn fill_element_node(
        &self,
        reader_for_microxml: &mut ReaderForMicroXml,
        mut element: ElementNode,
        html_or_svg_parent: HtmlOrSvg,
        dom_path: &mut Vec<String>,
        sub_templates: &Vec<SubTemplate>,
    ) -> Result<ElementNode, String> {
#//----------------------- selection end -----------------------
```
## static html template
The template's life starts as static content. The graphic designer can copy the html file to his disk and open it with the browser. He can use a text editor to change html and css design. The template contains static sample data similar to the dynamic data. So the designer has the visual clue how all will look at the end.

##### step 12 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/webfolder/templates/query/crev_query_template.html#L28)
```html
            <div class="middle"><h2><a style="color:white" href="https://bestia.dev/cargo_crev_web" target="_blank">
                    cargo_crev_web</a></h2></div>
            <div class="middle right"><h3 >query reviews from <a style="color:white" href="https://github.com/crev-dev/cargo-crev" target="_blank">cargo-crev</a></h3></div>
        </div>
            <div style="display: grid;grid-template-columns: 70% 15% 15%;">
#//---------------------- selection start ----------------------
                <div><h3 class="yellow">crate: <!--t_crate_name-->num-traits</h3></div>
                <div class="right" ><a style="color:white" data-t-href="t_lib_rs_link" href="https://lib.rs/crates/num-traits" target="_blank">lib.rs</a></div>
                <div class="right"><a style="color:white" data-t-href="t_crates_io_link" href="https://crates.io/crates/num-traits" target="_blank">crates.io</a></div>
            </div>
        </div>
        <div class="crate_summary">
#//----------------------- selection end -----------------------
```
## dynamic content
Modifying the visuals of a web page is an eternal task. Let's separate as much as possible the work of the graphic designer and of the (data) developer.
Once the graphic design is ready, we need to add placeholders for dynamic data. This placeholders will be replaced with dynamic data while rendering. The placeholders must not destroy the capability of the html file to be viewed statically. I choosed to use html comments, for example \<!--t_number--\> 

##### step 13 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/webfolder/templates/query/crev_query_template.html#L42)
```html
            <div class="crate_summary_cell bold" title="reviews count">c</div>
            <div class="crate_summary_cell bold greener" title="rating strong">S
            </div>
            <div class="crate_summary_cell bold green " title="rating positive">
                P</div>
            <div class="crate_summary_cell bold" title="rating neutral">E</div>
            <div class="crate_summary_cell bold red" title="rating negative">N
            </div>
            <div class="crate_summary_cell"></div>
            <div class="crate_summary_cell yellow" title="alternatives">v</div>
#//---------------------- selection start ----------------------
            <div class="crate_summary_cell orange" title="issues">i</div>
#//----------------------- selection end -----------------------
```
## replace text  
The static text "1" for this text node is used for the graphic preview. 
To replace it with dynamic data, we add before it a comment with the special syntax \<!--t_name--\>. 

##### step 14 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/webfolder/templates/query/crev_query_template.html#L48)
```html
            <div class="crate_summary_cell"></div>
            <div class="crate_summary_cell yellow" title="alternatives">v</div>
            <div class="crate_summary_cell orange" title="issues">i</div>
            <div class="crate_summary_cell red" title="advisories">a</div>
            <div class="crate_summary_cell" title="thoroughness">t</div>
            <div class="crate_summary_cell" title="understanding">u</div>

            <div class="crate_summary_cell bold"><a data-t-href="t_filter_crate" href="">crate</a></div>
            <div class="crate_summary_cell bold" title="reviews count"><a data-t-href="t_filter_crate" href="">
                <!--t_crate_review_number-->7</a></div>
#//---------------------- selection start ----------------------
            <div class="crate_summary_cell bold greener" title="rating strong"><a data-t-href="t_filter_strong" href="">
#//----------------------- selection end -----------------------
```
## text placeholder
The rendering finds the special comment. It calls `call_fn_string` and temporarily saves the result. 
It does not push the placeholder comment to the html nodes, because is not needed in the result html.

##### step 15 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L246)
```rust
                        node_enum: NodeEnum::Text(txt),
                    });
                }
                Event::Comment(txt) => {
                    // the main goal of comments is to change the value of the next text node
                    // with the result of a function
                    // it must look like <!--t_get_text-->
#//---------------------- selection start ----------------------

                    if txt.starts_with("t_") {
                        let repl_txt = self.call_fn_string(txt);
                        replace_string = Some(repl_txt);
#//----------------------- selection end -----------------------
```
## dynamic data
Every placeholder has code that returns dynamic data as a string. This method is implemented on the data model, so it has access to all the data it needs.

##### step 16 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/all_summary_mod.rs#L146)
```rust
    // / html_templating functions that return a String
    #[allow(
        clippy::needless_return,
        clippy::integer_arithmetic,
        clippy::indexing_slicing
    )]
#//---------------------- selection start ----------------------
    fn call_fn_string(&self, placeholder: &str) -> String {
        // eprintln!("{}",&format!("call_fn_string: {}", &placeholder));
        match placeholder {
            "t_crate_name" => self.crate_name.to_string(),
            "t_crates_io_link" => format!("https://crates.io/crates/{}", self.crate_name),
#//----------------------- selection end -----------------------
```
## next TextNode
When the rendering goes to the next TextNode it does not use the static content. 
It uses the dynamic content temporarily saved.

##### step 17 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L229)
```rust
                        element.attributes.push(Attribute {
                            name: name.to_string(),
                            value: value,
                        });
                    }
                }
#//---------------------- selection start ----------------------
                Event::TextNode(txt) => {
                    let txt = if let Some(repl) = replace_string {
                        // empty the replace_string for the next node
                        replace_string = None;
                        decode_5_xml_control_characters(&repl)
#//----------------------- selection end -----------------------
```
## push to parent node
Then this dynamic TextNode is pushed to the parent node.

##### step 18 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L237)
```rust
                        // empty the replace_string for the next node
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
#//----------------------- selection end -----------------------
```
## boolean placeholder
The special comment \<!--b_...--\> can result in true or false. 
It leaves or removes the next node completely.

##### step 19 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/webfolder/templates/query/crev_query_template.html#L105)
```html
    </div>
    <!--template_all_summaries end-->
    <!--template_review_proof start-->
    <div class="review_container">
        <div class="review_header">
            <div class="review_header_cell">
                <!--t_crate_name_version-->num-traits 0.2.11</div>
            <div data-t-class="t_rating_class_color" class="review_header_cell green bold" title="rating">
                <!--t_review_rating-->positive</div>
            <div class="review_header_cell">
#//---------------------- selection start ----------------------
                <!--t_review_date-->2020-01-18</div>
#//----------------------- selection end -----------------------
```
## boolean
The rendering finds the placeholder and calls the implementation method.
It saves temporarily the result.

##### step 20 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L249)
```rust
                Event::Comment(txt) => {
                    // the main goal of comments is to change the value of the next text node
                    // with the result of a function
                    // it must look like <!--t_get_text-->

                    if txt.starts_with("t_") {
                        let repl_txt = self.call_fn_string(txt);
                        replace_string = Some(repl_txt);
#//---------------------- selection start ----------------------
                    } else if txt.starts_with("b_") {
                        // boolean if this is true than render the next node, else don't render
                        replace_boolean = Some(self.call_fn_boolean(txt));
#//----------------------- selection end -----------------------
```
## next node
Before rendering the next node we look at the temporary value replace_boolean.
If it is false, then we don't render the next node. Just jump over it.

##### step 21 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L183)
```rust
                    }
                    // recursion
                    child_element = self.fill_element_node(
                        reader_for_microxml,
                        child_element,
                        html_or_svg_local,
                        dom_path,
                        sub_templates,
                    )?;
#//---------------------- selection start ----------------------
                    // if the boolean is empty or true then render the next node
                    if replace_boolean.unwrap_or(true) {
#//----------------------- selection end -----------------------
```
## boolean method
The implemented method returns true or false for the placeholder according to the data.

##### step 22 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/proof_mod.rs#L125)
```rust
    // return
    author
}
impl HtmlTemplating for Proof {
    // / html_templating boolean id the next node is rendered or not
    fn call_fn_boolean(&self, placeholder: &str) -> bool {
        // eprintln!("{}",&format!("call_fn_boolean: {}", &placeholder));
#//---------------------- selection start ----------------------
        match placeholder {
            "b_not_for_render" => false,
            "b_has_alternatives" => self.alternatives.is_some(),
            "b_has_issues" => self.issues.is_some(),
#//----------------------- selection end -----------------------
```
