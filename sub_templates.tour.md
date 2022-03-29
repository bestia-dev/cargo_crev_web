# sub_templates_children
## sub-template
A sub-template is needed when the data is from a different data model or when is needed to repeat a sub-template for a vector of data. The syntax is like \<!--template_all_summaries start--\>. It ends with \<!--template_all_summaries end--\>

##### step 1 of 5 [View code in GitHub](https://github.com/bestia-dev/cargo_crev_web/blob/master/web_server_folder/templates/query/crev_query_template.html#L14)
## extract sub-template
Only one level deep, only the children. Then recursively the chldren will extract their children. Save them in a vector. The [0] member is the parent template. It is drained from the sub-templates. Only a placeholder remains.

##### step 2 of 5 [View code in GitHub](https://github.com/bestia-dev/cargo_crev_web/blob/master/src/html_template_mod.rs#L81)
## template placeholder
When the template placeholder is found, the method render_sub_template is called.
The result is added to the parent node.

##### step 3 of 5 [View code in GitHub](https://github.com/bestia-dev/cargo_crev_web/blob/master/src/html_template_mod.rs#L256)
## render the sub-template
Find it in the sub_templates vector. If the data is a vector, then render_template for every element. Push all nodes to parent node (extend_from_slice).

##### step 4 of 5 [View code in GitHub](https://github.com/bestia-dev/cargo_crev_web/blob/master/src/all_summary_mod.rs#L219)
## render
The render method is the same for templates and sub-templates.

##### step 5 of 5 [View code in GitHub](https://github.com/bestia-dev/cargo_crev_web/blob/master/src/html_template_mod.rs#L77)
