# crev_query
## server route  
The web server recognizes the route /crate/ and calls html_for_crev_query().

##### step 1 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/main.rs#L266)
```rust
    clippy::nursery,
    clippy::cargo,
    // variable shadowing is idiomatic to Rust, but unnatural to me.
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::shadow_unrelated,
)]
#![allow(
    // library from dependencies have this clippy warnings. Not my code.
#//---------------------- selection start ----------------------
    clippy::cargo_common_metadata,
    clippy::multiple_crate_versions,
#//----------------------- selection end -----------------------
```
## data model  
Prepare CrevQueryData. This is the data model with all the data for templating in one place.

##### step 2 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/crev_query_mod.rs#L32)
## template on disk  
Read the template from the disk and start the rendering.

##### step 3 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/crev_query_mod.rs#L37)
## render_template_raw_to_nodes  
This default trait method for rendering has no special knowledge about the data. Only about html templates. The templating works with Nodes.

##### step 4 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L77)
## must implement methods
In the same trait we have specific functions that must be implemented for every data model separately.

##### step 5 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L66)
## extract children subtemplates  
The template can contain sub-templates. Here extract only the children (depth level 1).
The parent template is drained from subtemplates. Only a placeholder is retained for later replacement.

##### step 6 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L81)
## read template Tokens
The reader_for_microxml moves Token by Token sequentially. For different types of Tokens there is different code. Here we transform the input String into a Vec\<Node\> for easy manipulation.

##### step 7 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L153)
## new node
A new html node/element/tag. We create a new Node with only the basic data.

##### step 8 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L160)
## Svg namespace
Svg inside Html must be specially adorned with a namespace. Very annoying.

##### step 9 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L169)
## fill node recursively
The new node we created will be filled in this method. This goes recursive.

##### step 10 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L181)
## fill_element_node()  
This is the recursive method. It accepts a newly created ElementNode and fills it with attributes and children. Most of the template is just copied. Special Comments and data- attributes are points in the template to replace with dynamic content.

##### step 11 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L141)
## static html template
The template's life starts as static content. The graphic designer can copy the html file to his disk and open it with the browser. He can use a text editor to change html and css design. The template contains static sample data similar to the dynamic data. So the designer has the visual clue how all will look at the end.

##### step 12 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/web_server_folder/templates/query/crev_query_template.html#L28)
## dynamic content
Modifying the visuals of a web page is an eternal task. Let's separate as much as possible the work of the graphic designer and of the (data) developer.
Once the graphic design is ready, we need to add placeholders for dynamic data. This placeholders will be replaced with dynamic data while rendering. The placeholders must not destroy the capability of the html file to be viewed statically. I chose to use html comments, for example \<!--t_number--\> 

##### step 13 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/web_server_folder/templates/query/crev_query_template.html#L42)
## replace text  
The static text "1" for this text node is used for the graphic preview. 
To replace it with dynamic data, we add before it a comment with the special syntax \<!--t_name--\>. 

##### step 14 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/web_server_folder/templates/query/crev_query_template.html#L48)
## text placeholder
The rendering finds the special comment. It calls `call_fn_string` and temporarily saves the result. 
It does not push the placeholder comment to the html nodes, because is not needed in the result html.

##### step 15 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L246)
## dynamic data
Every placeholder has code that returns dynamic data as a string. This method is implemented on the data model, so it has access to all the data it needs.

##### step 16 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/all_summary_mod.rs#L146)
## next TextNode
When the rendering goes to the next TextNode it does not use the static content. 
It uses the dynamic content temporarily saved.

##### step 17 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L229)
## push to parent node
Then this dynamic TextNode is pushed to the parent node.

##### step 18 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L237)
## boolean placeholder
The special comment \<!--b_...--\> can result in true or false. 
It leaves or removes the next node completely.

##### step 19 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/web_server_folder/templates/query/crev_query_template.html#L105)
## boolean
The rendering finds the placeholder and calls the implementation method.
It saves temporarily the result.

##### step 20 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L249)
## next node
Before rendering the next node we look at the temporary value retain_next_node_or_attribute.
If it is false, then we don't render the next node. Just jump over it.

##### step 21 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/html_template_mod.rs#L183)
## boolean method
The implemented method returns true or false for the placeholder according to the data.

##### step 22 of 22 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/proof_mod.rs#L125)
