static mut UE4: u64 = 0;

mod offsets;
mod data;
mod data_types;
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
    loop{
        std::thread::sleep(Duration::from_millis(500));
        get_data(&mut game_mem, game_data.clone())
    }
}
struct Config {}

use imgui::Condition;
use imgui::Ui;

#[allow(unused_imports)]
use data_types::*;
#[allow(unused_imports)]
use data::*;
pub fn get_data(game_mem: &mut GameMem,game_data: std::sync::Arc<std::sync::RwLock<GameData>>) {

    let ue4 = unsafe { UE4 };
    
    let uworld = game_mem
        .read_with_offsets::<u64>(ue4, offsets::UWORLDOFFSET)
        .unwrap_or_default();
    let gname = game_mem
        .read_with_offsets::<u64>(ue4, offsets::GNAMEOFFSET)
        .unwrap_or_default();
    let ulevel = game_mem
        .read_with_offsets::<u64>(uworld, offsets::ULEVELOFFSET)
        .unwrap_or_default();
    let (obj_addr,obj_count) = game_mem
        .read_with_offsets::<(u64,i32)>(ulevel, offsets::OBJARROFFSET)
        .unwrap_or_default();
    println!("ue4:{ue4}\nuworld:{uworld}\ngname:{gname}\nulevel:{ulevel}\n");
    if obj_count <=0 || obj_count > 2000{
        return;
    }
    let mut game_data = game_data.write().unwrap();
    if !game_mem.read_memory_with_offsets(ue4,&mut game_data.matrix,offsets::PROJECTIONMATRIXOFFSET){
        panic!("fas")
    }
    for i in 0..4{
        for j in 0..4{
            print!("{:.2} ",game_data.matrix[i*4+j]);
        }
        println!("");
    }

    
    
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
