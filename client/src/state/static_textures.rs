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
use crate::{Texture, TextureArray};

pub struct TextureList {
    pub blocks: TextureArray,
    pub gui: Texture,
    pub pear: Texture,
    pub sky: Texture,
}

impl TextureList {
    pub fn new(display: &glium::Display) -> TextureList {
        let blocks =
            TextureArray::from_bytes(display, include_bytes!("../assets/textures/blocks.png"))
                .unwrap();
        let gui =
            Texture::from_bytes(display, include_bytes!("../assets/textures/gui.png")).unwrap();
        let sky =
            Texture::from_bytes(display, include_bytes!("../assets/textures/sky.png")).unwrap();
        let pear: Texture =
            Texture::from_bytes(display, include_bytes!("../assets/textures/pear.png")).unwrap();
        TextureList {
            blocks,
            gui,
            pear,
            sky,
        }
    }
}
