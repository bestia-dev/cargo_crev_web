//! html_server_template_mod
//! html templating library for the web server
//! should be compatible also with svg, because of namespaces

// region: use
use crate::utils_mod::*;
use crate::*;
use reader_for_microxml::*;
use std::fs;
use unwrap::unwrap;
// endregion: use

#[derive(Clone, Debug)]
pub enum Node {
    // A text node.
    Text(String),
    // An element potentially with attributes and children.
    Element(ElementNode),
    // comment
    Comment(String),
}
#[derive(Clone, Debug)]
pub struct ElementNode {
    pub tag_name: String,
    pub attributes: Vec<Attribute>,
    pub children: Vec<Node>,
    pub namespace: Option<String>,
}
/// An attribute on a DOM node, such as `id="my-thing"`
#[derive(Clone, Debug)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}
/// Svg elements are different because they have a namespace
#[derive(Clone, Copy)]
pub enum HtmlOrSvg {
    // html element
    Html,
    // svg element
    Svg,
}

#[derive(Clone, Debug)]
pub struct SubTemplate {
    pub name: String,
    pub placeholder: String,
    pub template: String,
}
pub trait HtmlServerTemplateRender {
    // region: methods must be implemented for a specific project
    // because the data model is always different and is known only to the project.

    /// The code for templating starts here.
    /// renders the complete html file. Not a sub-template/fragment.
    fn render_html_file(&self, templates_folder_name: &str) -> String;
    /// name of data model for debugging
    fn data_model_name(&self) -> String;
    /// returns a String to replace the next text-node
    fn replace_with_string(
        &self,
        placeholder: &str,
        subtemplate: &str,
        pos_cursor: usize,
    ) -> String;
    //// boolean : is the next node rendered or not
    fn retain_next_node(&self, placeholder: &str) -> bool;
    /// returns a vector of Nodes to replace the next Node
    fn replace_with_nodes(&self, placeholder: &str) -> Vec<Node>;
    /// renders sub-template
    fn render_sub_template(
        &self,
        template_name: &str,
        sub_templates: &Vec<SubTemplate>,
    ) -> Vec<Node>;
    // endregion: methods must be implemented for a specific project

    // region: this other methods should be private
    // but I don't know how to do it in Rust.

    /// render root template (not sub-templates) from file
    fn render_from_file(&self, template_file_name: &str) -> String {
        let mut template_raw = unwrap!(fs::read_to_string(&template_file_name));
        // find node <html >, jump over <!DOCTYPE html> because it is not microXml compatible
        // I will add <!DOCTYPE html> when the rendering ends, before returning the html.
        let pos_html = unwrap!(template_raw.find("<html"));
        template_raw.drain(..pos_html);

        self.render(&template_raw)
    }
    /// render for root template (not subtemplates) from string
    fn render(&self, html_template_raw: &str) -> String {
        let nodes =
            unwrap!(self.render_template_raw_to_nodes(&html_template_raw, HtmlOrSvg::Html, "", 0));
        // because this is the root template it must return one ElementNode
        let mut html = s!("");
        match &nodes[0] {
            Node::Element(temp_element_node) => {
                html = unwrap!(Self::root_element_node_to_html_string(temp_element_node));
            }
            _ => eprintln!("Error: render_template_raw_to_nodes() does not return one ElementNode."),
        }
        //return
        html
    }

    /// this is used for templates and subtemplates equally
    /// first extracts all children sub_templates
    /// returns Nodes
    fn render_template_raw_to_nodes(
        &self,
        html_template_raw: &str,
        html_or_svg_parent: HtmlOrSvg,
        subtemplate: &str,
        pos_cursor: usize,
    ) -> Result<Vec<Node>, String> {
        // html_template_raw can be a fragment. I add the root, that will later be removed.
        let html_template_raw = &format!("<template>{}</template>", html_template_raw);
        // extract sub_templates. Only one level deep.
        let sub_templates = Self::extract_children_sub_templates(html_template_raw);
        // the index zero is the drained main template
        let mut reader_for_microxml = ReaderForMicroXml::new(&sub_templates[0].template);
        let mut dom_path: Vec<String> = Vec::new();
        let mut root_element;
        let mut html_or_svg_local = html_or_svg_parent;

        #[allow(clippy::single_match_else, clippy::wildcard_enum_match_arm)]
        // the root element must be only one
        match reader_for_microxml.read_event() {
            Event::StartElement(tag_name) => {
                dom_path.push(s!(tag_name));
                root_element = ElementNode {
                    tag_name: s!(tag_name),
                    attributes: vec![],
                    children: vec![],
                    namespace: None,
                };
                if &tag_name == &"svg" {
                    html_or_svg_local = HtmlOrSvg::Svg;
                }
                if let HtmlOrSvg::Svg = html_or_svg_local {
                    // svg elements have this namespace
                    root_element.namespace = Some(s!("http://www.w3.org/2000/svg"));
                }
                // recursive function can return error

                match self.fill_element_node(
                    &mut reader_for_microxml,
                    root_element,
                    html_or_svg_local,
                    &mut dom_path,
                    &sub_templates,
                    subtemplate,
                    pos_cursor,
                    // retain_next_node
                    true,
                ) {
                    Ok(new_root_element) => root_element = new_root_element,
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
            _ => {
                // return error
                return Err(s!("Error: no root element"));
            }
        }
        // remove the added root <template>
        // return its children
        Ok(root_element.children)
    }

    // Recursive function to fill the Element with attributes
    // and sub-nodes(Element, Text, Comment).
    #[allow(clippy::too_many_lines, clippy::type_complexity)]
    fn fill_element_node(
        &self,
        reader_for_microxml: &mut ReaderForMicroXml,
        mut element: ElementNode,
        html_or_svg_parent: HtmlOrSvg,
        dom_path: &mut Vec<String>,
        sub_templates: &Vec<SubTemplate>,
        subtemplate: &str,
        pos_cursor: usize,
        retain_this_node: bool,
    ) -> Result<ElementNode, String> {
        let mut replace_string: Option<String> = None;
        let mut replace_vec_nodes: Option<Vec<Node>> = None;
        let mut retain_next_node = retain_this_node;
        let mut html_or_svg_local;
        // loop through all the siblings in this iteration
        loop {
            // the children inherits html_or_svg from the parent, but cannot change the parent
            html_or_svg_local = html_or_svg_parent;
            match reader_for_microxml.read_event() {
                Event::StartElement(tag_name) => {
                    dom_path.push(s!(tag_name));
                    // construct a child element and fill it (recursive)
                    let mut child_element = ElementNode {
                        tag_name: s!(tag_name),
                        attributes: vec![],
                        children: vec![],
                        namespace: None,
                    };
                    if tag_name == "svg" {
                        // this tagname changes to svg now
                        html_or_svg_local = HtmlOrSvg::Svg;
                    }
                    if let HtmlOrSvg::Svg = html_or_svg_local {
                        // this is the
                        // svg elements have this namespace
                        child_element.namespace = Some(s!("http://www.w3.org/2000/svg"));
                    }
                    if tag_name == "foreignObject" {
                        // this tagname changes to html for children, not for this element
                        html_or_svg_local = HtmlOrSvg::Html;
                    }
                    // recursion
                    child_element = self.fill_element_node(
                        reader_for_microxml,
                        child_element,
                        html_or_svg_local,
                        dom_path,
                        sub_templates,
                        subtemplate,
                        pos_cursor,
                        retain_next_node,
                    )?;
                    // ignore this node dynamic content, and don't push to result
                    // but traverse all template nodes.
                    if retain_next_node == true {
                        if let Some(repl_vec_nodes) = replace_vec_nodes {
                            for repl_node in repl_vec_nodes {
                                element.children.push(repl_node);
                            }
                            replace_vec_nodes = None;
                        } else {
                            element.children.push(Node::Element(child_element));
                        }
                        // the siblings get the parents retain, until sb_
                        retain_next_node = retain_this_node;
                    }
                }
                Event::Attribute(name, value) => {
                    if retain_this_node == true {
                        if name.starts_with("data-st-") {
                            // placeholder is in the attribute value.
                            // the attribute name is informative and should be similar to the next attribute
                            // example: data-st-href="st_placeholder" href="x"
                            // The replace_string will always be applied to the next attribute. No matter the name.
                            let placeholder = &value;
                            let repl_txt =
                                self.replace_with_string(placeholder, subtemplate, pos_cursor);
                            replace_string = Some(repl_txt);
                        } else {
                            let value = if let Some(repl) = replace_string {
                                // empty the replace_string for the next node
                                replace_string = None;
                                decode_5_xml_control_characters(&repl)
                            } else {
                                decode_5_xml_control_characters(value)
                            };
                            element.attributes.push(Attribute {
                                name: s!(name),
                                value: value,
                            });
                        }
                    }
                }
                Event::TextNode(txt) => {
                    if retain_this_node == true {
                        let txt = if let Some(repl) = replace_string {
                            // empty the replace_string for the next node
                            replace_string = None;
                            decode_5_xml_control_characters(&repl)
                        } else {
                            decode_5_xml_control_characters(txt)
                        };
                        // here accepts only utf-8.
                        // only minimum html entities are decoded
                        element.children.push(Node::Text(txt));
                    }
                }
                Event::Comment(txt) => {
                    if retain_this_node == true {
                        // the main goal of comments is to change the value of the next text node
                        // with the result of a function
                        // it must look like <!--st_get_text-->

                        if txt.starts_with("st_") {
                            let repl_txt = self.replace_with_string(txt, subtemplate, pos_cursor);
                            replace_string = Some(repl_txt);
                        } else if txt.starts_with("sb_") {
                            // boolean if this is true than render the next node, else don't render
                            retain_next_node = self.retain_next_node(txt);
                        } else if txt.starts_with("stmplt_") {
                            // replace exactly this placeholder for a sub-template
                            let template_name = txt.trim_end_matches(" start");
                            let repl_vec_nodes =
                                self.render_sub_template(template_name, sub_templates);
                            element.children.extend_from_slice(&repl_vec_nodes);
                        } else if txt.starts_with("sn_") {
                            // nodes  (in a vector)
                            let repl_vec_nodes = self.replace_with_nodes(txt);
                            replace_vec_nodes = Some(repl_vec_nodes);
                        } else {
                            // it is really a comment, retain it.
                            element.children.push(Node::Comment(s!(txt)));
                        }
                    }
                }
                Event::EndElement(name) => {
                    let last_name = unwrap!(dom_path.pop());
                    // it can be also auto-closing element
                    if last_name == name || name == "" {
                        return Ok(element);
                    } else {
                        return Err(format!(
                            "End element not correct: starts <{}> ends </{}>",
                            last_name, name
                        ));
                    }
                }
                Event::Error(error_msg) => {
                    return Err(s!(error_msg));
                }
                Event::Eof => {
                    return Ok(element);
                }
            }
        }
        /// private fn - decode 5 xml control characters : " ' & < >
        /// https://www.liquid-technologies.com/XML/EscapingData.aspx
        /// I will ignore all html entities, to keep things simple,
        /// because all others characters can be written as utf-8 characters.
        /// it is mandatory that text is encoded in utf-8.
        /// https://www.tutorialspoint.com/html5/html5_entities.htm
        fn decode_5_xml_control_characters(input: &str) -> String {
            // The standard library replace() function makes allocation,
            // but is probably fast enough for my use case.
            input
                .replace("&quot;", "\"")
                .replace("&apos;", "'")
                .replace("&amp;", "&")
                .replace("&lt;", "<")
                .replace("&gt;", ">")
        }
    }

    /// extracts and saves sub_templates only one level deep: children
    fn extract_children_sub_templates(template_raw: &str) -> Vec<SubTemplate> {
        // drain sub-template from main template and save into vector
        // the sub_templates[0] is the main_template
        // the main template will change with draining sub-templates
        let mut sub_templates = vec![SubTemplate {
            name: s!("main_template"),
            template: s!(template_raw),
            placeholder: String::new(),
        }];

        // the syntax is <!--stmplt_crate_version_summary start-->, <!--stmplt_crate_version_summary end-->
        // unique delimiters for start and end are great if there is nesting.
        let mut pos_for_loop = 0;
        loop {
            let mut exist_template = false;
            if let Some(pos_start) =
                find_pos_before_delimiter(&sub_templates[0].template, pos_for_loop, "<!--stmplt_")
            {
                if let Some(pos_end_name) =
                    find_pos_before_delimiter(&sub_templates[0].template, pos_start, " start-->")
                {
                    let sub_template_name =
                        s!(&sub_templates[0].template[pos_start + 4..pos_end_name]);
                    // dbg!(sub_template_name);
                    let pos_start_after_tag = pos_end_name + 9;
                    let end_tag = format!("<!--{} end-->", sub_template_name);
                    if let Some(pos_end_after_tag) =
                        find_pos_after_delimiter(&sub_templates[0].template, pos_start, &end_tag)
                    {
                        exist_template = true;
                        // special name for template that will not be used at all.
                        // this happens when the graphic designer need more repetition of the
                        // same sub-template only for visual effect while editing.
                        if sub_template_name == "stmplt_not_for_render" {
                            // dbg!(pos_start);
                            // dbg!(pos_end_after_tag);
                            // remove all the template
                            sub_templates[0]
                                .template
                                .drain(pos_start..pos_end_after_tag);
                        } else {
                            let sub_template_placeholder =
                                s!(&sub_templates[0].template[pos_start..pos_start_after_tag]);
                            pos_for_loop = pos_start_after_tag;

                            // drain - extract a substring and remove it from the original
                            // leave the header with the name. It will be used
                            // as placeholder for replace later.
                            let sub_template: String = sub_templates[0]
                                .template
                                .drain(pos_start_after_tag..pos_end_after_tag)
                                .collect();
                            // remove the end tag
                            let sub_template = sub_template.trim_end_matches(&end_tag);
                            sub_templates.push(SubTemplate {
                                name: s!(sub_template_name),
                                placeholder: s!(sub_template_placeholder),
                                template: s!(sub_template),
                            });
                            // dbg!(sub_template);
                        }
                    }
                }
            }
            if !exist_template {
                break;
            }
        }
        // dbg!(sub_templates.len());
        // return
        sub_templates
    }

    /// converts element node to string
    fn root_element_node_to_html_string(element_node: &ElementNode) -> Result<String, String> {
        /// recursive private fn sub element to html
        fn sub_element_node_mut_html(html: &mut String, element_node: &ElementNode) {
            html.push_str("<");
            html.push_str(&element_node.tag_name);
            html.push_str(" ");
            for attr in &element_node.attributes {
                html.push_str(&attr.name);
                html.push_str(" = \"");
                html.push_str(&attr.value);
                html.push_str("\" ");
            }
            html.push_str(">");
            for sub_elem in &element_node.children {
                match &sub_elem {
                    Node::Element(sub_element) => {
                        // recursion
                        sub_element_node_mut_html(html, sub_element);
                    }
                    Node::Text(text) => html.push_str(&text),
                    Node::Comment(text) => html.push_str(&format!("<!--{}-->", &text)),
                }
            }
            // end tag
            html.push_str("</");
            html.push_str(&element_node.tag_name);
            html.push_str(">");
        }

        let mut html = String::with_capacity(5000);
        html.push_str("<!DOCTYPE html>");
        sub_element_node_mut_html(&mut html, element_node);
        // return
        Ok(html)
    }
}
// region: utility fn
/// boilerplate
pub fn retain_next_node_match_else(data_model_name: &str, placeholder: &str) -> bool {
    eprintln!("Error: Unrecognized {} retain_next_node: \"{}\"",
        data_model_name, placeholder
    );
    true
}
/// boilerplate
pub fn replace_with_string_match_else(data_model_name: &str, placeholder: &str) -> String {
    let err_msg = format!(
        "Error: Unrecognized {} replace_with_string: \"{}\"",
        data_model_name, placeholder
    );
    eprintln!("{}", &err_msg);
    err_msg
}
/// boilerplate
pub fn replace_with_nodes_match_else(data_model_name: &str, placeholder: &str) -> Vec<Node> {
    let err_msg = format!(
        "Error: Unrecognized {} replace_with_nodes: \"{}\"",
        data_model_name, placeholder
    );
    eprintln!("{}", &err_msg);
    let node = Node::Element(ElementNode {
        tag_name: s!("h2"),
        attributes: vec![],
        children: vec![Node::Text(err_msg)],
        namespace: None,
    });
    return vec![node];
}
///boilerplate
pub fn render_sub_template_match_else(data_model_name: &str, template_name: &str) -> Vec<Node> {
    let err_msg = format!(
        "Error: Unrecognized {} render_sub_template: \"{}\"",
        data_model_name, template_name
    );
    eprintln!("{}", &err_msg);
    let node = Node::Element(ElementNode {
        tag_name: s!("h2"),
        attributes: vec![],
        children: vec![Node::Text(err_msg)],
        namespace: None,
    });
    return vec![node];
}
/// to string, but zero converts to empty
pub fn to_string_zero_to_empty(number: usize) -> String {
    if number == 0 {
        s!("")
    } else {
        number.to_string()
    }
}
// endregion: utility fn
