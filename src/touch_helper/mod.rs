use evdev::{AbsoluteAxisType, Device, InputEventKind, Synchronization};

#[derive(Debug, Clone)]
pub struct FingerState {
    pub is_down: bool,
    pub pos: (f32, f32),
}
impl FingerState {
    pub fn new() -> Self {
        Self {
            is_down: false,
            pos: (0.0, 0.0),
        }
    }
}
pub type MousePos = FingerState;
pub struct Touch {
    device: Device,
    finger_states: Vec<FingerState>,
    least_finger_idx: usize,
    scale_x: f32,
    scale_y: f32,
    screen_width: f32,
    screen_height: f32,
    realtime_orientation_cache: u8,
}

impl Touch {
    pub fn new(mut screen_width: f32,mut  screen_height: f32) -> Self {
        let mut i = 0;
        let device: Option<Device> = loop {
            if let Ok(dev) = Device::open(format!("/dev/input/event{i}")) {
                if is_touch(&dev) {
                    break Some(dev);
                }
                i += 1;
                continue;
            } else {
                break None;
            }
        };

        let device = device.unwrap_or_else(|| panic!("can not find touch in your device"));
        // 打印设备名称
        println!("found touch: {}", device.name().unwrap_or("Unknown"));
        let infos = device.get_abs_state().expect("can not get abs info");
        let phy_win_x = infos[0].maximum as f32;
        let phy_win_y = infos[1].maximum as f32;
        // 解决横屏的情况
        if screen_height<screen_width {
            std::mem::swap(&mut screen_height, &mut screen_width)
        }
        let scale_x = phy_win_x / screen_width;
        let scale_y = phy_win_y / screen_height;
        let mut finger_states: Vec<FingerState> = vec![];

        finger_states.resize_with(10, || FingerState {
            is_down: false,
            pos: (0.0, 0.0),
        });

        let least_finger_idx: usize = 0;
        Self {
            device,
            finger_states,
            least_finger_idx,
            scale_x,
            scale_y,
            realtime_orientation_cache: 1,
            screen_width,
            screen_height,
        }
    }
    pub fn refresh_current_state(
        &mut self,
        state: std::sync::Arc<std::sync::Mutex<MousePos>>,
        realtime_orientation: std::sync::Arc<std::sync::Mutex<u8>>,
    ) {
        loop {
            // 读取输入事件
            for ev in self.device.fetch_events().expect("fetch_events failed!") {
                match ev.kind() {
                    InputEventKind::AbsAxis(axis) => match axis {
                        AbsoluteAxisType::ABS_MT_SLOT => {
                            self.least_finger_idx = (ev.code() as usize).min(0).max(9);
                        }
                        AbsoluteAxisType::ABS_MT_TRACKING_ID => {
                            if ev.value() == -1 {
                                self.finger_states[self.least_finger_idx].is_down = false;
                            } else {
                                self.finger_states[self.least_finger_idx].is_down = true;
                            }
                        }
                        AbsoluteAxisType::ABS_MT_POSITION_X => {
                            self.finger_states[self.least_finger_idx].pos.0 =
                                ev.value() as f32 / self.scale_x;
                        }
                        AbsoluteAxisType::ABS_MT_POSITION_Y => {
                            self.finger_states[self.least_finger_idx].pos.1 =
                                ev.value() as f32 / self.scale_y;
                        }
                        _ => {}
                    },
                    InputEventKind::Synchronization(syn) => match syn {
                        Synchronization::SYN_REPORT => {
                            if self.finger_states[self.least_finger_idx].is_down {
                                if let Ok(mut pos) = state.try_lock() {
                                    if let Ok(orientation) = realtime_orientation.try_lock() {
                                        self.realtime_orientation_cache = *orientation;
                                    }
                                    (*pos) = Self::touch_2_screen(
                                        self.screen_width,
                                        self.screen_height,
                                        self.realtime_orientation_cache,
                                        self.finger_states[self.least_finger_idx].clone(),
                                    );
                                }
                            } else {
                                if let Ok(mut pos) = state.try_lock() {
                                    (*pos).is_down =
                                        self.finger_states[self.least_finger_idx].is_down;
                                }
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }

    fn touch_2_screen(
        screen_width: f32,
        screen_height: f32,
        realtime_orientation: u8,
        phy_mouse_pos: MousePos,
    ) -> MousePos {
        let mut phy_mouse_pos = phy_mouse_pos;
        let x = phy_mouse_pos.pos.0.clone();
        let y = phy_mouse_pos.pos.1.clone();
        match realtime_orientation {
            1 => {
                phy_mouse_pos.pos.0 = y;
                phy_mouse_pos.pos.1 = screen_width - x;
            }
            3 => {
                phy_mouse_pos.pos.1 = x;
                phy_mouse_pos.pos.0 = screen_height - y;
            }
            _ => {}
        }
        phy_mouse_pos
    }
}

fn is_touch(device: &Device) -> bool {
    return device.supported_absolute_axes().map_or(false, |axes| {
        axes.contains(AbsoluteAxisType::ABS_X)
            && axes.contains(AbsoluteAxisType::ABS_MT_SLOT)
            && axes.contains(AbsoluteAxisType::ABS_Y)
    });
}
