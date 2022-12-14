// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::ClientState;
use wolkenwelten_core::GAME_LOG;

pub fn prepare(fe: &mut ClientState) {
    let (_window_width, window_height) = fe.window_size();

    let x = 16;
    let y = (window_height - 32) as i32;

    GAME_LOG.with(|log| {
        for (i, s) in log.borrow().entries().iter().enumerate() {
            let (s, t) = s;
            let a = t.elapsed().as_millis().min(255) as u8;
            fe.ui_mesh.push_string(
                x as i16,
                (y - i as i32 * 20) as i16,
                2,
                [0xFF, 0xFF, 0xFF, a],
                s,
            );
        }
    });
}
