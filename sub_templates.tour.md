# sub_templates_children
## sub-template
A sub-template is needed when the data is from a different data model or when is needed to repeat a sub-template for a vector of data. The syntax is like \<!--template_all_summaries start--\>. It ends with \<!--template_all_summaries end--\>

##### step 1 of 3 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/webfolder/templates/query/crev_query_template.html#L14)
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

##### step 2 of 3 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L141)
```rust
    /// Recursive function to fill the Element with attributes
    /// and sub-nodes(Element, Text, Comment).  
    #[allow(clippy::too_many_lines, clippy::type_complexity)]
    fn fill_element_node(
        &self,
        reader_for_microxml: &mut ReaderForMicroXml,
        mut element: ElementNode,
        html_or_svg_parent: HtmlOrSvg,
        dom_path: &mut Vec<String>,
        sub_templates: &Vec<SubTemplate>,
#//---------------------- selection start ----------------------
    ) -> Result<ElementNode, String> {
#//----------------------- selection end -----------------------
```
## template placeholder
When the template placeholder is found, the method render_sub_template is called.
The result is added to the parent node.

##### step 3 of 3 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L316)
```rust
                .replace("&gt;", ">")
        }
    }

    /// extract and saves sub_templates only one level deep, children
    fn extract_children_sub_templates(template_raw: &str) -> Vec<SubTemplate> {
        // drain sub-template from main template and save into vector
        // the sub_templates[0] is the main_template
        // the main template will change with draining sub-templates
        let mut sub_templates = vec![SubTemplate {
#//---------------------- selection start ----------------------
            name: "main_template".to_string(),
#//----------------------- selection end -----------------------
```
