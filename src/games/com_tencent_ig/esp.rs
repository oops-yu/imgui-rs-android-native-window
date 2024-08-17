use imgui::Ui;

use super::data::GameData;
use super::data_types::*;

pub fn esp(ui: &mut Ui, game_data: &mut GameData) {
    let draw_list = ui.get_background_draw_list();
    let col = [1.0, 1.0, 1.0];
    for player in &game_data.players {
        if player.is_in_screen() {
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
            // for searching bones
            // for i in &player.bone_debug {
            //     let pos = i.position_on_screen.to_pos();
            //     let col = [1.0, 1.0, 1.0];

            //     draw_list.add_text(pos, col, i.name_for_debug.clone());
            //     draw_list
            //         .add_circle(pos, 10.0, col)
            //         .filled(true)
            //         .thickness(5.0)
            //         .build();
            // }
            let left = head.position_on_screen.x - width * 0.6;
            let right = head.position_on_screen.x + width * 0.6;
            let top = head.position_on_screen.y - width / 5.0;
            let bottom = right_ankle.position_on_screen.y + width / 10.0;
            draw_list
                .add_rect([left, top], [right, bottom], col)
                .thickness(2.0)
                .build();
            let name = if player.is_bot {
                format!("bot {}", player.max_health)
            } else {
                format!("{} {}", player.get_name(), player.max_health)
            };
            let text_size = ui.calc_text_size(&name);
            draw_list.add_text(
                [
                    head.position_on_screen.x - (text_size[0] / 2.0),
                    top - text_size[1],
                ],
                [1.0, 1.0, 1.0],
                name,
            );
            //绘制骨骼
            //绘制点
            // let radius = 3.0;
            // let color = col;
            // draw_list.add_circle(head.position_on_screen.to_pos(), radius, color).filled(true).build();
            // draw_list.add_circle(chest.position_on_screen.to_pos(), radius, color).filled(true).build();
            // draw_list.add_circle(pelvis.position_on_screen.to_pos(), radius, color).filled(true).build();
            // draw_list.add_circle(left_shoulder.position_on_screen.to_pos(), radius, color).filled(true).build();
            // draw_list.add_circle(right_shoulder.position_on_screen.to_pos(), radius, color).filled(true).build();
            // draw_list.add_circle(left_elbow.position_on_screen.to_pos(), radius, color).filled(true).build();
            // draw_list.add_circle(right_elbow.position_on_screen.to_pos(), radius, color).filled(true).build();
            // draw_list.add_circle(left_wrist.position_on_screen.to_pos(), radius, color).filled(true).build();
            // draw_list.add_circle(right_wrist.position_on_screen.to_pos(), radius, color).filled(true).build();
            // draw_list.add_circle(left_thigh.position_on_screen.to_pos(), radius, color).filled(true).build();
            // draw_list.add_circle(right_thigh.position_on_screen.to_pos(), radius, color).filled(true).build();
            // draw_list.add_circle(left_knee.position_on_screen.to_pos(), radius, color).filled(true).build();
            // draw_list.add_circle(right_knee.position_on_screen.to_pos(), radius, color).filled(true).build();
            // draw_list.add_circle(left_ankle.position_on_screen.to_pos(), radius, color).filled(true).build();
            // draw_list.add_circle(right_ankle.position_on_screen.to_pos(), radius, color).filled(true).build();
            //绘制骨骼
            // let c = col;
            // //chest -> left_shoulder
            // draw_list.add_line(chest.position_on_screen.to_pos(), left_shoulder.position_on_screen.to_pos(), c).thickness(2.0).build();
            // //chest -> right_shoulder
            // draw_list.add_line(chest.position_on_screen.to_pos(), right_shoulder.position_on_screen.to_pos(), c).thickness(2.0).build();
            // //left_shoulder->left_elbow
            // draw_list.add_line(left_shoulder.position_on_screen.to_pos(), left_elbow.position_on_screen.to_pos(), c).thickness(2.0).build();
            // //left_elbow->left_wrist
            // draw_list.add_line(left_elbow.position_on_screen.to_pos(), left_wrist.position_on_screen.to_pos(), c).thickness(2.0).build();
            // //right_shoulder->right_elbow
            // draw_list.add_line(right_shoulder.position_on_screen.to_pos(), right_elbow.position_on_screen.to_pos(), c).thickness(2.0).build();
            // //right_elbow->right_wrist
            // draw_list.add_line(right_elbow.position_on_screen.to_pos(), right_wrist.position_on_screen.to_pos(), c).thickness(2.0).build();
            // //chest -> pelvis
            // draw_list.add_line(chest.position_on_screen.to_pos(), pelvis.position_on_screen.to_pos(), c).thickness(2.0).build();
            // //pelvis -> left_thigh
            // draw_list.add_line(pelvis.position_on_screen.to_pos(), left_thigh.position_on_screen.to_pos(), c).thickness(2.0).build();
            // //pelvis -> right_thigh
            // draw_list.add_line(pelvis.position_on_screen.to_pos(), right_thigh.position_on_screen.to_pos(), c).thickness(2.0).build();
            // //left_thigh -> left_knee
            // draw_list.add_line(left_thigh.position_on_screen.to_pos(), left_knee.position_on_screen.to_pos(), c).thickness(2.0).build();
            // //left_knee -> left_ankle
            // draw_list.add_line(left_knee.position_on_screen.to_pos(), left_ankle.position_on_screen.to_pos(), c).thickness(2.0).build();
            // //right_thigh -> right_knee
            // draw_list.add_line(right_thigh.position_on_screen.to_pos(), right_knee.position_on_screen.to_pos(), c).thickness(2.0).build();
            // //right_knee -> right_ankle
            // draw_list.add_line(right_knee.position_on_screen.to_pos(), right_ankle.position_on_screen.to_pos(), c).thickness(2.0).build();
        }
    }
}
