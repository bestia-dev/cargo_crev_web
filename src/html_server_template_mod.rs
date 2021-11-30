//! html_server_template_mod
//! html templating library for the web server
//! should be compatible also with svg, because of namespaces

// region: use
use crate::*;
use reader_for_microxml::*;

use std::fs;
use unwrap::unwrap;
// endregion: use

#[derive(Clone, Debug)]
pub enum Node {
    /// A text node. The text must be not encoded.
    /// It will be xml encoded when converting the node to html string.
    Text(String),
    /// An element potentially with attributes and children.
    Element(ElementNode),
    /// comment. . The text must be not encoded.
    /// It will be xml encoded when converting the node to html string.
    Comment(String),
}
#[derive(Clone, Debug, Default)]
pub struct ElementNode {
    pub tag_name: String,
    pub attributes: Vec<Attribute>,
    pub children: Vec<Node>,
    pub namespace: Option<String>,
    pub is_self_closing: bool,
}
/// An attribute on a DOM node, such as `id="my-thing"`
#[derive(Clone, Debug)]
pub struct Attribute {
    pub name: String,
    /// attribute value. The text must be not encoded.
    /// It will be xml encoded when converting the node to html string.
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
    /// returns a String to replace the next text-node or attribute value
    /// use macro s!() for a normal string
    fn replace_with_string(
        &self,
        placeholder: &str,
        subtemplate: &str,
        pos_cursor: usize,
    ) -> String;
    /// same as replace_with_string, but return url
    /// exclusively for attributes value of href and src
    /// the url must be encoded in the beginning because it encodes segments of
    /// url prior to being composed together.
    /// use macro url_u!() to create an url, very like format!
    /// I try to avoid String here to force the developer to not forget to url_encode
    fn replace_with_url(
        &self,
        placeholder: &str,
        subtemplate: &str,
        pos_cursor: usize,
    ) -> UrlUtf8EncodedString;
    /// boolean : is the next node rendered or not
    fn retain_next_node_or_attribute(&self, placeholder: &str) -> bool;
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
        //dbg!(&template_file_name);
        let mut template_raw = unwrap!(fs::read_to_string(&template_file_name));
        // find node <html >, jump over <!DOCTYPE html> because it is not microXml compatible
        // I will add <!DOCTYPE html> when the rendering ends, before returning the html.
        if let Some(pos_html) = template_raw.find("<html") {
            template_raw.drain(..pos_html);
        }

        self.render(&template_raw)
    }
    /// render for root template (not subtemplates) from string
    fn render(&self, html_template_raw: &str) -> String {
        let nodes =
            unwrap!(self.render_template_raw_to_nodes(&html_template_raw, HtmlOrSvg::Html, "", 0));
        // because this is the root template it must return one ElementNode
        let mut html = s!();
        match &nodes[0] {
            Node::Element(temp_element_node) => {
                html = unwrap!(Self::root_element_node_to_html_string(temp_element_node));
            }
            _ => {
                eprintln!("Error: render_template_raw_to_nodes() does not return one ElementNode.")
            }
        }
        // return
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
    ) -> Result<Vec<Node>, &'static str> {
        // html_template_raw can be a fragment. I add the root, that will later be removed.
        let html_template_raw = &format!("<template>{}</template>", html_template_raw);
        // extract sub_templates. Only one level deep.
        let sub_templates = Self::extract_children_sub_templates(html_template_raw);
        // the index zero is the drained main template
        let mut reader_for_microxml = ReaderForMicroXml::new(&sub_templates[0].template);
        let mut dom_path: Vec<String> = Vec::new();
        let mut root_element;
        let mut html_or_svg_local = html_or_svg_parent;

        // the root element must be only one
        if let Some(result_token) = reader_for_microxml.next() {
            match result_token {
                Ok(token) => {
                    match token {
                        Token::StartElement(tag_name) => {
                            dom_path.push(s!(tag_name));
                            root_element = ElementNode {
                                tag_name: s!(tag_name),
                                ..Default::default()
                            };
                            if &tag_name == &"svg" {
                                html_or_svg_local = HtmlOrSvg::Svg;
                            }
                            if let HtmlOrSvg::Svg = html_or_svg_local {
                                // svg elements have this namespace
                                root_element.namespace = Some(s!("http://www.w3.org/2000/svg"));
                            }
                            // recursive function can return error

                            match unwrap!(self.fill_element_node(
                                &mut reader_for_microxml,
                                root_element,
                                html_or_svg_local,
                                &mut dom_path,
                                &sub_templates,
                                subtemplate,
                                pos_cursor,
                                // retain_next_node_or_attribute:
                                true,
                            )) {
                                Ok(new_root_element) => root_element = new_root_element,
                                Err(err) => {
                                    return Err(&err);
                                }
                            }
                        }
                        _ => {
                            // return error
                            return Err("Error: no root element");
                        }
                    }
                }
                Err(err_msg) => return Err(err_msg),
            }
        } else {
            return Err("Error: Not found root element.");
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
    ) -> Option<Result<ElementNode, &'static str>> {
        let mut replace_string: Option<String> = None;
        let mut replace_attr_name: Option<String> = None;
        let mut replace_attr_repl_name: Option<String> = None;
        let mut replace_url: Option<UrlUtf8EncodedString> = None;
        let mut replace_vec_nodes: Option<Vec<Node>> = None;
        let mut retain_next_node_or_attribute = retain_this_node;
        let mut html_or_svg_local;
        // loop through all the siblings in this iteration
        while let Some(result_token) = reader_for_microxml.next() {
            // the children inherits html_or_svg from the parent, but cannot change the parent
            html_or_svg_local = html_or_svg_parent;
            match result_token {
                Ok(token) => {
                    match token {
                        Token::StartElement(tag_name) => {
                            dom_path.push(s!(tag_name));
                            // println!("dom_path: {:?}",dom_path);
                            // construct a child element and fill it (recursive)
                            let mut child_element = ElementNode {
                                tag_name: s!(tag_name),
                                ..Default::default()
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
                            child_element = unwrap!(unwrap!(self.fill_element_node(
                                reader_for_microxml,
                                child_element,
                                html_or_svg_local,
                                dom_path,
                                sub_templates,
                                subtemplate,
                                pos_cursor,
                                retain_next_node_or_attribute,
                            )));

                            // ignore this node dynamic content, and don't push to result
                            // but traverse all template nodes.
                            if retain_next_node_or_attribute == true {
                                if let Some(repl_vec_nodes) = replace_vec_nodes {
                                    for repl_node in repl_vec_nodes {
                                        element.children.push(repl_node);
                                    }
                                    replace_vec_nodes = None;
                                } else {
                                    element.children.push(Node::Element(child_element));
                                }
                            }
                            // the siblings get the parents retain, until sb_
                            retain_next_node_or_attribute = retain_this_node;
                        }
                        Token::Attribute(name, value) => {
                            if retain_this_node == true {
                                if name.starts_with("data-st_") {
                                    // placeholder is in the attribute name.
                                    // the attribute value is only informative what is the next attribute name
                                    // example: data-st_placeholder="href" href="x"
                                    // The replace_string will always be applied to the next attribute. No matter the name.
                                    let placeholder = name.trim_start_matches("data-");
                                    let repl_txt = self.replace_with_string(
                                        placeholder,
                                        subtemplate,
                                        pos_cursor,
                                    );
                                    replace_attr_name = Some(s!(value));
                                    replace_attr_repl_name = Some(s!(name));
                                    replace_string = Some(repl_txt);
                                } else if name.starts_with("data-su_") {
                                    // the same as data-st_, but exclusive to href and src
                                    // because they must use an url encoded string
                                    let placeholder = name.trim_start_matches("data-");
                                    let repl_url =
                                        self.replace_with_url(placeholder, subtemplate, pos_cursor);
                                    replace_attr_name = Some(s!(value));
                                    replace_attr_repl_name = Some(s!(name));
                                    replace_url = Some(repl_url);
                                } else if name.starts_with("data-sb-") {
                                    // the next attribute existence
                                    // if false it will not be rendered
                                    let placeholder = &value;
                                    let repl_bool = self.retain_next_node_or_attribute(placeholder);
                                    retain_next_node_or_attribute = repl_bool;
                                } else if retain_next_node_or_attribute == false {
                                    // don't push the next attribute
                                    // usable for radio buttons checked attribute
                                    // a terrible html design choice
                                    retain_next_node_or_attribute = true;
                                } else {
                                    // add attribute to Node
                                    if let Some(repl) = replace_string {
                                        if name != &unwrap!(replace_attr_name) {
                                            panic!("Error: Attr value of {} is not equal the next attr name {} data-model:{} dom_path: {:?} ", 
                                            unwrap!(replace_attr_repl_name), name,  self.data_model_name(), dom_path);
                                        // replace_attr_name = None;
                                        // replace_attr_repl_name=None;
                                        } else {
                                            // exclusively href and src must contain url
                                            if name == "href" || name == "src" {
                                                // error it is NOT encoded
                                                panic!("Error: Repl of  {} name {} is NOT created as url, but as string: {}  data-model:{} dom_path: {:?}", 
                                                unwrap!(replace_attr_repl_name), name, repl, self.data_model_name(), dom_path);
                                            } else {
                                                element.attributes.push(Attribute {
                                                    name: s!(name),
                                                    value: repl,
                                                });
                                            }
                                            // empty the replace_string for the next node
                                            replace_string = None;
                                            replace_attr_name = None;
                                            replace_attr_repl_name = None;
                                        }
                                    } else if let Some(repl) = replace_url {
                                        if name != unwrap!(replace_attr_name.as_ref()) {
                                            panic!("Error: Attr value of {} is not equal the next attr name {} data-model:{} dom_path: {:?} ", 
                                             unwrap!(replace_attr_repl_name), name, self.data_model_name(), dom_path);
                                        // replace_attr_name = None;
                                        // replace_attr_repl_name = None;
                                        } else {
                                            // this is dynamic content. Must be already url encoded
                                            // from the source for "href" and "src" only.
                                            if name == "href" || name == "src" {
                                                element.attributes.push(Attribute {
                                                    name: s!(name),
                                                    value: repl.to_string(),
                                                });
                                            } else {
                                                //error. it is encoded for other attributes
                                                panic!("Repl of {} name {} is mistakenly url encoded: {} data-model:{} dom_path: {:?}", 
                                            unwrap!(replace_attr_repl_name), name, repl.to_string(), self.data_model_name(), dom_path);
                                            }
                                        }
                                        // empty the replace_string for the next node
                                        replace_url = None;
                                        replace_attr_name = None;
                                        replace_attr_repl_name = None;
                                    } else {
                                        // Value is coming from the template that must be well-formed.
                                        // It means that is html-encoded and we must decode it
                                        // to push it to Node where all the strings are NOT html-encoded.
                                        element.attributes.push(Attribute {
                                            name: s!(name),
                                            value: decode_5_xml_control_characters(value),
                                        });
                                    }
                                }
                            }
                        }
                        Token::TextNode(txt) => {
                            if retain_this_node == true {
                                if let Some(repl) = replace_string {
                                    // empty the replace_string for the next Text node
                                    replace_string = None;
                                    element.children.push(Node::Text(repl));
                                } else if let Some(repl) = replace_url {
                                    // empty the replace_string for the next Text node
                                    replace_url = None;
                                    element.children.push(Node::Text(repl.to_string()));
                                } else {
                                    // The template is well-formed.
                                    // The string is html-encoded and must be html-decoded
                                    // to push it to Node, where strings are "normal".
                                    // dbg!(&dom_path);
                                    // The <script> node is the exception with other rules for encoding
                                    if unwrap!(dom_path.last()) == "script" {
                                        let txt = decode_html_script_node(txt);
                                        element.children.push(Node::Text(txt));
                                    } else {
                                        let txt = decode_5_xml_control_characters(txt);
                                        element.children.push(Node::Text(txt));
                                    }
                                };
                            }
                        }
                        Token::Comment(txt) => {
                            if retain_this_node == true {
                                // the main goal of comments is to change the value of the next text node
                                // with the result of a function
                                // it must look like <!--st_get_text-->
                                // one small exception is <textarea> because it ignores the comment syntax.
                                // It is still working, and it is not very ugly.
                                if txt.starts_with("st_") {
                                    let repl_txt =
                                        self.replace_with_string(txt, subtemplate, pos_cursor);
                                    replace_string = Some(repl_txt);
                                } else if txt.starts_with("su_") {
                                    let repl_url =
                                        self.replace_with_url(txt, subtemplate, pos_cursor);
                                    replace_url = Some(repl_url);
                                } else if txt.starts_with("sb_") {
                                    // boolean if this is true than render the next node, else don't render
                                    retain_next_node_or_attribute =
                                        self.retain_next_node_or_attribute(txt);
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
                        Token::EndElement(name) => {
                            let last_name = unwrap!(dom_path.pop());
                            // it can be also auto-closing element
                            if last_name == name || name == "" {
                                if name == "" {
                                    element.is_self_closing = true;
                                }
                                return Some(Ok(element));
                            } else {
                                return Some(Err("End element not correct: "));
                            }
                        }
                    }
                }
                Err(err_msg) => return Some(Err(err_msg)),
            }
        }
        //return
        None
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
    /// the attribute values and Text nodes and Comments are xml encoded
    fn root_element_node_to_html_string(element_node: &ElementNode) -> Result<String, String> {
        let mut dom_path: Vec<String> = Vec::new();
        /// recursive private fn sub element to html
        fn sub_element_node_mut_html(
            html: &mut String,
            element_node: &ElementNode,
            dom_path: &mut Vec<String>,
        ) {
            html.push_str("<");
            html.push_str(&element_node.tag_name);
            html.push_str(" ");
            dom_path.push(element_node.tag_name.to_string());
            for attr in &element_node.attributes {
                html.push_str(&attr.name);
                html.push_str(" = \"");
                html.push_str(&encode_5_xml_control_characters(&attr.value));
                html.push_str("\" ");
            }
            if element_node.is_self_closing == true {
                // auto-closing element
                // for <br /> is significant to stay auto-closed
                // because <br></br> is rendered differently
                html.push_str("/>");
                unwrap!(dom_path.pop());
            // dbg!(&html);
            } else {
                html.push_str(">");
                for sub_elem in &element_node.children {
                    match &sub_elem {
                        Node::Element(sub_element) => {
                            // recursion
                            sub_element_node_mut_html(html, sub_element, dom_path);
                        }
                        Node::Text(text) => {
                            if unwrap!(dom_path.last()) == "script" {
                                // in html script elements are encoded differently
                                html.push_str(&encode_html_script_node(&text));
                            } else {
                                html.push_str(&encode_5_xml_control_characters(&text));
                            }
                        }
                        Node::Comment(text) => html.push_str(&format!(
                            "<!--{}-->",
                            encode_5_xml_control_characters(&text)
                        )),
                    }
                }
                // end tag
                html.push_str("</");
                html.push_str(&element_node.tag_name);
                html.push_str(">");
                unwrap!(dom_path.pop());
            }
        }

        let mut html = String::with_capacity(5000);
        html.push_str("<!DOCTYPE html>");
        sub_element_node_mut_html(&mut html, element_node, &mut dom_path);
        // return
        Ok(html)
    }
}
// region: utility fn

/// in html the <script> element is encoded differently
pub fn encode_html_script_node(input: &str) -> String {
    input.replace("</script>", "\\x3c/script>")
}

/// in html the <script> element is decoded differently
pub fn decode_html_script_node(input: &str) -> String {
    input.replace("\\x3c/script>", "</script>")
}

/// private fn - decode 5 xml control characters : " ' & < >
/// <https://www.liquid-technologies.com/XML/EscapingData.aspx>
/// I will ignore all html entities, to keep things simple,
/// because all others characters can be written as utf-8 characters.
/// it is mandatory that text is valid utf-8.
/// <https://www.tutorialspoint.com/html5/html5_entities.htm>
/// TODO: find a faster method // The standard library replace() function makes allocation,
fn decode_5_xml_control_characters(input: &str) -> String {
    input
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
}

/// TODO: find a faster method // The standard library replace() function makes allocation,
/// Just to talk about XSS attack on attribute value.
/// let name = "dummy onmouseover=alert(/XSS/)";    // User input
/// let tag = format!("<option value={}>", htmlescape::encode_minimal(name));
/// // Here `tag` is    "<option value=dummy onmouseover=alert(/XSS/)>"
/// I use templates that must be microxml compatible.
/// There cannot exist an attribute value without quotes.
fn encode_5_xml_control_characters(input: &str) -> String {
    input
        .replace("&", "&amp;")
        .replace("\"", "&quot;")
        .replace("'", "&apos;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
}

/// boilerplate
pub fn retain_next_node_or_attribute_match_else(data_model_name: &str, placeholder: &str) -> bool {
    eprintln!(
        "Error: Unrecognized {} retain_next_node_or_attribute: \"{}\"",
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
    s!(err_msg)
}
/// boilerplate
pub fn replace_with_url_match_else(
    data_model_name: &str,
    placeholder: &str,
) -> UrlUtf8EncodedString {
    let err_msg = format!(
        "Error: Unrecognized {} replace_with_url: \"{}\"",
        data_model_name, placeholder
    );
    eprintln!("{}", &err_msg);
    url_u!(&err_msg, "")
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
        children: vec![Node::Text(err_msg)],
        ..Default::default()
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
        children: vec![Node::Text(err_msg)],
        ..Default::default()
    });
    return vec![node];
}
/// to string, but zero converts to empty
pub fn url_s_zero_to_empty(number: usize) -> String {
    if number == 0 {
        s!()
    } else {
        s!(number)
    }
}
// endregion: utility fn
