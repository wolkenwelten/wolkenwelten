use super::{BlockType, Character, ChunkBlockData};
use glam::f32::Vec3;
use glam::i32::IVec3;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Chungus {
    pub blocks: Vec<BlockType>,
    pub block_data: HashMap<IVec3, ChunkBlockData>,
}

impl Chungus {
    pub fn gc(&mut self, player: &Character) {
        let mut removal_queue: Vec<IVec3> = Vec::new();
        for pos in self.block_data.keys() {
            let diff: Vec3 = (pos.as_vec3() * 16.0) - player.pos;
            let d = diff.dot(diff);
            if d > (256.0 * 256.0) {
                removal_queue.push(*pos);
            }
        }
        for pos in removal_queue {
            self.block_data.remove(&pos);
        }
    }
    pub fn get(&self, k: &IVec3) -> Option<&ChunkBlockData> {
        self.block_data.get(k)
    }
    pub fn insert(&mut self, k: IVec3, v: ChunkBlockData) {
        self.block_data.insert(k, v);
    }

    pub fn get_block_type(&self, i: u8) -> &BlockType {
        &self.blocks[i as usize]
    }

    pub fn is_solid(&self, pos: Vec3) -> bool {
        let cp = IVec3::new(pos.x as i32 >> 4, pos.y as i32 >> 4, pos.z as i32 >> 4);
        let chnk = self.get(&cp);
        if let Some(chnk) = chnk {
            let cx = (pos.x as i32 & 15) as usize;
            let cy = (pos.y as i32 & 15) as usize;
            let cz = (pos.z as i32 & 15) as usize;
            let b = chnk.data[cx][cy][cz];
            b != 0
        } else {
            false
        }
    }
}

impl Default for Chungus {
    fn default() -> Self {
        Self {
            blocks: BlockType::load_all(),
            block_data: HashMap::with_capacity(4096),
        }
    }
}
