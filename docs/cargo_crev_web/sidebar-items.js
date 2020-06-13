initSidebarItems({"enum":[["HtmlOrSvg","Svg elements are different because they have a namespace"],["Node",""]],"fn":[["author_name_from_url",""],["conditional_usize","similar to ternary operator"],["decode_html_script_node","in html the  element is decoded differently"],["encode_html_script_node","in html the  element is encoded differently"],["find_from","find str from pos_cursor low level"],["find_pos_after_delimiter","return the position after the delimiter or None Does NOT mutate the pos_cursor, because that is for a higher level logic to decide."],["find_pos_before_delimiter","return the position before the delimiter or None Does NOT mutate the pos_cursor, because that is for a higher level logic to decide."],["find_range_between_delimiters","find and return the range of the first occurrence between start and end delimiters Success: mutates also the cursor position, so the next find will continue from there Fail: return None if not found and don't mutate pos_cursor I use type Range to avoid references &str and lifetimes. But the programmer can make the error to apply the range to the wrong vector."],["main","main function of the binary"],["main_code",""],["ns_elapsed",""],["ns_print",""],["ns_start",""],["parse_semver","parse semver ex. 12.99.88alpha"],["render_sub_template_match_else","boilerplate"],["replace_with_nodes_match_else","boilerplate"],["replace_with_string_match_else","boilerplate"],["replace_with_url_match_else","boilerplate"],["retain_next_node_or_attribute_match_else","boilerplate"],["traverse_dir_with_exclude_dir","traverse dir (sub-dir) with exclude dir the find_file and the exclude dir strings must start with /"],["url_s_zero_to_empty","to string, but zero converts to empty"],["version_for_sorting","version for sorting"]],"macro":[["s","short macro `s!` for &str.to_string or format!(). because that is so common. Equivalents: String::new(), x.to_string(), x.to_owned(),..."],["url_u","returns UrlUtf8EncodedString::new_x Constructor macro for UrlUtf8EncodedString The attribute [macro_export] \"moves\" the macro in the main module. Macros cannot be inside impl like fn. The module names must be added to the code to work properly. TODO: use macro repetition to avoid having 4 different fn."]],"mod":[["author_reviews_mod","author_reviews_mod"],["authors_mod","authors_mod"],["badge_mod","badge_mod"],["cargo_registry_index_mod",""],["crate_reviews_mod","crate_reviews_mod"],["crate_version_summary_mod","crate_version_summary_mod"],["crates_mod","crates_mod"],["data_file_scan_mod","data_file_scan_mod"],["html_server_template_mod","html_server_template_mod html templating library for the web server should be compatible also with svg, because of namespaces"],["issue_mod","issue_mod"],["macros_mod",""],["main_code_mod","main.rs"],["reserved_folder_mod","reserved_folder_mod This is only one module/html page, but can execute different actions. The data model must have fields for every action as Option<>. Because only this data can influence the html render. There are different \"new\" functions for different actions, to prepare adequate data. If field is is_some(), then render the html part dedicated to this action."],["review_index_mod","review_index_mod"],["review_index_summary_mod","review_index_summary_mod"],["review_mod","review_mod"],["review_new_mod","review_mod"],["router_mod","router_mod"],["state_mod",""],["url_utf8_mod","url_utf8_mod url encoding and decoding on the web server This module is strictly limited to utf8 urls.\\ Url is made of parts, fragments or segments mostly delimited by slash \"/\".\\ They must be separately encoded/decoded, not as a whole string.\\ It is impossible to guarantee that the whole string is correctly encoded/decoded.\\ But is possible to minimize the misuse of the String type for Url.\\ With the normal String it is not possible to force the developer to encode/decode.\\ With special wrapper types around String is possible to help the coder to write properly and not forget about it.\\ TODO: analyze if is possible to use more &str and Cow instead of always allocating String.\\ But urls are usually small and this is not a priority."],["utils_mod","utils_mod.rs"],["version_summary_mod","crate_version_summary_mod"]],"struct":[["Attribute","An attribute on a DOM node, such as `id=\"my-thing\"`"],["ElementNode",""],["SubTemplate",""],["UrlPartUtf8Decoded","the url must be utf 8. Only the 5 control characters are encoded. url has parts or fragments or segments delimited mostly by slash / every part must be encoded/decoded separately, to maintain the control character slash /"],["UrlUtf8EncodedString","Type UrlUtf8EncodedString explicitly informs that the content has been url encoded. It contains a string with the whole url. The url is constructed with a special macro, where the dynamic parts are always encoded. It is impossible to force the developer to properly encode the static part of the url. But this special type is making this kind of errors difficult, obvious and traceable. TODO: the macro could use repetition to avoid having 4 fn with different number of parameters."]],"trait":[["HtmlServerTemplateRender",""]],"type":[["ArcMutStateGlobal",""]]});