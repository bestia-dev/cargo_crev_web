initSidebarItems({"fn":[["conditional_usize","similar to ternary operator"],["find_from","find str from pos_cursor low level"],["find_pos_after_delimiter","return the position after the delimiter or None Does NOT mutate the pos_cursor, because that is for a higher level logic to decide."],["find_pos_before_delimiter","return the position before the delimiter or None Does NOT mutate the pos_cursor, because that is for a higher level logic to decide."],["find_range_between_delimiters","find and return the range of the first occurrence between start and end delimiters Success: mutates also the cursor position, so the next find will continue from there Fail: return None if not found and don’t mutate pos_cursor I use type Range to avoid references &str and lifetimes. But the programmer can make the error to apply the range to the wrong vector."],["ns_elapsed",""],["ns_print",""],["ns_start",""],["parse_next_number","parse next characters until is numeric or end"],["parse_semver","parse semver ex. 12.99.88alpha"],["reviewer_name_from_url",""],["traverse_dir_with_exclude_dir","traverse dir (sub-dir) with exclude dir the find_file and the exclude dir strings must start with /"],["version_for_sorting","version for sorting"]]});