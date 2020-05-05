//! html_template_mod

// region: use
use crate::utils_mod::*;

use reader_for_microxml::*;
//use serde_derive::{Deserialize, Serialize};
use std::fs;
use unwrap::unwrap;

// endregion: use

// TODO: first I allocate to all this structs. Better would be to borrow,
// but lifetimes are a headache.

#[derive(Clone, Debug)]
pub struct SubTemplate {
    name: String,
    placeholder: String,
    template: String,
}
#[derive(Clone, Debug)]
pub struct TemplateAndSubTemplates {
    template: String,
    sub_templates: Vec<SubTemplate>,
}
#[derive(PartialEq)]
pub enum SubTemplateVisibility {
    Visible,
    Invisible,
}
/// extract and saves sub_templates
pub fn prepare_template_and_sub_templates(file_name: &str) -> TemplateAndSubTemplates {
    //read the local file template
    let template = unwrap!(fs::read_to_string(file_name));
    // only the html inside the <body> </body>
    let tm = between_body_tag(&template);

    let mut template_and_sub_templates = TemplateAndSubTemplates {
        template: tm,
        sub_templates: vec![],
    };
    // drain and save sub_templates
    drain_sub_templates(
        &mut template_and_sub_templates,
        SubTemplateVisibility::Invisible,
    );
    drain_sub_templates(
        &mut template_and_sub_templates,
        SubTemplateVisibility::Visible,
    );

    //return
    template_and_sub_templates
}
/// invisible sub_templates are in node <template name="xxx"></template>
/// visible sub_template have the same node, but is commented out <!--<template name="xxx">-->
fn drain_sub_templates(
    template_and_sub_templates: &mut TemplateAndSubTemplates,
    visible_or_invisible: SubTemplateVisibility,
) {
    let (start_delim, end_tag_delim, end_delim) =
        if visible_or_invisible == SubTemplateVisibility::Invisible {
            ("<!--<template ", "<!--</template>-->", ">-->")
        } else {
            ("<template ", "</template>", ">")
        };
    let mut pos_for_loop = 0;
    loop {
        let mut exist_template = false;
        // just for a shorter name
        let tm = &mut template_and_sub_templates.template;
        if let Some(pos_start) = find_pos_before_delimiter(tm, pos_for_loop, start_delim) {
            if let Some(pos_end_after_tag) = find_pos_after_delimiter(tm, pos_start, end_tag_delim)
            {
                exist_template = true;
                let sub_template_name;
                //extract name of sub_template
                if let Some(pos_name_start) = find_pos_after_delimiter(tm, pos_start, "name=\"") {
                    if let Some(pos_name_end) = find_pos_before_delimiter(tm, pos_name_start, "\"")
                    {
                        sub_template_name = tm[pos_name_start..pos_name_end].to_string();
                        println!("sub_template_name: {}", sub_template_name);
                        if let Some(pos_start_after_tag) =
                            find_pos_after_delimiter(tm, pos_name_end, end_delim)
                        {
                            // special name for template that will not be used at all.
                            // this happens when the graphic designer need more repetition of the
                            // same sub-template only for visual effect while editing.
                            if sub_template_name == "s_not_for_render" {
                                //remove all the template
                                tm.drain(pos_start..pos_end_after_tag);
                            } else {
                                let sub_template_placeholder =
                                    tm[pos_start..pos_start_after_tag].to_string();
                                pos_for_loop = pos_start_after_tag;

                                // drain - extract a substring and remove it from the original
                                // leave the header with the name. It will be used
                                // as placeholder for replace later.
                                let sub_template: String =
                                    tm.drain(pos_start_after_tag..pos_end_after_tag).collect();
                                // remove the end tag
                                let sub_template = sub_template.trim_end_matches(end_tag_delim);
                                template_and_sub_templates.sub_templates.push(SubTemplate {
                                    name: sub_template_name.to_string(),
                                    placeholder: sub_template_placeholder.to_string(),
                                    template: sub_template.to_string(),
                                });
                                //println!("{}",sub_template);
                            }
                        }
                    }
                }
            }
        }
        if !exist_template {
            break;
        }
    }
}
#[derive(Clone, Debug)]
pub struct Node {
    pub node_enum: NodeEnum,
}
#[derive(Clone, Debug)]
pub enum NodeEnum {
    /// A text node.
    Text(TextNode),
    /// An element potentially with attributes and children.
    Element(ElementNode),
}
#[derive(Clone, Debug)]
pub struct TextNode {
    pub text: String,
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
    // while rendering, cannot mut rrc

    fn call_fn_string(&self, fn_name: &str) -> String;
    fn call_fn_boolean(&self, fn_name: &str) -> bool;
    fn call_fn_node(&self, fn_name: &str) -> Node;
    fn call_fn_vec_nodes(&self, fn_name: &str) -> Vec<Node>;

    /*
    fn call_fn_listener(
        &self,
        fn_name: String,
    ) -> Box<dyn Fn(&mut dyn RootRender, VdomWeak, web_sys::Event) + 'static>;
    */
    // endregion: methods to be implemented

    // region: generic code (in trait definition)

    /// get root element Node.   
    fn render_template(
        &self,
        html_template: &str,
        html_or_svg_parent: HtmlOrSvg,
    ) -> Result<Node, String> {
        let mut reader_for_microxml = ReaderForMicroXml::new(html_template);
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
        Ok(Node {
            node_enum: NodeEnum::Element(root_element),
        })
    }

    /// Recursive function to fill the Element with attributes and sub-nodes(Element, Text, Comment).  
    /// Moves & Returns ElementBuilder or error.  
    /// I must `move` ElementBuilder because its methods are all `move`.  
    /// It makes the code less readable. It is only good for chaining and type changing.  

    #[allow(clippy::too_many_lines, clippy::type_complexity)]
    fn fill_element_node(
        &self,
        reader_for_microxml: &mut ReaderForMicroXml,
        mut element: ElementNode,
        html_or_svg_parent: HtmlOrSvg,
        dom_path: &mut Vec<String>,
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
                    if name.starts_with("data-t_") {
                        // fn_name is in the attribute name.
                        // the value is only informative for what attribute it should replace.
                        // The replace_string will always be applied to the next attribute.
                        let fn_name = &name[5..];
                        let repl_txt = self.call_fn_string(fn_name);
                        replace_string = Some(repl_txt);
                    /*
                    } else if name.starts_with("data-on_") {
                        // Only one listener for now because the api does not give me other method.
                        let fn_name = &name[5..];
                        let event_to_listen = unwrap!(name.get(8..)).to_string();
                    //println!("{}","&event_to_listen");
                    //println!("{}",&event_to_listen);
                    //element = element.add_listener(event_to_listen, self.call_fn_listener(fn_name));
                    */
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
                        node_enum: NodeEnum::Text(TextNode { text: txt }),
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
                        let repl_node = self.call_fn_node(txt);
                        replace_node = Some(repl_node);
                    } else if txt.starts_with("v_") {
                        // vector of nodes
                        let repl_vec_nodes = self.call_fn_vec_nodes(txt);
                        replace_vec_nodes = Some(repl_vec_nodes);
                    } else if txt.starts_with("b_") {
                        // boolean if this is true than render the next node, else don't render
                        replace_boolean = Some(self.call_fn_boolean(txt));
                    } else {
                        // nothing. it is really a comment
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
    }
    // endregion: generic code
}

/// decode 5 xml control characters : " ' & < >  
/// https://www.liquid-technologies.com/XML/EscapingData.aspx
/// I will ignore all html entities, to keep things simple,
/// because all others characters can be written as utf-8 characters.
/// https://www.tutorialspoint.com/html5/html5_entities.htm  
pub fn decode_5_xml_control_characters(input: &str) -> String {
    // The standard library replace() function makes allocation,
    //but is probably fast enough for my use case.
    input
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
}

pub fn from_node_to_string(root_node: Node) -> String {
    let mut html = String::with_capacity(5000);

    match root_node.node_enum {
        NodeEnum::Element(element_node) => element_node_to_html(&mut html, element_node),
        NodeEnum::Text(text_node) => println!("root_node must not be a text node: {:?}", text_node),
    }
    //return
    html
}

// sub element to html
pub fn element_node_to_html(html: &mut String, element_node: ElementNode) {
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
            NodeEnum::Text(text_node) => html.push_str(&text_node.text),
        }
    }
    //end tag
    html.push_str("</");
    html.push_str(&element_node.tag_name);
    html.push_str(">");
}
/// only the html between the <body> </body>
/// it must be a SINGLE root node
pub fn between_body_tag(resp_body_text: &str) -> String {
    let pos1 = resp_body_text.find("<body>").unwrap_or(0);
    let pos2 = resp_body_text.find("</body>").unwrap_or(0);
    // return
    if pos1 == 0 {
        resp_body_text.to_string()
    } else {
        #[allow(clippy::integer_arithmetic)]
        {
            unwrap!(resp_body_text.get(pos1 + 6..pos2)).to_string()
        }
    }
}
