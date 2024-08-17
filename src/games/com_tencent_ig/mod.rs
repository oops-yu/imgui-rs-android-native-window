mod aim_bot;
mod config;
mod data;
mod data_types;
mod esp;
mod offsets;
mod ui;

use crate::common::System;
use crate::memory_helper::GameMem;
use data::GameData;
pub fn run() {
    //0.init driver
    let mut game_mem = GameMem::new();
    game_mem.initialize_with_process_name("com.tencent.ig");

    data::set_ue4(game_mem.get_module_base("libUE4.so").unwrap());
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
            ui::gen_user_interface(run, ui1, frame_rate);
            data::prepare_data(&mut game_mem, &mut game_data);
            esp::esp(ui1, &mut game_data);
        })
        .expect("failed");
}
