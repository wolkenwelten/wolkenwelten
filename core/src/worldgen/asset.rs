// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::{ChunkBlockData, CHUNK_SIZE};
use anyhow::{anyhow, Result};
use glam::IVec3;
use vox_format::types::Model;

#[derive(Debug)]
pub struct WorldgenAsset {
    pub palette: Vec<u8>,
    pub data: Vec<u8>,
    pub size: IVec3,
}

#[derive(Debug)]
pub struct WorldgenAssetList {
    pub trees: [WorldgenAsset; 3],
    pub bushes: [WorldgenAsset; 3],
    pub rocks: [WorldgenAsset; 3],
    pub spruce_trees: [WorldgenAsset; 1],
}

impl WorldgenAssetList {
    pub fn new() -> Result<Self> {
        let trees = [
            WorldgenAsset::from_vox_data(include_bytes!("../../assets/tree.vox"))?
                .with_palette(vec![0, 5, 11]),
            WorldgenAsset::from_vox_data(include_bytes!("../../assets/tree_b.vox"))?
                .with_palette(vec![0, 5, 11]),
            WorldgenAsset::from_vox_data(include_bytes!("../../assets/tree_c.vox"))?
                .with_palette(vec![0, 5, 11]),
        ];

        let bushes = [
            WorldgenAsset::from_vox_data(include_bytes!("../../assets/bush_a.vox"))?
                .with_palette(vec![0, 10, 6]),
            WorldgenAsset::from_vox_data(include_bytes!("../../assets/bush_b.vox"))?
                .with_palette(vec![0, 6, 10]),
            WorldgenAsset::from_vox_data(include_bytes!("../../assets/bush_c.vox"))?
                .with_palette(vec![0, 6, 10]),
        ];

        let rocks = [
            WorldgenAsset::from_vox_data(include_bytes!("../../assets/rock_a.vox"))?
                .with_palette(vec![0, 3]),
            WorldgenAsset::from_vox_data(include_bytes!("../../assets/rock_b.vox"))?
                .with_palette(vec![0, 3, 4]),
            WorldgenAsset::from_vox_data(include_bytes!("../../assets/rock_c.vox"))?
                .with_palette(vec![0, 3, 12]),
        ];

        let spruce_trees =
            [
                WorldgenAsset::from_vox_data(include_bytes!("../../assets/spruce_a.vox"))?
                    .with_palette(vec![0, 5, 11]),
            ];

        Ok(Self {
            trees,
            bushes,
            rocks,
            spruce_trees,
        })
    }
}

pub trait WorldgenAssetBlit {
    fn blit(&mut self, asset: &WorldgenAsset, pos: IVec3);
}

impl WorldgenAssetBlit for ChunkBlockData {
    fn blit(&mut self, asset: &WorldgenAsset, pos: IVec3) {
        for x in 0..asset.size.x {
            for y in 0..asset.size.y {
                for z in 0..asset.size.z {
                    let off = IVec3::new(x, y, z);
                    let pos = pos + off;
                    if (pos.x < 0) || (pos.x >= CHUNK_SIZE as i32) {
                        continue;
                    }
                    if (pos.y < 0) || (pos.y >= CHUNK_SIZE as i32) {
                        continue;
                    }
                    if (pos.z < 0) || (pos.z >= CHUNK_SIZE as i32) {
                        continue;
                    }
                    let block = asset.get_block(off);
                    if block != 0 {
                        self.set_block(block, pos)
                    }
                }
            }
        }
    }
}

impl WorldgenAsset {
    pub fn fits(&self, pos: IVec3) -> bool {
        pos.x > 0
            && pos.y > 0
            && pos.z > 0
            && pos.x + self.size.x < CHUNK_SIZE as i32
            && pos.y + self.size.y < CHUNK_SIZE as i32
            && pos.z + self.size.z < CHUNK_SIZE as i32
    }

    pub fn get_block(&self, pos: IVec3) -> u8 {
        let off = pos.x + (pos.y * self.size.x) + (pos.z * self.size.x * self.size.y);
        self.palette[self.data[off as usize] as usize]
    }

    pub fn set_block(&mut self, pos: IVec3, block: u8) {
        let off = pos.x + (pos.y * self.size.x) + (pos.z * self.size.x * self.size.y);
        self.data[off as usize] = block;
    }

    fn generate_lut(m: &Model) -> Vec<u8> {
        let mut i = 1;
        let mut lut = vec![];
        lut.resize(256, 0);
        m.voxels.iter().for_each(|v| {
            if lut[v.color_index.0 as usize] == 0 {
                lut[v.color_index.0 as usize] = i;
                i += 1;
            }
        });
        lut
    }

    fn fill_with_model(&mut self, model: &Model) {
        let lut = Self::generate_lut(model);
        self.palette.resize(lut.len(), 0);
        model.voxels.iter().for_each(|vox| {
            let block = lut[vox.color_index.0 as usize];
            // We need to rotate the model, at least for models exported via goxel
            let pos = IVec3::new(vox.point.x.into(), vox.point.z.into(), vox.point.y.into());
            self.set_block(pos, block);
        });
    }

    pub fn from_vox_data(data: &[u8]) -> Result<Self> {
        let vox_data = vox_format::from_slice(data)?;
        if let Some(model) = vox_data.models.first() {
            // We need to rotate the model, at least for models exported via goxel
            let size = IVec3::new(
                model.size.x as i32,
                model.size.z as i32,
                model.size.y as i32,
            );
            let palette = vec![];
            let mut data = vec![];
            data.resize((size.x * size.y * size.z) as usize, 0);
            let mut ret = Self {
                data,
                palette,
                size,
            };
            ret.fill_with_model(model);

            Ok(ret)
        } else {
            Err(anyhow!("Couldn't create mesh from .vox"))
        }
    }

    pub fn with_palette(mut self, palette: Vec<u8>) -> Self {
        self.palette = palette;
        self
    }
}
