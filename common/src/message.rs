// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::Item;
use glam::{IVec3, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
#[serde(tag = "T")]
pub enum Message {
    #[default]
    NoOp,
    DrawFrame {
        player_pos: Vec3,
        ticks: u64,
        render_distance: f32,
    },
    FinishedFrame {
        player_pos: Vec3,
        ticks: u64,
        render_distance: f32,
    },
    GameTick {
        ticks: u64,
    },
    GameQuit,
    GameInit,

    CharacterPosRotVel {
        pos: Vec3,
        rot: Vec3,
        vel: Vec3,
    },
    CharacterJump {
        pos: Vec3,
    },
    CharacterStomp {
        pos: Vec3,
    },
    CharacterShoot {
        pos: Vec3,
    },
    CharacterDamage {
        pos: Vec3,
        damage: i16,
    },
    CharacterDeath {
        pos: Vec3,
    },
    CharacterStep {
        pos: Vec3,
    },
    BlockMine {
        pos: IVec3,
        block: u8,
    },
    BlockBreak {
        pos: IVec3,
        block: u8,
    },
    BlockPlace {
        pos: IVec3,
        block: u8,
    },
    EntityCollision {
        pos: Vec3,
    },
    ItemDropPickup {
        pos: Vec3,
        item: Item,
    },

    PlayerShoot,
    PlayerDropItem,
    PlayerMove {
        direction: Vec3,
    },
    PlayerFly {
        direction: Vec3,
    },
    PlayerTurn {
        direction: Vec3,
    },
    PlayerBlockMine {
        pos: Option<IVec3>,
    },
    PlayerBlockPlace {
        pos: IVec3,
    },
    PlayerSwitchSelection {
        delta: i32,
    },
    PlayerSelect {
        i: i32,
    },
    PlayerNoClip {
        no_clip: bool,
    },

    SfxPlay {
        pos: Vec3,
        volume: f32,
        sfx: SfxId,
    },
}

// Would love to replace this enum by a string or something similar
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
#[serde(tag = "T")]
pub enum SfxId {
    #[default]
    Void,
    Jump,
    HookFire,
    Ungh,
    Step,
    Stomp,
    Bomb,
    Pock,
    Tock,
}

impl Message {
    /// Returns a positions if there is one associated with that message, mainly
    /// used for positioning sound effects.
    pub fn pos(&self) -> Option<Vec3> {
        match self {
            Message::BlockPlace { pos, .. }
            | Message::BlockBreak { pos, .. }
            | Message::BlockMine { pos, .. } => Some(pos.as_vec3()),
            Message::ItemDropPickup { pos, .. }
            | Message::EntityCollision { pos, .. }
            | Message::SfxPlay { pos, .. }
            | Message::CharacterStep { pos, .. }
            | Message::CharacterDeath { pos, .. }
            | Message::CharacterDamage { pos, .. }
            | Message::CharacterShoot { pos, .. }
            | Message::CharacterStomp { pos, .. }
            | Message::CharacterJump { pos, .. } => Some(*pos),
            _ => None,
        }
    }
}
