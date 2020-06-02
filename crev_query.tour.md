# crev_query
## server route  
The web server recognizes the route /query/ and calls html_for_crev_query().

##### step 1 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/main.rs#L266)
```rust
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .get_matches();

    let local_ip = IpAddr::V4("127.0.0.1".parse::<Ipv4Addr>().expect("not an ip address"));
    let local_port = u16::from_str_radix("8051", 10).expect("not a number");
    let local_addr = SocketAddr::new(local_ip, local_port);

    info!(
        "cargo_crev_web http server listening on {} ",
#//---------------------- selection start ----------------------
        Red.paint(local_addr.to_string())
    );
#//----------------------- selection end -----------------------
```
## data model  
Prepare CrevQueryData. This is the data model with all the data for templating in one place.

##### step 2 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/crev_query_mod.rs#L32)
```rust
    crate_name: &str,
    version: &str,
    kind: &str,
) -> String {
#//---------------------- selection start ----------------------
    let start = duration_mod::start_ns();
    eprintln!(
        "{}: crate_name: '{}', version '{}', kind '{}'",
        &Local::now().format("%Y-%m-%d %H:%M:%S"),
        Green.paint(crate_name),
        Green.paint(version),
        Green.paint(kind)
#//----------------------- selection end -----------------------
```
## template on disk  
Read the template from the disk and start the rendering.

##### step 3 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/crev_query_mod.rs#L37)
```rust
        "{}: crate_name: '{}', version '{}', kind '{}'",
        &Local::now().format("%Y-%m-%d %H:%M:%S"),
        Green.paint(crate_name),
        Green.paint(version),
        Green.paint(kind)
    );

    // first fill a vector with proofs, because I need to filter and sort them
    let mut proofs = proofs_crev_query(crate_name);
#//---------------------- selection start ----------------------
    let before_sum_and_filter =
        duration_mod::eprint_duration_ns("  after proofs_crev_query()", start);
#//----------------------- selection end -----------------------
```
## render_template_raw_to_nodes  
This default trait method for rendering has no special knowledge about the data. Only about html templates. The templateing works with Nodes.

##### step 4 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L77)
```rust
    fn render_sub_template(
        &self,
        template_name: &str,
        sub_templates: &Vec<SubTemplate>,
    ) -> Vec<Node>;
    // endregion: methods to be implemented for a specific project

    // region: the only 2 true public methods - default implementation code
    /// render for root template (not subtemplates) from file
    fn render_from_file(&self, template_file_name: &str) -> String {
#//---------------------- selection start ----------------------
        let mut template_raw = unwrap!(fs::read_to_string(&template_file_name));
#//----------------------- selection end -----------------------
```
## must implement methods
In the same trait we have specific functions that must be implemented for every data model separately.

##### step 5 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L66)
```rust
    // plumbing between trait declaration and implementation
    // while rendering, cannot mut rrc
#//---------------------- selection start ----------------------
    fn data_model_name(&self) -> String;
    fn call_fn_string(&self, placeholder: &str, cursor_pos: usize) -> String;
    fn call_fn_boolean(&self, placeholder: &str) -> bool;
    // this is also for sub-templates
    fn call_fn_vec_nodes(&self, placeholder: &str) -> Vec<Node>;
    fn render_sub_template(
        &self,
        template_name: &str,
        sub_templates: &Vec<SubTemplate>,
#//----------------------- selection end -----------------------
```
## extract children subtemplates  
The template can contain sub-templates. Here extract only the children (depth level 1).
The parent template is drained from subtemplates. Only a placeholder is retained for later replacement.

##### step 6 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L81)
```rust
    /// render for root template (not subtemplates) from file
    fn render_from_file(&self, template_file_name: &str) -> String {
        let mut template_raw = unwrap!(fs::read_to_string(&template_file_name));
        // find node <html >, jump over <!DOCTYPE html> because it is not microXml compatible
        // I will add <!DOCTYPE html> when the rendering ends, before returning the html.
        let pos_html = unwrap!(template_raw.find("<html"));
        template_raw.drain(..pos_html);

#//---------------------- selection start ----------------------
        self.render(&template_raw)
    }
    /// render for root template (not subtemplates) from string
#//----------------------- selection end -----------------------
```
## read template events
The reader_for_microxml moves event by event sequentialy. For different types of events there is different code. Here we transform the input String into a Vec\<Node\> for easy manipulation.

##### step 7 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L153)
```rust
                    html_or_svg_local,
                    &mut dom_path,
                    &sub_templates,
                    cursor_pos,
                ) {
#//---------------------- selection start ----------------------
                    Ok(new_root_element) => root_element = new_root_element,
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
#//----------------------- selection end -----------------------
```
## new node
A new html node/element/tag. We create a new Node with only the basic data.

##### step 8 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L160)
```rust
                        return Err(err);
                    }
                }
            }
            _ => {
#//---------------------- selection start ----------------------
                // return error
                return Err("Error: no root element".to_owned());
            }
        }
        // remove the added root <template>
        // return its children
#//----------------------- selection end -----------------------
```
## Svg namespace
Svg inside Html must be specially adorned with a namespace. Very annoying.

##### step 9 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L169)
```rust
        // remove the added root <template>
        // return its children
#//---------------------- selection start ----------------------
        Ok(root_element.children)
    }

    // Recursive function to fill the Element with attributes
    // and sub-nodes(Element, Text, Comment).
    #[allow(clippy::too_many_lines, clippy::type_complexity)]
    fn fill_element_node(
        &self,
        reader_for_microxml: &mut ReaderForMicroXml,
#//----------------------- selection end -----------------------
```
## fill node recursively
The new node we created will be filled in this method. This goes recursive.

##### step 10 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L181)
```rust
        html_or_svg_parent: HtmlOrSvg,
        dom_path: &mut Vec<String>,
        sub_templates: &Vec<SubTemplate>,
        cursor_pos: usize,
#//---------------------- selection start ----------------------
    ) -> Result<ElementNode, String> {
        let mut replace_string: Option<String> = None;
        let mut replace_node: Option<Node> = None;
        let mut replace_vec_nodes: Option<Vec<Node>> = None;
        let mut replace_boolean: Option<bool> = None;
        let mut html_or_svg_local;
        // loop through all the siblings in this iteration
#//----------------------- selection end -----------------------
```
## fill_element_node()  
This is the recursive method. It accepts a newly created ElementNode and fills it with attributes and children. Most of the template is just copied. Special Comments and data- attributes are points in the template to replace with dynamic content.

##### step 11 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L141)
```rust
                if &tag_name == &"svg" {
                    html_or_svg_local = HtmlOrSvg::Svg;
                }
#//---------------------- selection start ----------------------
                if let HtmlOrSvg::Svg = html_or_svg_local {
                    // svg elements have this namespace
                    root_element.namespace = Some(String::from("http://www.w3.org/2000/svg"));
                }
                // recursive function can return error

                match self.fill_element_node(
                    &mut reader_for_microxml,
#//----------------------- selection end -----------------------
```
## static html template
The template's life starts as static content. The graphic designer can copy the html file to his disk and open it with the browser. He can use a text editor to change html and css design. The template contains static sample data similar to the dynamic data. So the designer has the visual clue how all will look at the end.

##### step 12 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/web_server_folder/templates/query/crev_query_template.html#L28)
```html
            <div class="middle"><h2><a style="color:white" href="https://web.crev.dev/cargo_crev_web" target="_blank">
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

##### step 13 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/web_server_folder/templates/query/crev_query_template.html#L42)
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

##### step 14 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/web_server_folder/templates/query/crev_query_template.html#L48)
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
                }
                Event::Attribute(name, value) => {
                    if name.starts_with("data-t-") {
                        // placeholder is in the attribute value.
                        // the attribute name is informative and should be similar to the next attribute
                        // example: data-t-href="t_placeholder" href="x"
                        // The replace_string will always be applied to the next attribute. No matter the name.
#//---------------------- selection start ----------------------
                        let placeholder = &value;
                        let repl_txt = self.call_fn_string(placeholder, cursor_pos);
                        replace_string = Some(repl_txt);
                    } else {
#//----------------------- selection end -----------------------
```
## dynamic data
Every placeholder has code that returns dynamic data as a string. This method is implemented on the data model, so it has access to all the data it needs.

##### step 16 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/all_summary_mod.rs#L146)
```rust
    #[allow(
        clippy::needless_return,
        clippy::integer_arithmetic,
        clippy::indexing_slicing
    )]
    fn call_fn_string(&self, placeholder: &str, _cursor_pos: usize) -> String {
#//---------------------- selection start ----------------------
        // eprintln!("{}",&format!("call_fn_string: {}", &placeholder));
        match placeholder {
            "t_crate_name" => self.crate_name.to_string(),
            "t_crates_io_link" => format!("https://crates.io/crates/{}", self.crate_name),
            "t_lib_rs_link" => format!("https://lib.rs/crates/{}", self.crate_name),
#//----------------------- selection end -----------------------
```
## next TextNode
When the rendering goes to the next TextNode it does not use the static content. 
It uses the dynamic content temporarily saved.

##### step 17 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L229)
```rust
                        if let Some(repl_node) = replace_node {
                            element.children.push(repl_node);
                            replace_node = None;
                        } else if let Some(repl_vec_nodes) = replace_vec_nodes {
                            for repl_node in repl_vec_nodes {
                                element.children.push(repl_node);
#//---------------------- selection start ----------------------
                            }
                            replace_vec_nodes = None;
                        } else {
                            element.children.push(Node {
                                node_enum: NodeEnum::Element(child_element),
#//----------------------- selection end -----------------------
```
## push to parent node
Then this dynamic TextNode is pushed to the parent node.

##### step 18 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L237)
```rust
                        } else {
                            element.children.push(Node {
                                node_enum: NodeEnum::Element(child_element),
                            });
                        }
                    }
                    if replace_boolean.is_some() {
#//---------------------- selection start ----------------------
                        replace_boolean = None;
                    }
                }
                Event::Attribute(name, value) => {
#//----------------------- selection end -----------------------
```
## boolean placeholder
The special comment \<!--b_...--\> can result in true or false. 
It leaves or removes the next node completely.

##### step 19 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/web_server_folder/templates/query/crev_query_template.html#L105)
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
                        // placeholder is in the attribute value.
                        // the attribute name is informative and should be similar to the next attribute
                        // example: data-t-href="t_placeholder" href="x"
                        // The replace_string will always be applied to the next attribute. No matter the name.
                        let placeholder = &value;
                        let repl_txt = self.call_fn_string(placeholder, cursor_pos);
                        replace_string = Some(repl_txt);
                    } else {
#//---------------------- selection start ----------------------
                        let value = if let Some(repl) = replace_string {
                            // empty the replace_string for the next node
                            replace_string = None;
#//----------------------- selection end -----------------------
```
## next node
Before rendering the next node we look at the temporary value replace_boolean.
If it is false, then we don't render the next node. Just jump over it.

##### step 21 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L183)
```rust
        sub_templates: &Vec<SubTemplate>,
        cursor_pos: usize,
    ) -> Result<ElementNode, String> {
        let mut replace_string: Option<String> = None;
        let mut replace_node: Option<Node> = None;
        let mut replace_vec_nodes: Option<Vec<Node>> = None;
        let mut replace_boolean: Option<bool> = None;
        let mut html_or_svg_local;
        // loop through all the siblings in this iteration
#//---------------------- selection start ----------------------
        loop {
            // the children inherits html_or_svg from the parent, but cannot change the parent
#//----------------------- selection end -----------------------
```
## boolean method
The implemented method returns true or false for the placeholder according to the data.

##### step 22 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/proof_mod.rs#L125)
```rust
            .url
            .replace("https://github.com/", "")
            .replace("/crev-proofs", "");
        // return
        author
    }
    /// version for sorting
#//---------------------- selection start ----------------------
    pub fn version_for_sorting(&self) -> String {
        let (major, minor, patch) = parse_semver(&self.package.version);
        let version_for_sorting = format!(
            "{:09}.{:09}.{:09}-{}",
#//----------------------- selection end -----------------------
```
