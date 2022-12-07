// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
mod block_type;
mod chunk;
mod experience;
mod item;
mod iter;
mod message;
mod queue;
mod reactor;

pub use block_type::*;
pub use chunk::*;
pub use experience::*;
pub use item::*;
pub use iter::*;
pub use message::*;
pub use queue::*;
pub use reactor::*;

pub const CHUNK_BITS: i32 = 5;
pub const CHUNK_SIZE: usize = 1 << CHUNK_BITS;
pub const CHUNK_MASK: i32 = CHUNK_SIZE as i32 - 1;

pub type ChunkData = [[[u8; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

pub type ChunkBuffer = [[[u8; CHUNK_SIZE + 2]; CHUNK_SIZE + 2]; CHUNK_SIZE + 2];

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub enum Side {
    #[default]
    Front = 0,
    Back,
    Top,
    Bottom,
    Left,
    Right,
}

impl From<Side> for u8 {
    fn from(s: Side) -> Self {
        s as u8
    }
}
impl From<Side> for usize {
    fn from(s: Side) -> Self {
        s as usize
    }
}
pub use self::character::{Character, CharacterAnimation, RaycastReturn};
pub use self::chungus::Chungus;
pub use self::entity::Entity;
pub use self::game_log::GameLog;
pub use self::health::Health;
pub use self::state::GameState;

mod character;
mod chungus;
mod entity;
mod game_log;
mod health;
mod state;
mod worldgen;
