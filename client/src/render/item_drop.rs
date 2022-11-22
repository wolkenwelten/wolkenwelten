// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::ClientState;
use anyhow::Result;
use glam::{Mat4, Vec3};
use wolkenwelten_game::ItemDrop;

pub fn draw(
    frame: &mut glium::Frame,
    fe: &ClientState,
    entity: &ItemDrop,
    view: &Mat4,
    projection: &Mat4,
) -> Result<()> {
    let rot = entity.rot();
    let pos = entity.pos();

    let model = Mat4::from_scale(Vec3::new(1.0 / 32.0, 1.0 / 32.0, 1.0 / 32.0));
    let model = Mat4::from_rotation_x(rot.x.to_radians()) * model;
    let model = Mat4::from_rotation_y(rot.y.to_radians()) * model;
    let model = Mat4::from_translation(pos) * model;
    let vp = projection.mul_mat4(view);
    let mvp = vp.mul_mat4(&model);
    fe.meshes
        .bag
        .draw(frame, fe.block_indeces(), &fe.shaders.block, &mvp, 1.0)
}
