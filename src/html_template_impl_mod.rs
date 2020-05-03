//! html_template_impl_mod  

use crate::html_template_mod::HtmlTemplating;
use crate::*;

use unwrap::unwrap;

impl HtmlTemplating for crev_query_mod::Proof {
    /*
        /// html_templating boolean id the next node is rendered or not
        fn call_fn_boolean(&self, fn_name: &str) -> bool {
            // websysmod::debug_write(&format!("call_fn_boolean: {}", &fn_name));
            match fn_name {
                "is_first_player" => self. self.game_data.my_player_number == 1,

                _ => {
                    let x = format!("Error: Unrecognized call_fn_boolean: \"{}\"", fn_name);
                    websysmod::debug_write(&x);
                    true
                }
            }
        }

        /// html_templating functions that return a String
        #[allow(
            clippy::needless_return,
            clippy::integer_arithmetic,
            clippy::indexing_slicing
        )]
        fn call_fn_string(&self, fn_name: &str) -> String {
            // websysmod::debug_write(&format!("call_fn_string: {}", &fn_name));
            match fn_name {
                "my_nickname" => self.game_data.my_nickname.to_owned(),

                "blink_or_not_group_id" => blink_or_not_group_id(self),
                "my_ws_uid" => format!("{}", self.web_data.my_ws_uid),
                "receiver_ws_uid" => format!("{}", self.web_data.web_rtc_data.rtc_receiver_ws_uid),
                "players_count" => format!("{} ", self.game_data.players.len() - 1),
                "game_name" => self.game_data.game_name.to_string(),
                "group_id" => self.game_data.group_id.to_string(),
                "url_to_join" => format!("bestia.dev/mem6/#p03.{}", self.web_data.my_ws_uid),
                "cargo_pkg_version" => env!("CARGO_PKG_VERSION").to_string(),
                "debug_text" => websysmod::get_debug_text(),
                "game_status" => self.game_data.game_status.as_ref().to_string(),
                "my_player_number" => self.game_data.my_player_number.to_string(),
                "gameboard_btn" => {
                    // different class depend on status
                    "btn".to_owned()
                }
                "card_moniker_first" => {
                    return unwrap!(self.game_data.game_config.as_ref()).card_moniker
                        [self.game_data.get_1st_card().card_number]
                        .to_string();
                }
                "card_moniker_second" => {
                    return unwrap!(self.game_data.game_config.as_ref()).card_moniker
                        [self.game_data.get_2nd_card().card_number]
                        .to_string();
                }
                "my_points" => {
                    return format!("{} ", self.game_data.my_player().points,);
                }
                "player_turn_nickname" => {
                    //websysmod::debug_write("player_turn_nickname");
                    return self.game_data.player_turn_now().nickname.to_string();
                }
                "sounds_and_labels" => {
                    return if self.game_data.sounds_and_labels == true {
                        "sounds and labels ON".to_string()
                    } else {
                        "sounds and labels OFF".to_string()
                    };
                }
                _ => {
                    let x = format!("Error: Unrecognized call_fn_string: \"{}\"", fn_name);
                    websysmod::debug_write(&x);
                    x
                }
            }
        }

        /// return a closure for the listener.
        #[allow(clippy::too_many_lines, clippy::type_complexity)]
        fn call_fn_listener(
            &self,
            fn_name: String,
        ) -> Box<dyn Fn(&mut dyn RootRender, VdomWeak, Event) + 'static> {
            Box::new(move |root, vdom, event| {
                let fn_name = fn_name.clone();
                let fn_name = fn_name.as_str();
                let rrc = root.unwrap_mut::<RootRenderingComponent>();
                //websysmod::debug_write(&format!("call_fn_listener: {}", &fn_name));
                match fn_name {

                    "open_youtube" => {
                        // randomly choose a link from rrc.videos
                        let num = websysmod::get_random(0, rrc.game_data.videos.len());
                        #[allow(clippy::indexing_slicing)]
                        // cannot panic:the num is 0..video.len
                        websysmod::open_new_tab(&format!(
                            "https://www.youtube.com/watch?v={}",
                            rrc.game_data.videos[num]
                        ));
                    }
                    "open_menu" => {
                        websysmod::open_new_local_page_push_to_history("#p21");
                    }
                    "sounds_and_labels" => {
                        // toggle sound and label on/off
                        websysmod::debug_write(&format!("on click sounds and labels: {}", ""));
                        if rrc.game_data.sounds_and_labels == true {
                            rrc.game_data.sounds_and_labels = false;
                        } else {
                            rrc.game_data.sounds_and_labels = true;
                        }

                        vdom.schedule_render();
                    }
                    "back_to_game" => {
                        let h = unwrap!(websysmod::window().history());
                        let _x = h.back();
                    }
                    "open_instructions" => {
                        websysmod::open_new_tab("#p08");
                    }
                    "debug_log" => {
                        websysmod::open_new_tab("#p31");
                    }
                    "webrtc" => {
                        open_new_local_page("#p41");
                    }

                    "web_rtc_start" => {
                        rrc.web_data
                            .web_rtc_data
                            .web_rtc_start(vdom, unwrap!(rrc.web_data.websocket_data.ws.clone()));
                    }

                    "web_rtc_send_chat" => {
                        rrc.web_data.web_rtc_data.web_rtc_send_chat(vdom);
                    }
                    "start_a_group_onclick" => {
                        // entry point for the game
                        rrc.start_websocket(vdom);
                        open_new_local_page("#p02");
                    }

                    "join_a_group_onclick" => {
                        websysmod::open_new_local_page_push_to_history("#p03");
                    }
                    "choose_a_game_onclick" => {
                        open_new_local_page("#p05");
                    }

                    "game_type_right_onclick" => {
                        game_type_right_onclick(rrc, vdom);
                    }
                    "game_type_left_onclick" => {
                        game_type_left_onclick(rrc, vdom);
                    }
                    "join_group_on_click" => {
                        open_new_local_page("#p04");
                    }
                    "drink_end" => {
                        // send a msg to end drinking to all players

                        websysmod::debug_write(&format!("MsgDrinkEnd send{}", ""));

                        // if all the cards are permanently up, this is the end of the game
                        // websysmod::debug_write("if is_all_permanently(rrc)");

                        // end the drink page
                        open_new_local_page("#p11");
                    }



                    "hide_big_img" => {
                        hide_big_img();
                    }
                    _ => {
                        let x = format!("Error: Unrecognized call_fn_listener: \"{}\"", fn_name);
                        websysmod::debug_write(&x);
                    }
                }
            })
        }

        /// html_templating functions that return a Node
        #[allow(clippy::needless_return)]
        fn call_fn_node<'a>(&self, cx: &mut RenderContext<'a>, fn_name: &str) -> Node<'a> {
            let bump = cx.bump;
            // websysmod::debug_write(&format!("call_fn_node: {}", &fn_name));
            match fn_name {

                "svg_qrcode" => {
                    return svg_qrcode_to_node(self, cx);
                }
                _ => {
                    let node = ElementBuilder::new(bump, "h2")
                        .children([text(
                            bumpalo::format!(in bump,
                                "Error: Unrecognized call_fn_node: \"{}\"",
                                fn_name
                            )
                            .into_bump_str(),
                        )])
                        .finish();

                    return node;
                }
            }
        }

        /// html_templating functions that return a vector of Nodes
        #[allow(clippy::needless_return)]
        fn call_fn_vec_nodes<'a>(&self, cx: &mut RenderContext<'a>, fn_name: &str) -> Vec<Node<'a>> {
            let bump = cx.bump;
            // websysmod::debug_write(&format!("call_fn_node: {}", &fn_name));
            match fn_name {

                _ => {
                    let node = ElementBuilder::new(bump, "h2")
                        .children([text(
                            bumpalo::format!(in bump,
                                "Error: Unrecognized call_fn_node: \"{}\"",
                                fn_name
                            )
                            .into_bump_str(),
                        )])
                        .finish();

                    return vec![node];
                }
            }
        }
    */
}

/// fn open new local page with #
/// does not push to history
pub fn open_new_local_page(hash: &str) {
    // I want to put the first url in history.
    // These are opened from outside my app and I cannot manage that differently.
    // There are 2 of them:
    // 1. if the players starts without hash
    // 2. if the player scanned the qrcode and opened the p3 with group_id
    // For links opened inside the app, I can call the open with or without history.
    // For example for menu p21 I want to have a back button.
    /*
    let (_old_location_href, old_href_hash) = websysmod::get_url_and_hash();
    if old_href_hash.is_empty() || old_href_hash.starts_with("#p03.") {
        websysmod::open_new_local_page_push_to_history(hash)
    } else {
        let _x = websysmod::window().location().replace(hash);
    }
    */
}
/*
/// update html_template and extract and saves html_sub_templates
#[allow(clippy::integer_arithmetic)]
#[allow(clippy::indexing_slicing)]
pub fn update_html_template_and_sub_templates(
    rrc: &mut RootRenderingComponent,
    resp_body_text: &str,
) {
    // only the html inside the <body> </body>
    let mut tm = between_body_tag(&resp_body_text);
    // parse and save sub_templates <template name="xxx"></template>
    rrc.web_data.html_sub_templates.clear();
    loop {
        let mut exist_template = false;

        let pos1 = tm.find("<template ");
        let del2 = "</template>";
        let pos2 = tm.find(del2);
        if let Some(pos_start) = pos1 {
            if let Some(pos_end) = pos2 {
                exist_template = true;
                // drain - extract a substring and remove it from the original
                let sub1: String = tm.drain(pos_start..pos_end + del2.len()).collect();

                let del3 = "name=\"";
                let pos_name_start = unwrap!(sub1.find(del3));
                let sub2 = &sub1[pos_name_start + del3.len()..];
                //websysmod::debug_write(sub2);

                let pos_name_end = unwrap!(sub2.find('"'));
                let name = &sub2[0..pos_name_end];
                //websysmod::debug_write(name);

                let del5 = '>';
                let pos_name_end_tag = unwrap!(sub1.find(del5));
                let pos6 = unwrap!(sub1.find(del2));
                let sub_template = &sub1[pos_name_end_tag + 1..pos6];
                //websysmod::debug_write(sub_template);

                rrc.web_data
                    .html_sub_templates
                    .push((name.to_string(), sub_template.to_string()));
            }
        }
        if !exist_template {
            break;
        }
    }
    rrc.web_data.html_template = tm;
}
*/
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
