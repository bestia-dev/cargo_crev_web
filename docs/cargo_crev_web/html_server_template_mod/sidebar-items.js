initSidebarItems({"enum":[["HtmlOrSvg","Svg elements are different because they have a namespace"],["Node",""]],"fn":[["decode_5_xml_control_characters","private fn - decode 5 xml control characters : \" ' & < > https://www.liquid-technologies.com/XML/EscapingData.aspx I will ignore all html entities, to keep things simple, because all others characters can be written as utf-8 characters. it is mandatory that text is valid utf-8. https://www.tutorialspoint.com/html5/html5_entities.htm TODO: find a faster method // The standard library replace() function makes allocation,"],["decode_html_script_node","in html the  element is decoded differently"],["encode_5_xml_control_characters","TODO: find a faster method // The standard library replace() function makes allocation, Just to talk about XSS attack on attribute value. let name = \"dummy onmouseover=alert(/XSS/)\";    // User input let tag = format!(\"\", htmlescape::encode_minimal(name)); // Here `tag` is    \"\" I use templates that must be microxml compatible. There cannot exist an attribute value without quotes."],["encode_html_script_node","in html the  element is encoded differently"],["render_sub_template_match_else","boilerplate"],["replace_with_nodes_match_else","boilerplate"],["replace_with_string_match_else","boilerplate"],["replace_with_url_match_else","boilerplate"],["retain_next_node_or_attribute_match_else","boilerplate"],["url_s_zero_to_empty","to string, but zero converts to empty"]],"struct":[["Attribute","An attribute on a DOM node, such as `id=\"my-thing\"`"],["ElementNode",""],["SubTemplate",""]],"trait":[["HtmlServerTemplateRender",""]]});