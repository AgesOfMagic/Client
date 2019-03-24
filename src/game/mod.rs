use std::collections::HashMap;
pub struct Game{
    pub entity_vec: HashMap<u32,(i32,i32)>,
    pub player_pos: (i32,i32)
}

pub fn get_distance(pos1: (i32,i32),pos2: (i32,i32)) -> f64{
    ((pos1.0 - pos2.0).pow(2)  as f64 + (pos1.1 - pos2.1).pow(2) as f64).sqrt().abs()
}