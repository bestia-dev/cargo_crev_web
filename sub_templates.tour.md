# sub_templates_children
## sub-template
A sub-template is needed when the data is from a different data model or when is needed to repeat a sub-template for a vector of data. The syntax is like \<!--template_all_summaries start--\>. It ends with \<!--template_all_summaries end--\>

##### step 1 of 5 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/web_server_folder/templates/query/crev_query_template.html#L14)
```html

<head>
    <meta http-equiv="Content-type" content="text/html; charset=utf-8" />
    <title>crev_query</title>
    <meta name="Description" content="web app for querying reviews from cargo-crev" />
    <link rel="stylesheet" data-t-href="t_css_href"  href="../css/cargo_crev_web.css" />
    <meta name="viewport" content="width = device-width,initial-scale = 1.0" />
    <link rel="shortcut icon" type="image/x-icon" data-t-href="t_favicon_href" href="../favicon.png" />
</head>

#//---------------------- selection start ----------------------
<body>
#//----------------------- selection end -----------------------
```
## extract sub-template
Only one level deep, only the children. Then recursively the chldren will extract their children. Save them in a vector. The [0] member is the parent template. It is drained from the sub-templates. Only a placeholder remains.

##### step 2 of 5 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L81)
```rust
    /// render for root template (not subtemplates) from file
    fn render_from_file(&self, template_file_name: &str) -> String {
        let mut template_raw = unwrap!(fs::read_to_string(&template_file_name));
        // find node <html >, jump over <!DOCTYPE html> because it is not microXml compatible
        // I will add <!DOCTYPE html> when the rendering ends, before returning the html.
        let pos_html = unwrap!(template_raw.find("<html"));
        template_raw.drain(..pos_html);

        self.render(&template_raw)
    }
#//---------------------- selection start ----------------------
    /// render for root template (not subtemplates) from string
#//----------------------- selection end -----------------------
```
## template placeholder
When the template placeholder is found, the method render_sub_template is called.
The result is added to the parent node.

##### step 3 of 5 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L256)
```rust
                        let repl_txt = self.call_fn_string(placeholder, cursor_pos);
                        replace_string = Some(repl_txt);
                    } else {
                        let value = if let Some(repl) = replace_string {
                            // empty the replace_string for the next node
                            replace_string = None;
#//---------------------- selection start ----------------------
                            decode_5_xml_control_characters(&repl)
                        } else {
                            decode_5_xml_control_characters(value)
                        };
                        element.attributes.push(Attribute {
#//----------------------- selection end -----------------------
```
## render the sub-template
Find it in the sub_templates vector. If the data is a vector, then render_template for every element. Push all nodes to parent node (extend_from_slice).

##### step 4 of 5 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/all_summary_mod.rs#L219)
```rust
#//---------------------- selection start ----------------------
        }
    }
}
## render
The render method is the same for templates and sub-templates.

##### step 5 of 5 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L77)
```rust
    ) -> Vec<Node>;
    // endregion: methods to be implemented for a specific project

    // region: the only 2 true public methods - default implementation code
    /// render for root template (not subtemplates) from file
    fn render_from_file(&self, template_file_name: &str) -> String {
#//---------------------- selection start ----------------------
        let mut template_raw = unwrap!(fs::read_to_string(&template_file_name));
        // find node <html >, jump over <!DOCTYPE html> because it is not microXml compatible
        // I will add <!DOCTYPE html> when the rendering ends, before returning the html.
        let pos_html = unwrap!(template_raw.find("<html"));
        template_raw.drain(..pos_html);
#//----------------------- selection end -----------------------
```
