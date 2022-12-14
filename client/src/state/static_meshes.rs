// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::{Mesh, MeshVertex, VoxelMesh};
use anyhow::Result;
use wolkenwelten_core::BLOCKS;

/// This struct is meant to be a simple way to store
/// all static meshes included with WW.
#[derive(Debug)]
pub struct MeshList {
    pub fist: VoxelMesh,
    pub blocks: Vec<Mesh>,
}

impl MeshList {
    fn gen_block_meshes(display: &glium::Display) -> Result<Vec<Mesh>> {
        let tile_size = 64.0 / 2048.0;
        BLOCKS.with(|blocks| {
            Ok(blocks
                .borrow()
                .iter()
                .map(|block| {
                    let mut vertices: Vec<MeshVertex> = vec![];
                    Mesh::add_block_type(&mut vertices, block, tile_size);
                    Mesh::from_vec(display, &vertices).expect("Couldn't create block mesh")
                })
                .collect())
        })
    }

    /// Load all the the models from the build-in raw .obj/.vox bytes.
    pub fn new(display: &glium::Display) -> Result<Self> {
        Ok(Self {
            blocks: Self::gen_block_meshes(display)?,
            fist: VoxelMesh::from_vox_data(display, include_bytes!("../../assets/fist.vox"))?,
        })
    }
}
