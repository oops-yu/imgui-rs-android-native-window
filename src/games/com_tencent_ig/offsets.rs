
macro_rules! define_offsets {
    ($($name:ident : $value:expr),*) => {
        $(
            pub static $name: &[u64] = &$value;
        )*
    };
}


define_offsets!(
    UWORLD: [0xCB39300, 0x20],
    GNAME: [0xC4444F0, 0x120],
    ULEVEL: [0x30],
    OBJARR: [0xA0],
    PROJECTIONMATRIX: [0xCB12028, 0x20, 0x270],
    LOCALPALYER: [0xCB15820, 0x30, 0x4A8],
    PLAYERPOSITION: [0x1b0, 0x1c0],
    LOCALFOV: [0xCF67840, 0x108, 0x4D4],
    ISFIRING:[0x1608],
    ISAIMING:[0x1030],
    OBJTYPE:[0x27d8],
    UK0X1B0:[0x1B0],
    UK0XF60:[0xF60],
    UK0X1C0:[0x1C0],
    TEAMID:[0x928],
    HEALTH:[0xda0],
    ONVEHICLE:[0x1c0],
    VELOCITYNOTONVEHICLE:[0x1bb0, 0x12c],
    VELOCITYONVEHICLE:[0x1330],
    ISBOT:[0x9d9]

);