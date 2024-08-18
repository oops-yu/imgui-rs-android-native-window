use imgui::{FontId, StyleVar, Ui};

use super::data::GameData;
use super::data_types::*;

static WHITE_OUTER: imgui::ImColor32 = imgui::ImColor32::from_rgba(255, 255, 255, 191);
static WHITE_INNER: imgui::ImColor32 = imgui::ImColor32::from_rgba(255, 255, 255, 12);
static GREEN_OUTER: imgui::ImColor32 = imgui::ImColor32::from_rgba(0, 255, 0, 191);
static GREEN_INNER: imgui::ImColor32 = imgui::ImColor32::from_rgba(0, 255, 0, 12);
static YELLOW: imgui::ImColor32 = imgui::ImColor32::from_rgba(255, 255, 0, 191);

pub fn esp(ui: &mut Ui, game_data: &mut GameData) {
    let draw_list = ui.get_background_draw_list();

    for player in &game_data.players {
        if player.is_in_screen() {
            let font_scale: f32 = 0.8;

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

            let bottom = player.ground_contact.position_on_screen.y + width / 10.0;
            if player.is_bot {
                draw_list
                    .add_rect([left, top], [right, bottom], WHITE_OUTER)
                    .thickness(2.0)
                    .filled(false)
                    .build();
                draw_list
                    .add_rect([left, top], [right, bottom], WHITE_INNER)
                    .thickness(2.0)
                    .filled(true)
                    .build();
            } else {
                draw_list
                    .add_rect([left, top], [right, bottom], GREEN_OUTER)
                    .thickness(2.0)
                    .filled(false)
                    .build();
                draw_list
                    .add_rect([left, top], [right, bottom], GREEN_INNER)
                    .thickness(2.0)
                    .filled(true)
                    .build();
            }

            //血量
            if player.health_percentage != 1.0 {
                draw_list
                    .add_line(
                        [right + 3.0, bottom],
                        [
                            right + 3.0,
                            (top + (bottom - top) * (1.0 - player.health_percentage)),
                        ],
                        [1.0, 0.0, 0.0],
                    )
                    .thickness(2.0)
                    .build();
            }

            //队伍+距离
            let distance = format!("[{}]{:.0}m",player.team_id, player.distance_to_player);
            let mut distance_text_size = ui.calc_text_size(&distance);

            distance_text_size[0] *= font_scale;
            distance_text_size[1] *= font_scale;
            draw_list.add_text_with_font_size(
                [
                    head.position_on_screen.x - (distance_text_size[0] / 2.0),
                    top - distance_text_size[1],
                ],
                WHITE_OUTER,
                distance,
                distance_text_size[1],
            );
            top -= distance_text_size[1];
            let name = if player.is_bot {
                "BOT"
            } else {
                player.get_name()
            };
            let mut name_text_size = ui.calc_text_size(&name);

            name_text_size[0] *= font_scale;
            name_text_size[1] *= font_scale;
            draw_list.add_text_with_font_size(
                [
                    head.position_on_screen.x - (name_text_size[0] / 2.0),
                    top - name_text_size[1],
                ],
                YELLOW,
                name,
                name_text_size[1],
            );
            //射线
            draw_list
                .add_line(
                    [1200.0, 0.0],
                    [head.position_on_screen.x, top - name_text_size[1]],
                    WHITE_OUTER,
                )
                .thickness(2.0)
                .build();
        }
    }
}
