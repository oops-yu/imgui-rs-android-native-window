use imgui::Condition;
use imgui::Ui;

pub fn gen_user_interface(opened: &mut bool, ui: &mut Ui, frame_rate: &mut f32) {
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
