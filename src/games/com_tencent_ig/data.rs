#[allow(unused_imports)]
use super::data_types::*;
use nohash_hasher::IntSet;
pub struct GameData {
    pub local_player:u64,
    pub local_team_id:i32,
    pub fov: f32,          // 自身fov
    pub matrix: [f32; 16], // 游戏矩阵
    pub firing: i32,       // 开火判断
    pub aiming: i32,       // 开镜判断
    pub local_weapon: i32, // 自身手持
    pub angle: f32,
    pub local_position: Vec3,
    pub players: Vec<Player>,
    pub supplies: Vec<Supply>,
    pub players_set:IntSet<u64>,
    pub non_player_set:IntSet<u64>,
    pub local_team_set:IntSet<u64>,
    pub actor_array:[u64;2000]
}
impl Default for GameData {
    fn default() -> Self {
        Self {
            local_player:0,
            local_team_id:0,
            fov: 0.0,
            matrix: [0.0; 16],
            firing: 0,
            aiming: 0,
            local_weapon: 0,
            angle: 0.0,
            local_position: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            players: Vec::with_capacity(100),   // 使用默认值初始化
            supplies: Vec::with_capacity(1000), // 初始化 Vec 具有 1000 的容量
            local_team_set:IntSet::default(),
            players_set:IntSet::default(),
            non_player_set:IntSet::default(),
            actor_array:[0;2000]
        }
    }
}
