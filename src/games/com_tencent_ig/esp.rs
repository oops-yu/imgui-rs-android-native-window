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
        }
    }
}
