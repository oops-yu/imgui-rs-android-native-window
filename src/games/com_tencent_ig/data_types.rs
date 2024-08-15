#[repr(C)]
#[derive(Default, Debug)]
pub struct Quat {
    
    pub x: f32,

    pub y: f32,
    
    pub z: f32,

    pub w: f32,

}
pub fn multiply_matrices(a: &[f32; 16], b: &[f32; 16]) -> [f32; 16] {
    let mut result = [0.0; 16];

    for i in 0..4 {
        for j in 0..4 {
            result[i * 4 + j] = a[i * 4 + 0] * b[0 * 4 + j]
                             + a[i * 4 + 1] * b[1 * 4 + j]
                             + a[i * 4 + 2] * b[2 * 4 + j]
                             + a[i * 4 + 3] * b[3 * 4 + j];
        }
    }

    result
}
pub fn transform_to_matrix(transform: &FTransform) -> [f32; 16] {
    let x2 = transform.rotation.x + transform.rotation.x;
    let y2 = transform.rotation.y + transform.rotation.y;
    let z2 = transform.rotation.z + transform.rotation.z;
    
    let xx2 = transform.rotation.x * x2;
    let yy2 = transform.rotation.y * y2;
    let zz2 = transform.rotation.z * z2;

    let yz2 = transform.rotation.y * z2;
    let wx2 = transform.rotation.w * x2;
    let xy2 = transform.rotation.x * y2;
    let wz2 = transform.rotation.w * z2;
    let xz2 = transform.rotation.x * z2;
    let wy2 = transform.rotation.w * y2;

    [
        (1.0 - (yy2 + zz2)) * transform.scale.x, (xy2 + wz2) * transform.scale.y, (xz2 - wy2) * transform.scale.z, 0.0,
        (xy2 - wz2) * transform.scale.x, (1.0 - (xx2 + zz2)) * transform.scale.y, (yz2 + wx2) * transform.scale.z, 0.0,
        (xz2 + wy2) * transform.scale.x, (yz2 - wx2) * transform.scale.y, (1.0 - (xx2 + yy2)) * transform.scale.z, 0.0,
        transform.translation.x, transform.translation.y, transform.translation.z, 1.0,
    ]
}
#[repr(C)]
#[derive(Default, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vec3{
    pub fn translate(&self,other:&Self)->Self{
        Self{
            x:self.x+other.x,
            y:self.y+other.y,
            z:self.z+other.z
        }
    }
}

#[repr(C)]
#[derive(Default, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Default, Debug)]
pub struct FTransform {
    pub rotation: Quat,    // 旋转四元数
    pub translation: Vec3, // 位移向量
    pub chunk: f32,
    pub scale: Vec3, // 3D 缩放向量
}
#[repr(C)]
#[derive(Default, Debug)]
pub struct Bone {
    pub world_position: Vec3,     // 世界坐标
    pub position_on_screen: Vec2, // 屏幕坐标
}
#[repr(C)]
#[derive(Default, Debug)]
pub struct Player {
    pub width: f32,              // 人物宽度
    pub world_position: Vec3,    // 世界坐标
    pub screen_position: Vec2,   // 屏幕坐标
    pub camera_angle: f32,       // 人物相机
    pub team_id: i32,            // 队标
    pub action_id: i32,          // 动作
    pub weapon_id: i32,          // 手持
    pub bullet_count: i32,       // 子弹
    pub max_bullets: i32,        // 最大子弹
    pub backpack: i32,           // 背包
    pub helmet: i32,             // 头盔
    pub armor: i32,              // 敌人甲
    pub is_bot: bool,            // 人机
    pub health_percentage: f32,  // 血量百分比
    pub distance_to_player: f32, // 距离
    pub player_name: [u8; 32],   // 玩家名称，字符数组需要转为字节数组
    pub velocity: Vec3,          // 速度
    pub head: Bone,
    pub chest: Bone,
    pub pelvis: Bone,
    pub left_shoulder: Bone,
    pub right_shoulder: Bone,
    pub left_elbow: Bone,
    pub right_elbow: Bone,
    pub left_wrist: Bone,
    pub right_wrist: Bone,
    pub left_thigh: Bone,
    pub right_thigh: Bone,
    pub left_knee: Bone,
    pub right_knee: Bone,
    pub left_ankle: Bone,
    pub right_ankle: Bone,
}
impl Player {
    pub fn position_valid(&self) -> bool {
        !(self.world_position.x == 0.0
            && self.world_position.y == 0.0
            && self.world_position.z == 0.0)
    }
    pub fn is_in_screen(&self) -> bool {
        self.camera_angle > 0.0
    }
    pub fn get_name<'a>(&'a self) -> &'a str {
        // 查找0x00的位置
        let len = self
            .player_name
            .iter()
            .position(|&c| c == 0x00)
            .unwrap_or(self.player_name.len());

        // 创建不包括0x00的子切片
        let utf8_slice = &self.player_name[..len];

        // 将子切片转换为 &str
        std::str::from_utf8(utf8_slice).expect("Invalid UTF-8 sequence")
    }
}
#[repr(C)]
pub struct Supply {
    pub width_on_screen: f32, // 在屏幕上的宽度宽度
    pub world_position: Vec3, // 世界坐标
    pub screen_pos: Vec2,     // 屏幕坐标
    pub camera_angle: f32,    // 人物相机
    pub distance: f32,        // 距离
    pub name: [u8; 128],      // 玩家名称，字符数组需要转为字节数组
}
