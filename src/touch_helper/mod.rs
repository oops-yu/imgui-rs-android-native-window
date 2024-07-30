use evdev::{AbsoluteAxisType, Device, InputEventKind, Synchronization};


#[derive(Debug,Clone)]
pub struct FingerState {
    pub is_down: bool,
    pub pos: (f32, f32),
}
impl FingerState {
    pub fn new()->Self{
        Self { is_down: false, pos: (0.0,0.0) }
    }
}
pub type MousePos = FingerState;
pub struct Touch {
    device: Device,
    finger_states: Vec<FingerState>,
    least_finger_idx: usize,
    scale_x: f32,
    scale_y: f32,
}


impl Touch {
    pub fn new() -> Self {
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
        let scale_x = phy_win_x / 1080.0;
        let scale_y = phy_win_y / 2400.0;
        let mut finger_states: Vec<FingerState> = vec![];

        finger_states.resize_with(10, || FingerState {
            is_down: false,
            pos: (0.0, 0.0),
        });

        let  least_finger_idx: usize = 0;
        Self { device, finger_states , least_finger_idx, scale_x, scale_y }
    }
    pub fn refresh_current_state(&mut self,state:std::sync::Arc<std::sync::Mutex<MousePos>>){
        loop {
            // 读取输入事件
            for ev in self.device.fetch_events().expect("fetch_events failed!") {
                match ev.kind() {
                    InputEventKind::AbsAxis(axis) => match axis {
                        AbsoluteAxisType::ABS_MT_SLOT => {
                            self.least_finger_idx =  (ev.code() as usize).min(0).max(9);
                        }
                        AbsoluteAxisType::ABS_MT_TRACKING_ID => {
                            if ev.value() == -1 {
                                self.finger_states[self.least_finger_idx].is_down = false;
                            } else {
                                self.finger_states[self.least_finger_idx].is_down = true;
                            }
                        }
                        AbsoluteAxisType::ABS_MT_POSITION_X => {
                            self.finger_states[self.least_finger_idx].pos.0 = ev.value() as f32 / self.scale_x;
                        }
                        AbsoluteAxisType::ABS_MT_POSITION_Y => {
                            self.finger_states[self.least_finger_idx].pos.1 = ev.value() as f32 / self.scale_y;
                        }
                        _ => {}
                    },
                    InputEventKind::Synchronization(syn) => match syn {
                        Synchronization::SYN_REPORT => {
                            if self.finger_states[self.least_finger_idx].is_down {
                                
                                if let Ok(mut pos) = state.try_lock(){
                                    //println!("update finger state:{:?}", self.finger_states[self.least_finger_idx]);
                                    (*pos) = self.finger_states[self.least_finger_idx].clone();
                                }
                            } else {
                                if let Ok(mut pos) = state.try_lock(){
                                    //println!("update finger state:{:?}", self.finger_states[self.least_finger_idx]);
                                    (*pos).is_down = self.finger_states[self.least_finger_idx].is_down;
                                }
                            }
                        }
                        _ => {}
                    },
                    _ => {
                        
                    }
                }
            }
        }
    }
}

fn is_touch(device: &Device) -> bool {
    return device.supported_absolute_axes().map_or(false, |axes| {
        axes.contains(AbsoluteAxisType::ABS_X)
            && axes.contains(AbsoluteAxisType::ABS_MT_SLOT)
            && axes.contains(AbsoluteAxisType::ABS_Y)
    });
}
