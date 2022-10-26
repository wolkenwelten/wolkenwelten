/* Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */
pub use self::static_meshes::MeshList;
pub use self::static_shaders::ShaderList;
pub use self::static_textures::TextureList;
use crate::input::InputState;
use crate::meshes::{BlockMesh, TextMesh, Vbo};
use crate::RENDER_DISTANCE;
use glam::f32::Vec3;
use glam::i32::IVec3;
use std::collections::HashMap;
use std::time::Instant;
use wolkenwelten_game::{Character, CHUNK_SIZE};

pub mod static_meshes;
pub mod static_shaders;
pub mod static_textures;

#[derive(Debug)]
pub struct ClientState {
    instant: Instant,

    block_index_buffer: Vbo,
    pub world_mesh: HashMap<IVec3, BlockMesh>,

    window_width: u32,
    window_height: u32,

    pub meshes: MeshList,
    pub shaders: ShaderList,
    pub textures: TextureList,

    pub ui_mesh: TextMesh,

    pub input: InputState,

    cur_fov: f32,
    cur_fps: u32,
    frame_count: u32,
    last_ticks: u128,

    wireframe: bool,
}

impl Default for ClientState {
    fn default() -> Self {
        Self {
            instant: Instant::now(),
            block_index_buffer: BlockMesh::gen_index_buffer(65536 / 4),
            world_mesh: HashMap::new(),

            window_width: 640,
            window_height: 480,

            meshes: MeshList::new(),
            shaders: ShaderList::new(),
            input: InputState::new(),
            textures: TextureList::new(),
            ui_mesh: TextMesh::new(),

            cur_fov: 90.0,
            cur_fps: 0,
            frame_count: 0,
            last_ticks: 0,
            wireframe: false,
        }
    }
}

impl ClientState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn block_indeces(&self) -> &Vbo {
        &self.block_index_buffer
    }

    pub fn fps(&self) -> u32 {
        self.cur_fps
    }
    pub fn calc_fps(&mut self) {
        let ticks = self.instant.elapsed().as_millis();
        if ticks > self.last_ticks + 1000 {
            self.cur_fps =
                (((self.frame_count as f32) / ((ticks - self.last_ticks) as f32)) * 1000.0) as u32;
            self.last_ticks = ticks;
            self.frame_count = 0;
        }
        self.frame_count += 1;
    }

    pub fn gc(&mut self, player: &Character) {
        self.world_mesh.retain(|&pos, _| {
            let diff: Vec3 = (pos.as_vec3() * CHUNK_SIZE as f32) - player.pos;
            let d = diff.dot(diff);
            d < (RENDER_DISTANCE * RENDER_DISTANCE)
        });
    }

    pub fn set_wireframe(&mut self, w: bool) {
        self.wireframe = w;
    }
    pub fn wireframe(&self) -> bool {
        self.wireframe
    }

    pub fn set_fov(&mut self, fov: f32) {
        self.cur_fov = fov;
    }
    pub fn fov(&self) -> f32 {
        self.cur_fov
    }

    pub fn window_size(&self) -> (u32, u32) {
        (self.window_width, self.window_height)
    }
    pub fn set_window_size(&mut self, (w, h): (u32, u32)) {
        self.window_width = w;
        self.window_height = h;
    }
}
