static mut UE4: u64 = 0;

mod data;
mod data_types;
mod offsets;
use std::time::Duration;

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
    let game_data = std::sync::Arc::new(std::sync::RwLock::new(GameData::default()));

    loop {
        std::thread::sleep(Duration::from_millis(2000));
        get_data(&mut game_mem, game_data.clone())
    }
}
struct Config {}

use imgui::Condition;
use imgui::Ui;

#[allow(unused_imports)]
use data::*;
#[allow(unused_imports)]
use data_types::*;
pub fn get_data(game_mem: &mut GameMem, game_data: std::sync::Arc<std::sync::RwLock<GameData>>) {
    let ue4 = unsafe { UE4 };

    let uworld = game_mem.read_with_offsets::<u64>(ue4, offsets::UWORLD);
    let gname = game_mem.read_with_offsets::<u64>(ue4, offsets::GNAME);
    let ulevel = game_mem.read_with_offsets::<u64>(uworld, offsets::ULEVEL);
    let (objs_addr, objs_count) = game_mem.read_with_offsets::<(u64, i32)>(ulevel, offsets::OBJARR);
    println!(
        "ue4:{ue4:#016x}\nuworld:{uworld:#016x}\ngname:{gname:#016x}\nulevel:{ulevel:#016x}\n"
    );
    if objs_count <= 0 || objs_count > 2000 {
        return;
    }

    let mut game_data = game_data.write().unwrap();
    //read local player information
    game_mem.read_memory_with_offsets(ue4, &mut game_data.matrix, offsets::PROJECTIONMATRIX);
    game_data.local_player = game_mem.read_with_offsets(ue4, offsets::LOCALPALYER);
    game_mem.read_memory_with_offsets(game_data.local_player, &mut game_data.local_position, offsets::PLAYERPOSITION);
    game_data.fov = game_mem.read_with_offsets(game_data.local_player, offsets::LOCALFOV);
    game_data.firing = game_mem.read_with_offsets(game_data.local_player, offsets::ISFIRING);
    game_data.aiming = game_mem.read_with_offsets(game_data.local_player, offsets::ISAIMING);
    game_data.local_team_id = game_mem.read_with_offsets(game_data.local_player, offsets::TEAMID);

    game_data.players.clear();
    //read player array
    for i in 0..objs_count{
        let current_obj = game_mem.read_with_offsets::<u64>(objs_addr, &[8*i as u64]);
        let current_obj_type = game_mem.read_with_offsets::<f32>(current_obj, offsets::OBJTYPE);
        if current_obj_type != 479.5{
            continue;
        }
        //这里不知道跳过了啥
        let uk0x1b0 = game_mem.read_with_offsets::<u64>(current_obj, offsets::UK0X1B0);
        if uk0x1b0 <= 0xffff || uk0x1b0 == 0 || uk0x1b0 <= 0x10000000 || uk0x1b0 % 4 != 0 || uk0x1b0 >= 0x10000000000{
            continue;
        }
        let uk0xf60 = game_mem.read_with_offsets::<i32>(current_obj, offsets::UK0XF60);
        if uk0xf60 == 262144 || uk0xf60 == 262152{
            continue;
        }
        
        let mut  current_player = Player::default();
        game_mem.read_memory_with_offsets(uk0x1b0, &mut current_player.world_position, offsets::UK0X1C0);
        if !current_player.position_valid(){
            continue;
        }
        current_player.team_id = game_mem.read_with_offsets(current_obj, offsets::TEAMID);
        if current_player.team_id == game_data.local_team_id || current_player.team_id < 1{
            continue;
        }

        //血量
        let (health,max_health) = game_mem.read_with_offsets::<(f32,f32)>(current_obj, offsets::HEALTH);
        current_player.health_percentage = health/max_health * 100.0;

        //头甲包

        //手持武器，子弹数量，最大子弹数量，人物姿态

        //玩家的速度

        let on_vehicle = game_mem.read_with_offsets::<u64>(current_obj, offsets::ONVEHICLE);
        if on_vehicle != 0 { // player is on vehicle
            game_mem.read_memory_with_offsets(on_vehicle, &mut current_player.velocity, offsets::VELOCITYONVEHICLE);
        }else{
            game_mem.read_memory_with_offsets(current_obj, &mut current_player.velocity, offsets::VELOCITYNOTONVEHICLE);
        }
        //玩家是否为bot
        current_player.is_bot = game_mem.read_with_offsets(current_obj, offsets::ISBOT);


        println!("{:?}",current_player);

        
        
        
        
    }



    //和
    // for i in 0..4{
    //     for j in 0..4{
    //         print!("{:.2} ",game_data.matrix[i*4+j]);
    //     }
    //     println!("");
    // }
}

pub fn ui(opened: &mut bool, ui: &mut Ui, frame_rate: &mut f32, config: &mut Config) {
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
}

// #[allow(unused_imports)]
// use simple_logger::SimpleLogger;
// #[cfg(debug_assertions)]
// SimpleLogger::new().init()?;
// let mut value = 0;
// let choices = ["test test this is 1", "test test this is 2"];
// System::new(APP_NAME)?.run((), move |run, ui, frame_rate| {
//  ui(run,ui,frame_rate,config)
//  esp(ui,config)
// })?;
