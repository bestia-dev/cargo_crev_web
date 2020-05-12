//! html_template_mod

// region: use
use crate::utils_mod::*;
use reader_for_microxml::*;
use std::fs;
use unwrap::unwrap;
// endregion: use

// #region: template and sub-templates
#[derive(Clone, Debug)]
pub struct SubTemplate {
    pub name: String,
    pub placeholder: String,
    pub template: String,
}

pub fn template_raw_from_file(file_name: &str) -> String {
    let mut template_raw = unwrap!(fs::read_to_string(file_name));
    // find node <html >, jump over <!DOCTYPE html> because it is not microXml compatible
    // I will add <!DOCTYPE html> when the rendering ends.
    let pos_html = unwrap!(template_raw.find("<html"));
    template_raw.drain(..pos_html);
    //return
    template_raw
}

pub fn extract_sub_templates_from_file(file_name: &str) -> Vec<SubTemplate> {
    let template_raw = template_raw_from_file(file_name);
    // Returns the main template and children sub-templates. Only depth 1.
    //return
    extract_sub_templates(&template_raw)
}
/// extract and saves sub_templates from local file_name
pub fn extract_sub_templates(template_str: &str) -> Vec<SubTemplate> {
    // region: private fn
    /// drain sub-template from super-template and save into vector
    /// only one level deep
    fn drain_and_save_sub_templates(sub_templates: &mut Vec<SubTemplate>, index: usize) {
        // the syntax is <!--template_all_summaries start-->, <!--template_all_summaries end-->
        // unique delimiters for start and end are great if there is nesting.
        let mut pos_for_loop = 0;
        loop {
            let mut exist_template = false;
            if let Some(pos_start) = find_pos_before_delimiter(
                &sub_templates[index].template,
                pos_for_loop,
                "<!--template_",
            ) {
                if let Some(pos_end_name) = find_pos_before_delimiter(
                    &sub_templates[index].template,
                    pos_start,
                    " start-->",
                ) {
                    let sub_template_name =
                        sub_templates[index].template[pos_start + 4..pos_end_name].to_string();
                    //println!("sub_template_name: {}", sub_template_name);
                    let pos_start_after_tag = pos_end_name + 9;
                    let end_tag = format!("<!--{} end-->", sub_template_name);
                    if let Some(pos_end_after_tag) = find_pos_after_delimiter(
                        &sub_templates[index].template,
                        pos_start,
                        &end_tag,
                    ) {
                        exist_template = true;
                        // special name for template that will not be used at all.
                        // this happens when the graphic designer need more repetition of the
                        // same sub-template only for visual effect while editing.
                        if sub_template_name == "template_not_for_render" {
                            //println!("template_not_for_render {} {}",pos_start, pos_end_after_tag);
                            //remove all the template
                            sub_templates[index]
                                .template
                                .drain(pos_start..pos_end_after_tag);
                        } else {
                            let sub_template_placeholder = sub_templates[index].template
                                [pos_start..pos_start_after_tag]
                                .to_string();
                            pos_for_loop = pos_start_after_tag;

                            // drain - extract a substring and remove it from the original
                            // leave the header with the name. It will be used
                            // as placeholder for replace later.
                            let sub_template: String = sub_templates[index]
                                .template
                                .drain(pos_start_after_tag..pos_end_after_tag)
                                .collect();
                            // remove the end tag
                            let sub_template = sub_template.trim_end_matches(&end_tag);
                            sub_templates.push(SubTemplate {
                                name: sub_template_name.to_string(),
                                placeholder: sub_template_placeholder.to_string(),
                                template: sub_template.to_string(),
                            });
                            //println!("{}",sub_template);
                        }
                    }
                }
            }
            if !exist_template {
                break;
            }
        }
    }
    // endregion: private fn
    //read the local file template

    // the sub_templates[0] is the main_template
    let mut sub_templates = vec![SubTemplate {
        name: "main_template".to_string(),
        template: template_str.to_string(),
        placeholder: String::new(),
    }];
    // loop to drain and save sub_templates and their sub-templates in depth levels
    let mut index = 0;
    //the vector may acquires new members on every loop
    //each one must be processed for sub-templates
    while index < sub_templates.len() {
        drain_and_save_sub_templates(&mut sub_templates, index);
        index += 1;
    }
    //println!("sub_templates.len(): {}", sub_templates.len());
    //return
    sub_templates
}
// endregion: template and sub-templates

// region: render_template
#[derive(Clone, Debug)]
pub struct Node {
    pub node_enum: NodeEnum,
}
#[derive(Clone, Debug)]
pub enum NodeEnum {
    /// A text node.
    Text(String),
    /// An element potentially with attributes and children.
    Element(ElementNode),
    /// comment
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
    /// html element
    Html,
    /// svg element
    Svg,
}
pub trait HtmlTemplating {
    // region: methods to be implemented for a specific project
    // these are not really public methods. They are used only as
    //plumbing between trait declaration and implementation
    // while rendering, cannot mut rrc
    fn call_fn_string(&self, fn_name: &str) -> String;
    fn call_fn_boolean(&self, fn_name: &str) -> bool;
    // this is also for sub-templates
    fn call_fn_vec_nodes(&self, fn_name: &str) -> Vec<ElementNode>;
    fn render_sub_template(
        &self,
        template_name: &str,
        sub_templates: &Vec<SubTemplate>,
    ) -> Vec<ElementNode>;
    // endregion: methods to be implemented for a specific project

    // region: the only true public method - default implementation code
    /// default implementation - render template to string
    fn render_template_to_string(
        &self,
        html_template_raw: &str,
        html_or_svg_parent: HtmlOrSvg,
    ) -> Result<String, String> {
        // region: private fn
        /// sub element to html
        fn element_node_to_html(html: &mut String, element_node: ElementNode) {
            html.push_str("<");
            html.push_str(&element_node.tag_name);
            html.push_str(" ");
            for attr in element_node.attributes {
                html.push_str(&attr.name);
                html.push_str(" = \"");
                html.push_str(&attr.value);
                html.push_str("\" ");
            }
            html.push_str(">");
            for sub_elem in element_node.children {
                match sub_elem.node_enum {
                    NodeEnum::Element(sub_element) => {
                        //recursion
                        element_node_to_html(html, sub_element);
                    }
                    NodeEnum::Text(text) => html.push_str(&text),
                    NodeEnum::Comment(text) => html.push_str(&format!("<!--{}-->", &text)),
                }
            }
            //end tag
            html.push_str("</");
            html.push_str(&element_node.tag_name);
            html.push_str(">");
        }
        // region: private fn

        // Every template will be wrapped in a <template></template> node to assure
        // a unique root node. At the end this temporary node will be discarded.
        let html_template_raw = &format!("<template>{}</template>", html_template_raw);
        let element_node = self.extract_sub_templates_and_render_template_to_element_node(
            html_template_raw,
            html_or_svg_parent,
        )?;

        let mut html = String::with_capacity(html_template_raw.len() * 2);
        element_node_to_html(&mut html, element_node);
        html.drain(.."<template>".len() + 1);
        html.drain(html.len() - "</template>".len()..);
        //return
        Ok(html)
    }
    // endregion: default implementation
    // region: this methods should be private somehow, but I don't know in Rust how to do it
    /// extract subtemlates and get root element Node.   
    fn extract_sub_templates_and_render_template_to_element_node(
        &self,
        html_template_raw: &str,
        html_or_svg_parent: HtmlOrSvg,
    ) -> Result<ElementNode, String> {
        // extract sub_templates. Only one level deep.
        let sub_templates = extract_sub_templates(html_template_raw);
        // the index zero is the drained main template

        let mut reader_for_microxml = ReaderForMicroXml::new(&sub_templates[0].template);
        let mut dom_path: Vec<String> = Vec::new();
        let mut root_element;
        let mut html_or_svg_local = html_or_svg_parent;

        #[allow(clippy::single_match_else, clippy::wildcard_enum_match_arm)]
        // the root element must be only one
        match reader_for_microxml.read_event() {
            Event::StartElement(tag_name) => {
                dom_path.push(tag_name.to_string());
                root_element = ElementNode {
                    tag_name: tag_name.to_string(),
                    attributes: vec![],
                    children: vec![],
                    namespace: None,
                };
                if &tag_name == &"svg" {
                    html_or_svg_local = HtmlOrSvg::Svg;
                }
                if let HtmlOrSvg::Svg = html_or_svg_local {
                    // svg elements have this namespace
                    root_element.namespace = Some(String::from("http://www.w3.org/2000/svg"));
                }
                // recursive function can return error

                match self.fill_element_node(
                    &mut reader_for_microxml,
                    root_element,
                    html_or_svg_local,
                    &mut dom_path,
                    &sub_templates,
                ) {
                    // the methods are move, so I have to return the moved value
                    Ok(new_root_element) => root_element = new_root_element,
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
            _ => {
                // return error
                return Err("Error: no root element".to_owned());
            }
        }
        // return
        Ok(root_element)
    }

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
    ) -> Result<ElementNode, String> {
        let mut replace_string: Option<String> = None;
        let mut replace_node: Option<Node> = None;
        let mut replace_vec_nodes: Option<Vec<Node>> = None;
        let mut replace_boolean: Option<bool> = None;
        let mut html_or_svg_local;
        // loop through all the siblings in this iteration
        loop {
            // the children inherits html_or_svg from the parent, but cannot change the parent
            html_or_svg_local = html_or_svg_parent;
            match reader_for_microxml.read_event() {
                Event::StartElement(tag_name) => {
                    dom_path.push(tag_name.to_owned());
                    // construct a child element and fill it (recursive)
                    let mut child_element = ElementNode {
                        tag_name: String::from(tag_name),
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
                        child_element.namespace = Some("http://www.w3.org/2000/svg".to_string());
                    }
                    if tag_name == "foreignObject" {
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
                    )?;
                    // if the boolean is empty or true then render the next node
                    if replace_boolean.unwrap_or(true) {
                        if let Some(repl_node) = replace_node {
                            element.children.push(repl_node);
                            replace_node = None;
                        } else if let Some(repl_vec_nodes) = replace_vec_nodes {
                            for repl_node in repl_vec_nodes {
                                element.children.push(repl_node);
                            }
                            replace_vec_nodes = None;
                        } else {
                            element.children.push(Node {
                                node_enum: NodeEnum::Element(child_element),
                            });
                        }
                    }
                    if replace_boolean.is_some() {
                        replace_boolean = None;
                    }
                }
                Event::Attribute(name, value) => {
                    if name.starts_with("data-t-") {
                        // fn_name is in the attribute value.
                        // the attribute name is informative and should be similar to the next attribute
                        // example: data-t-href="t_fn_name" href="x"
                        // The replace_string will always be applied to the next attribute. No matter the name.
                        let fn_name = &value;
                        let repl_txt = self.call_fn_string(fn_name);
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
                            name: name.to_string(),
                            value: value,
                        });
                    }
                }
                Event::TextNode(txt) => {
                    let txt = if let Some(repl) = replace_string {
                        // empty the replace_string for the next node
                        replace_string = None;
                        decode_5_xml_control_characters(&repl)
                    } else {
                        decode_5_xml_control_characters(txt)
                    };
                    // here accepts only utf-8.
                    // only minimum html entities are decoded
                    element.children.push(Node {
                        node_enum: NodeEnum::Text(txt),
                    });
                }
                Event::Comment(txt) => {
                    // the main goal of comments is to change the value of the next text node
                    // with the result of a function
                    // it must look like <!--t_get_text-->

                    if txt.starts_with("t_") {
                        let repl_txt = self.call_fn_string(txt);
                        replace_string = Some(repl_txt);
                    } else if txt.starts_with("n_") {
                        // nodes  (in a vector)
                        let repl_vec_element_nodes = self.call_fn_vec_nodes(txt);
                        //TODO: nodes or element nodes?
                        let mut repl_vec_nodes = vec![];
                        for el in &repl_vec_element_nodes {
                            repl_vec_nodes.push(Node {
                                node_enum: NodeEnum::Element(el.clone()),
                            });
                        }
                        replace_vec_nodes = Some(repl_vec_nodes);
                    } else if txt.starts_with("b_") {
                        // boolean if this is true than render the next node, else don't render
                        replace_boolean = Some(self.call_fn_boolean(txt));
                    } else if txt.starts_with("template_") {
                        // replace exactly this placeholder for a sub-template
                        let repl_vec_nodes = self.call_fn_vec_nodes(txt);
                        for repl_node in repl_vec_nodes {
                            element.children.push(Node {
                                node_enum: NodeEnum::Element(repl_node.clone()),
                            });
                        }
                    } else {
                        // it is really a comment
                        element.children.push(Node {
                            node_enum: NodeEnum::Comment(txt.to_string()),
                        });
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
                    return Err(error_msg.to_string());
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
        /// https://www.tutorialspoint.com/html5/html5_entities.htm  
        fn decode_5_xml_control_characters(input: &str) -> String {
            // The standard library replace() function makes allocation,
            //but is probably fast enough for my use case.
            input
                .replace("&quot;", "\"")
                .replace("&apos;", "'")
                .replace("&amp;", "&")
                .replace("&lt;", "<")
                .replace("&gt;", ">")
        }
    }

    // region: this methods should be private somehow, but I don't know in Rust how to do it
}
pub fn to_string_zero_to_empty(number: usize) -> String {
    if number == 0 {
        "".to_string()
    } else {
        number.to_string()
    }
}
