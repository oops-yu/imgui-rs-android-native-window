use imgui::{FontId, StyleVar, Ui};

use super::data::GameData;
use super::data_types::*;

pub fn esp(ui: &mut Ui, game_data: &mut GameData) {
    let draw_list = ui.get_background_draw_list();
    
    for player in &game_data.players {
        if player.is_in_screen() {
            let font_scale:f32 = 0.8;
            let white = [1.0, 1.0, 1.0,0.74];
            let yellow = [1.0,1.0,0.0,0.74];
            let Player {
                width,
                head,
                chest,
                pelvis,
                left_shoulder,
                right_shoulder,
                left_elbow,
                right_elbow,
                left_wrist,
                right_wrist,
                left_thigh,
                right_thigh,
                left_knee,
                right_knee,
                left_ankle,
                right_ankle,
                ..
            } = player;
            #[cfg(feature = "debug_bones")]
            {
                for i in &player.bone_debug {
                    let pos = i.position_on_screen.to_pos();
                    let col = [1.0, 1.0, 1.0];

                    draw_list.add_text(pos, col, i.name_for_debug.clone());
                    draw_list
                        .add_circle(pos, 10.0, col)
                        .filled(true)
                        .thickness(5.0)
                        .build();
                }
            }
            //框
            let left = head.position_on_screen.x - width * 0.8;
            let right = head.position_on_screen.x + width * 0.8;
            let mut top = head.position_on_screen.y - width / 3.0;
            let bottom_ankle = if left_ankle.position_on_screen.y > right_ankle.position_on_screen.y
            {
                left_ankle.position_on_screen.y
            } else {
                right_ankle.position_on_screen.y
            };

            let bottom = bottom_ankle + width / 10.0;
            draw_list
                .add_rect([left, top], [right, bottom], white)
                .thickness(2.0)
                .build();
            //血量
            if player.health_percentage != 1.0{
                draw_list.add_line([right+3.0,bottom],[right+3.0,(top+(bottom-top)*(1.0-player.health_percentage))],[1.0,0.0,0.0]).thickness(2.0).build();
            }
            
            
            //距离
            let distance = format!("{:.0}m",player.distance_to_player);
            let mut distance_text_size = ui.calc_text_size(&distance);


            distance_text_size[0]*=font_scale;
            distance_text_size[1]*=font_scale;
            draw_list.add_text_with_font_size(
                [
                    head.position_on_screen.x - (distance_text_size[0] / 2.0),
                    top - distance_text_size[1],
                ],
                white,
                distance,
                distance_text_size[1]
            );
            top -=distance_text_size[1];
            let name = if player.is_bot {
                "BOT"
            } else {
                player.get_name()
            };
            let mut name_text_size = ui.calc_text_size(&name);
            
            name_text_size[0]*=font_scale;
            name_text_size[1]*=font_scale;
            draw_list.add_text_with_font_size(
                [
                    head.position_on_screen.x - (name_text_size[0] / 2.0),
                    top - name_text_size[1],
                ],
                yellow,
                name,
                name_text_size[1]
            );
            //射线
            draw_list.add_line([1200.0,0.0], [
                head.position_on_screen.x,
                top - name_text_size[1],
            ], white).thickness(2.0).build();
        }
    }
}
