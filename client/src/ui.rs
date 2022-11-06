// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::ClientState;
use wolkenwelten_game::GameState;

fn prepare_healthbar(fe: &mut ClientState, game: &GameState, x: i16, y: i16, heart_beat: bool) {
    let health = game.player().health();
    let hp = health.health();
    let max_hp = health.max_health();
    let rem = hp % 4;

    if heart_beat && hp > 0 {
        let tick_rate = if hp < max_hp / 2 {
            if hp < 4 {
                2
            } else {
                3
            }
        } else {
            4
        };
        let ticks = 127 - ((fe.ticks() >> tick_rate) & 0x7F);
        let rgba: [u8; 4] = [255, 255, 255, (ticks << 1) as u8];
        let hb_off = 16 - (ticks as i16 >> 3);

        if rem == 0 {
            fe.ui_mesh.push_heart(
                x + hp / 4 * 40 - hb_off - 40,
                y - hb_off,
                32 + hb_off * 2,
                rgba,
                4,
            );
        } else {
            fe.ui_mesh.push_heart(
                x + hp / 4 * 40 - hb_off,
                y - hb_off,
                32 + hb_off * 2,
                rgba,
                rem,
            );
        }
    }

    for heart in 0..hp / 4 {
        fe.ui_mesh.push_heart(x + heart * 40, y, 32, [255; 4], 4);
    }

    if hp < max_hp {
        fe.ui_mesh.push_heart(x + hp / 4 * 40, y, 32, [255; 4], rem);
    }

    let rest = hp / 4 + 1;
    for dot in rest..max_hp / 4 {
        fe.ui_mesh.push_heart(x + dot * 40, y, 32, [255; 4], 0);
    }
}

fn prepare_fps(fe: &mut ClientState) {
    let (window_width, _window_height) = fe.window_size();
    let fps_text = format!("FPS: {}", fe.fps());
    fe.ui_mesh.push_string(
        window_width as i16 - 128,
        8,
        2,
        [0xFF, 0xFF, 0xFF, 0xFF],
        fps_text.as_str(),
    );
}

fn prepare_pos(fe: &mut ClientState, game: &GameState) {
    let pos = game.player().pos;
    let pos_text = format!(
        "X:{:8.2} Y:{:8.2} Z:{:8.2}   Ticks:{}",
        pos[0], pos[1], pos[2], game.ticks_elapsed
    );
    fe.ui_mesh
        .push_string(8, 64, 2, [0xFF, 0xFF, 0xFF, 0xFF], pos_text.as_str());
}

fn prepare_block_selection(fe: &mut ClientState, game: &GameState) {
    let block_name = game.world.blocks[game.player().block_selection() as usize].name();
    let block_sel_text = format!("Block selection: {}", block_name);
    fe.ui_mesh.push_string(
        8,
        fe.window_size().1 as i16 - 20,
        2,
        [0xFF, 0xFF, 0xFF, 0xFF],
        block_sel_text.as_str(),
    );
}

#[cfg(debug_assertions)]
fn prepare_debug_text(fe: &mut ClientState, game: &GameState) {
    let col_text = format!(
        "Entities: {}   Chunks: {}   BlockMeshes: {}  Particles: {}",
        game.get_entity_count(),
        game.world.chunks.len(),
        fe.world_mesh.len(),
        fe.particles.len(),
    );
    fe.ui_mesh
        .push_string(8, 84, 2, [0xFF, 0xFF, 0xFF, 0xFF], col_text.as_str());
}

fn prepare_crosshair(fe: &mut ClientState) {
    let (window_width, window_height) = fe.window_size();

    let pos = (
        window_width as i16 / 2 - 32,
        window_height as i16 / 2 - 32,
        32,
        32,
    );
    let tex = (200, 252, 4, 4);
    fe.ui_mesh.push_box(pos, tex, [0xFF, 0xFF, 0xFF, 0x7F]);
}

pub fn prepare(fe: &mut ClientState, game: &GameState) {
    prepare_fps(fe);
    prepare_pos(fe, game);
    prepare_block_selection(fe, game);
    prepare_crosshair(fe);
    prepare_healthbar(fe, game, 16, 16, true);

    #[cfg(debug_assertions)]
    prepare_debug_text(fe, game);

    fe.ui_mesh.prepare(&fe.display);
}