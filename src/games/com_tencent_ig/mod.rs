static mut UE4: u64 = 0;
static mut OLDUWORLD: u64 = 0;
static mut OLDULEVEL: u64 = 0;
static mut OLDGNAME: u64 = 0;
mod data;
mod data_types;
mod offsets;
use crate::common::System;
use crate::memory_helper::GameMem;

pub fn run() {
    //0.init driver
    let mut game_mem = GameMem::new();
    game_mem.initialize_with_process_name("com.tencent.ig");
    unsafe {
        UE4 = game_mem.get_module_base("libUE4.so").unwrap();
    }
    //1.auth (no need to consider now)
    //1.5 load config(no need to consider now)
    //2.drawui(no need to consider now)

    //3.cheat features

    //4.loop 2,3
    let mut game_data = GameData::default();
    // let mut value = 0;
    // let choices = ["test test this is 1", "test test this is 2"];

    System::new("title")
        .unwrap()
        .run((), move |run, ui1, frame_rate| {
            ui(run, ui1, frame_rate, &mut game_data, &mut game_mem);
        })
        .expect("failed");
}
struct Config {}

use imgui::Condition;
use imgui::Ui;

#[allow(unused_imports)]
use data::*;
#[allow(unused_imports)]
use data_types::*;

pub fn get_data(game_mem: &mut GameMem, game_data: &mut GameData) {
    let ue4 = unsafe { UE4 };

    let uworld = game_mem.read_with_offsets::<u64>(ue4, offsets::UWORLD);
    let (mut gname, mut ulevel) = unsafe { (OLDGNAME, OLDULEVEL) };
    unsafe {
        if OLDUWORLD != uworld {
            gname = game_mem.read_with_offsets::<u64>(ue4, offsets::GNAME);
            ulevel = game_mem.read_with_offsets::<u64>(uworld, offsets::ULEVEL);

            game_data.non_player_set.clear();
            game_data.players_set.clear();

            OLDUWORLD = uworld;
            OLDGNAME = gname;
            OLDULEVEL = ulevel;
        }
    }

    let (actors_addr, actors_count) =
        game_mem.read_with_offsets::<(u64, i32)>(ulevel, offsets::OBJARR);
    // println!(
    //     "ue4:{ue4:#016x}\nuworld:{uworld:#016x}\ngname:{gname:#016x}\nulevel:{ulevel:#016x}\n"
    // );
    if actors_count <= 0 || actors_count > 2000 {
        return;
    }

    //read local player information
    game_mem.read_memory_with_offsets(ue4, &mut game_data.matrix, offsets::PROJECTIONMATRIX);
    game_data.local_player = game_mem.read_with_offsets(ue4, offsets::LOCALPALYER);
    game_mem.read_memory_with_offsets(
        game_data.local_player,
        &mut game_data.local_position,
        offsets::PLAYERPOSITION,
    );
    game_data.fov = game_mem.read_with_offsets(game_data.local_player, offsets::LOCALFOV);
    game_data.firing = game_mem.read_with_offsets(game_data.local_player, offsets::ISFIRING);
    game_data.aiming = game_mem.read_with_offsets(game_data.local_player, offsets::ISAIMING);
    game_data.local_team_id = game_mem.read_with_offsets(game_data.local_player, offsets::TEAMID);

    game_data.players.clear();
    //read player array
    // for i in 0..actors_count {
    //     game_data.actor_array[i as usize] = game_mem.read_with_offsets(actors_addr, &[(i*8) as u64])
    // }
    game_mem.read_memory_with_length_and_offsets(
        actors_addr,
        game_data.actor_array.as_mut_ptr() as _,
        actors_count as usize * 8,
        &[0],
    );
    for i in 0..actors_count {
        let current_actor = game_data.actor_array[i as usize];
        if game_data.non_player_set.contains(&current_actor) {
            continue;
        }
        if !game_data.players_set.contains(&current_actor) {
            let current_actor_type =
                game_mem.read_with_offsets::<f32>(current_actor, offsets::OBJTYPE);
            if current_actor_type != 479.5 {
                game_data.non_player_set.insert(current_actor);
                continue;
            }
            game_data.players_set.insert(current_actor);
        }

        //读取玩家信息
        let uk0x1b0 = game_mem.read_with_offsets::<u64>(current_actor, offsets::UK0X1B0);
        if uk0x1b0 <= 0xffff
            || uk0x1b0 == 0
            || uk0x1b0 <= 0x10000000
            || uk0x1b0 % 4 != 0
            || uk0x1b0 >= 0x10000000000
        {
            continue;
        }
        let uk0xf60 = game_mem.read_with_offsets::<i32>(current_actor, offsets::UK0XF60);
        if uk0xf60 == 262144 || uk0xf60 == 262152 {
            continue;
        }
        let mut current_player = Player::default();
        game_mem.read_memory_with_offsets(
            uk0x1b0,
            &mut current_player.world_position,
            offsets::UK0X1C0,
        );
        if !current_player.position_valid() {
            continue;
        }
        // current_player.team_id = game_mem.read_with_offsets(current_actor.clone(), offsets::TEAMID);
        // if current_player.team_id != game_data.local_team_id || current_player.team_id < 1 {
        //     continue;
        // }

        // //血量
        let (health, max_health) =
            game_mem.read_with_offsets::<(f32, f32)>(current_actor, offsets::HEALTH);
        current_player.health_percentage = health / max_health * 100.0;

        //头甲包

        //手持武器，子弹数量，最大子弹数量，人物姿态

        //玩家的速度

        let on_vehicle = game_mem.read_with_offsets::<u64>(current_actor, offsets::ONVEHICLE);
        if on_vehicle != 0 {
            // player is on vehicle
            game_mem.read_memory_with_offsets(
                on_vehicle,
                &mut current_player.velocity,
                offsets::VELOCITYONVEHICLE,
            );
        } else {
            game_mem.read_memory_with_offsets(
                current_actor,
                &mut current_player.velocity,
                offsets::VELOCITYNOTONVEHICLE,
            );
        }
        //玩家是否为bot
        current_player.is_bot = game_mem.read_with_offsets(current_actor, offsets::ISBOT);
        //玩家name
        let mut src: [u16; 16] = [0; 16];
        game_mem.read_memory_with_offsets(current_actor, &mut src, offsets::PLAYERNAME);
        get_utf8(&mut current_player.player_name, &src);

        world_to_screen(
            &mut current_player.screen_position,
            &mut current_player.camera_angle,
            &mut current_player.width,
            &current_player.world_position,
            &game_data.matrix,
            1200.0,
            540.0,
        );

        // read bones positions
        if current_player.camera_angle > 0.0 {
            let mesh: u64 = game_mem.read_with_offsets(current_actor, offsets::MESH);
            let c2w_trans: FTransform =
                game_mem.read_with_offsets(current_actor, offsets::C2W_TRANSFORM);

            let mut head: FTransform = game_mem.read_with_offsets(mesh, offsets::HEAD);
            head.translation.z+=30.0;
            get_bone_pos(
                &head,
                &c2w_trans,
                &mut current_player.head,
                &game_data.matrix,
            );

            let chest: FTransform = game_mem.read_with_offsets(mesh, offsets::CHEST);

            get_bone_pos(
                &chest,
                &c2w_trans,
                &mut current_player.chest,
                &game_data.matrix,
            );

            let pelvis: FTransform = game_mem.read_with_offsets(mesh, offsets::PELVIS);

            get_bone_pos(
                &pelvis,
                &c2w_trans,
                &mut current_player.pelvis,
                &game_data.matrix,
            );

            let left_shoulder: FTransform =
                game_mem.read_with_offsets(mesh, offsets::LEFT_SHOULDER);

            get_bone_pos(
                &left_shoulder,
                &c2w_trans,
                &mut current_player.left_shoulder,
                &game_data.matrix,
            );

            let right_shoulder: FTransform =
                game_mem.read_with_offsets(mesh, offsets::RIGHT_SHOULDER);

            get_bone_pos(
                &right_shoulder,
                &c2w_trans,
                &mut current_player.right_shoulder,
                &game_data.matrix,
            );

            let left_elbow: FTransform = game_mem.read_with_offsets(mesh, offsets::LEFT_ELBOW);

            get_bone_pos(
                &left_elbow,
                &c2w_trans,
                &mut current_player.left_elbow,
                &game_data.matrix,
            );

            let right_elbow: FTransform = game_mem.read_with_offsets(mesh, offsets::RIGHT_ELBOW);

            get_bone_pos(
                &right_elbow,
                &c2w_trans,
                &mut current_player.right_elbow,
                &game_data.matrix,
            );

            let left_wrist: FTransform = game_mem.read_with_offsets(mesh, offsets::LEFT_WRIST);

            get_bone_pos(
                &left_wrist,
                &c2w_trans,
                &mut current_player.left_wrist,
                &game_data.matrix,
            );

            let right_wrist: FTransform = game_mem.read_with_offsets(mesh, offsets::RIGHT_WRIST);

            get_bone_pos(
                &right_wrist,
                &c2w_trans,
                &mut current_player.right_wrist,
                &game_data.matrix,
            );

            let left_thigh: FTransform = game_mem.read_with_offsets(mesh, offsets::LEFT_THIGH);

            get_bone_pos(
                &left_thigh,
                &c2w_trans,
                &mut current_player.left_thigh,
                &game_data.matrix,
            );

            let right_thigh: FTransform = game_mem.read_with_offsets(mesh, offsets::RIGTH_THIGH);

            get_bone_pos(
                &right_thigh,
                &c2w_trans,
                &mut current_player.right_thigh,
                &game_data.matrix,
            );

            let left_knee: FTransform = game_mem.read_with_offsets(mesh, offsets::LEFT_KNEE);

            get_bone_pos(
                &left_knee,
                &c2w_trans,
                &mut current_player.left_knee,
                &game_data.matrix,
            );

            let right_knee: FTransform = game_mem.read_with_offsets(mesh, offsets::RIGHT_KNEE);

            get_bone_pos(
                &right_knee,
                &c2w_trans,
                &mut current_player.right_knee,
                &game_data.matrix,
            );

            let left_ankle: FTransform = game_mem.read_with_offsets(mesh, offsets::LEFT_ANKLE);

            get_bone_pos(
                &left_ankle,
                &c2w_trans,
                &mut current_player.left_ankle,
                &game_data.matrix,
            );

            let right_ankle: FTransform = game_mem.read_with_offsets(mesh, offsets::RIGHT_ANKLE);

            get_bone_pos(
                &right_ankle,
                &c2w_trans,
                &mut current_player.right_ankle,
                &game_data.matrix,
            );
        }

        //for searching bone indexes

        // for i in 0..70 {
        //     let bone: FTransform = game_mem.read_with_offsets(mesh, &[48*i as u64]);
        //     let v2 = trans.rotation.rotate_vec(&bone.translation);
        //     let v3 = trans.translation.translate(&v2);
        //     //v3.z += 7.0;
        //     let mut bone1 :Bone = Bone::default();
        //     world_to_screen_without_depth(
        //         &mut bone1.position_on_screen,
        //         &v3,
        //         &game_data.matrix,
        //         1200.0,
        //         540.0,
        //     );
        //     bone1.name_for_debug = i.to_string();
        //     current_player.bone_debug.push(bone1);
        // }
        game_data.players.push(current_player);
    }
}
fn get_bone_pos(
    bone_trans: &FTransform,
    c2w_trans: &FTransform,
    bone: &mut Bone,
    w2s_matrix: &[f32; 16],
) {
    let v1 = bone_trans.rotation.rotate_vec(&bone_trans.translation);
    let v2 = c2w_trans.rotation.rotate_vec(&v1);
    let v3 = c2w_trans.translation.translate(&v2);
    world_to_screen_without_depth(&mut bone.position_on_screen, &v3, w2s_matrix, 1200.0, 540.0);
}
fn world_to_screen(
    bscreen: &mut Vec2,
    camea: &mut f32,
    w: &mut f32,
    obj: &Vec3,
    matrix: &[f32; 16],
    width: f32,
    height: f32,
) {
    *camea = matrix[3] * obj.x + matrix[7] * obj.y + matrix[11] * obj.z + matrix[15];

    bscreen.x = width
        + (matrix[0] * obj.x + matrix[4] * obj.y + matrix[8] * obj.z + matrix[12]) / *camea * width;
    bscreen.y = height
        - (matrix[1] * obj.x + matrix[5] * obj.y + matrix[9] * obj.z + matrix[13]) / *camea
            * height;

    let bscreen_z = height
        - (matrix[1] * obj.x + matrix[5] * obj.y + matrix[9] * (obj.z + 165.0) + matrix[13])
            / *camea
            * height;
    let bscreenz = bscreen.y - bscreen_z;
    *w = bscreenz / 2.0;
}
fn world_to_screen_without_depth(
    bscreen: &mut Vec2,
    obj: &Vec3,
    matrix: &[f32; 16],
    width: f32,
    height: f32,
) {
    let camea = matrix[3] * obj.x + matrix[7] * obj.y + matrix[11] * obj.z + matrix[15];

    bscreen.x = width
        + (matrix[0] * obj.x + matrix[4] * obj.y + matrix[8] * obj.z + matrix[12]) / camea * width;
    bscreen.y = height
        - (matrix[1] * obj.x + matrix[5] * obj.y + matrix[9] * obj.z + matrix[13]) / camea * height;
}
fn get_utf8(buf: &mut [u8], buf16: &[u16; 16]) {
    let mut p_temp_utf16 = 0;
    let mut p_temp_utf8 = 0;
    let p_utf8_end = buf.len();

    while p_temp_utf16 < 16 && p_temp_utf8 < p_utf8_end && buf16[p_temp_utf16] != 0 {
        let utf16 = buf16[p_temp_utf16];

        if utf16 <= 0x007F && p_temp_utf8 + 1 <= p_utf8_end {
            buf[p_temp_utf8] = utf16 as u8;
            p_temp_utf8 += 1;
        } else if utf16 >= 0x0080 && utf16 <= 0x07FF && p_temp_utf8 + 2 <= p_utf8_end {
            buf[p_temp_utf8] = (utf16 >> 6) as u8 | 0xC0;
            buf[p_temp_utf8 + 1] = (utf16 & 0x3F) as u8 | 0x80;
            p_temp_utf8 += 2;
        } else if utf16 >= 0x0800 && p_temp_utf8 + 3 <= p_utf8_end {
            buf[p_temp_utf8] = (utf16 >> 12) as u8 | 0xE0;
            buf[p_temp_utf8 + 1] = ((utf16 >> 6) & 0x3F) as u8 | 0x80;
            buf[p_temp_utf8 + 2] = (utf16 & 0x3F) as u8 | 0x80;
            p_temp_utf8 += 3;
        } else {
            break;
        }

        p_temp_utf16 += 1;
    }
}

pub fn ui(
    opened: &mut bool,
    ui: &mut Ui,
    frame_rate: &mut f32,
    game_data: &mut GameData,
    game_mem: &mut GameMem,
) {
    ui.window("HEllo world")
        .opened(opened)
        .size([440.0, 320.0], Condition::FirstUseEver)
        .build(|| {
            ui.button("This...is...imgui-rs!");
            ui.separator();
            let mouse_pos = ui.io().mouse_pos;
            ui.text(format!(
                "Mouse Position: ({:.1},{:.1})",
                mouse_pos[0], mouse_pos[1]
            ));

            ui.separator();
            ui.text("chose:");
            ui.same_line();
            ui.radio_button("30", frame_rate, 30.0);
            ui.same_line();
            ui.radio_button("60", frame_rate, 60.0);
            ui.same_line();
            ui.radio_button("90", frame_rate, 90.0);
            ui.same_line();
            ui.radio_button("120", frame_rate, 120.0);
            ui.same_line();

            ui.separator();
            ui.text_colored([1.0, 1.0, 1.0, 1.0], format!("fps : {}", ui.io().framerate));
        });
    get_data(game_mem, game_data);
    esp(ui, game_data);
}


fn esp(ui: &mut Ui, game_data: &mut GameData) {
    let draw_list = ui.get_background_draw_list();
    let col = [1.0,1.0,1.0];
    for player in &game_data.players {
        if player.camera_angle > 0.0 {
            draw_list.add_text(
                player.screen_position.to_pos(),
                [1.0, 1.0, 1.0],
                if player.is_bot {
                    "bot"
                } else {
                    player.get_name()
                },
            );
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
            
            draw_list.add_circle(player.head.position_on_screen.to_pos(), 10.0, col).filled(true).build();

            
        }
    }
}
