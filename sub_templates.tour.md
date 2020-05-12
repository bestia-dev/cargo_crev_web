# sub_templates_children
## sub-template
A sub-template is needed when the data is from a different data model or when is needed to repeat a sub-template for a vector of data. The syntax is like \<!--template_all_summaries start--\>. It ends with \<!--template_all_summaries end--\>

##### step 1 of 5 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/webfolder/templates/query/crev_query_template.html#L14)
```html

<head>
    <meta http-equiv="Content-type" content="text/html; charset=utf-8" />
    <title>crev_query</title>
    <meta name="Description" content="web app for querying reviews from cargo-crev" />
    <link rel="stylesheet" href="/cargo_crev_web/css/cargo_crev_web.css" />
    <meta name="viewport" content="width = device-width,initial-scale = 1.0" />
</head>

<body>
#//---------------------- selection start ----------------------
    <!--template_all_summaries start-->
#//----------------------- selection end -----------------------
```
## extract sub-template
Only one level deep, only the children. Then recursively the chldren will extract their children. Save them in a vector. The [0] member is the parent template. It is drained from the sub-templates. Only a placeholder remains.

##### step 2 of 5 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L81)
```rust
    // region: this methods should be private somehow, but I don't know in Rust how to do it
    /// extract sub_templates and get root element Node.   
    fn render_template_raw_to_nodes(
        &self,
        html_template_raw: &str,
        html_or_svg_parent: HtmlOrSvg,
    ) -> Result<Vec<Node>, String> {
        // html_template_raw can be a fragment. I add the root, that will later be removed.
        let html_template_raw = &format!("<template>{}</template>", html_template_raw);
        // extract sub_templates. Only one level deep.
#//---------------------- selection start ----------------------
        let sub_templates = Self::extract_children_sub_templates(html_template_raw);
#//----------------------- selection end -----------------------
```
## template placeholder
When the template placeholder is found, the method render_sub_template is called.
The result is added to the parent node.

##### step 3 of 5 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L256)
```rust
                    if txt.starts_with("t_") {
                        let repl_txt = self.call_fn_string(txt);
                        replace_string = Some(repl_txt);
                    } else if txt.starts_with("b_") {
                        // boolean if this is true than render the next node, else don't render
                        replace_boolean = Some(self.call_fn_boolean(txt));
#//---------------------- selection start ----------------------
                    } else if txt.starts_with("template_") {
                        // replace exactly this placeholder for a sub-template
                        let template_name = txt.trim_end_matches(" start");
                        let repl_vec_nodes = self.render_sub_template(template_name, sub_templates);
                        element.children.extend_from_slice(&repl_vec_nodes);
#//----------------------- selection end -----------------------
```
## render the sub-template
Find it in the sub_templates vector. If the data is a vector, then render_template for every element. Push all nodes to parent node (extend_from_slice).

##### step 4 of 5 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/all_summary_mod.rs#L219)
```rust
#//---------------------- selection start ----------------------
            "template_summary_version" => {
                let sub_template = unwrap!(sub_templates
                    .iter()
                    .find(|&template| template.name == template_name));
                let mut nodes = vec![];
                for version_summary in &self.version_summaries {
                    let vec_node = unwrap!(version_summary
                        .render_template_raw_to_nodes(&sub_template.template, HtmlOrSvg::Html,));
                    nodes.extend_from_slice(&vec_node);
                }
                //return
                nodes
#//----------------------- selection end -----------------------
```
## render
The render method is the same for templates and sub-templates.

##### step 5 of 5 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L77)
```rust
    // endregion: methods to be implemented for a specific project

    // region: the only true public method - default implementation code
    // endregion: default implementation
    // region: this methods should be private somehow, but I don't know in Rust how to do it
    /// extract sub_templates and get root element Node.   
#//---------------------- selection start ----------------------
    fn render_template_raw_to_nodes(
        &self,
        html_template_raw: &str,
        html_or_svg_parent: HtmlOrSvg,
    ) -> Result<Vec<Node>, String> {
#//----------------------- selection end -----------------------
```
