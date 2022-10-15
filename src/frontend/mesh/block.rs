use std::ffi::CString;
use super::vertex::vertex_attrib_int_pointer;
use gl::types::{GLuint, GLvoid};

pub struct BlockMesh {
	vao: GLuint,
	vbo: GLuint,
	vertex_count: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct BlockVertex {
	xyz: u16, // We've got 1 bit left here
	texture_index: u8, // Right now we don't really use 256 distinct block faces, ~32 should suffice for a long time
	side_and_light: u8, // And another one here as well
}

#[derive(Copy, Clone)]
enum Side {
	Front = 0,
	Back,
	Top,
	Bottom,
	Left,
	Right,
}

impl From<Side> for u8 {
	fn from(item: Side) -> Self { item as u8 }
}

impl BlockVertex {
	fn new(x:u16, y:u16, z:u16, texture_index:u8, side:u8, light:u8) -> Self {
		let xyz:u16 = x | (y << 5) | (z << 10);
		let side_and_light = side | (light << 4);
		Self { xyz, texture_index, side_and_light }
	}

	fn add_front(vertices:&mut Vec<Self>, x:u16, y:u16, z:u16, w:u16, h:u16, d:u16, texture_index:u8, light:u8) {
		let side:u8 = Side::Front.into();
		let z = z + d;
		vertices.push(Self::new(x,y,z,texture_index,side,light));
		vertices.push(Self::new(x+w,y,z,texture_index,side,light));
		vertices.push(Self::new(x+w,y+h,z,texture_index,side,light));

		vertices.push(Self::new(x+w,y+h,z,texture_index,side,light));
		vertices.push(Self::new(x,y+h,z,texture_index,side,light));
		vertices.push(Self::new(x,y,z,texture_index,side,light));
	}

	fn add_back(vertices:&mut Vec<Self>, x:u16, y:u16, z:u16, w:u16, h:u16, _d:u16, texture_index:u8, light:u8) {
		let side:u8 = Side::Back.into();
		vertices.push(Self::new(x,y,z,texture_index,side,light));
		vertices.push(Self::new(x,y+h,z,texture_index,side,light));
		vertices.push(Self::new(x+w,y+h,z,texture_index,side,light));

		vertices.push(Self::new(x+w,y+h,z,texture_index,side,light));
		vertices.push(Self::new(x+w,y,z,texture_index,side,light));
		vertices.push(Self::new(x,y,z,texture_index,side,light));
	}

	fn add_top(vertices:&mut Vec<Self>, x:u16, y:u16, z:u16, w:u16, h:u16, d:u16, texture_index:u8, light:u8) {
		let side:u8 = Side::Top.into();
		let y = y + h;
		vertices.push(Self::new(x,y,z,texture_index,side,light));
		vertices.push(Self::new(x,y,z+d,texture_index,side,light));
		vertices.push(Self::new(x+w,y,z+d,texture_index,side,light));

		vertices.push(Self::new(x+w,y,z+d,texture_index,side,light));
		vertices.push(Self::new(x+w,y,z,texture_index,side,light));
		vertices.push(Self::new(x,y,z,texture_index,side,light));
	}

	fn add_bottom(vertices:&mut Vec<Self>, x:u16, y:u16, z:u16, w:u16, _h:u16, d:u16, texture_index:u8, light:u8) {
		let side:u8 = Side::Bottom.into();
		vertices.push(Self::new(x,y,z,texture_index,side,light));
		vertices.push(Self::new(x+w,y,z,texture_index,side,light));
		vertices.push(Self::new(x+w,y,z+d,texture_index,side,light));

		vertices.push(Self::new(x+w,y,z+d,texture_index,side,light));
		vertices.push(Self::new(x,y,z+d,texture_index,side,light));
		vertices.push(Self::new(x,y,z,texture_index,side,light));
	}

	fn add_left(vertices:&mut Vec<Self>, x:u16, y:u16, z:u16, _w:u16, h:u16, d:u16, texture_index:u8, light:u8) {
		let side:u8 = Side::Left.into();
		vertices.push(Self::new(x,y,z,texture_index,side,light));
		vertices.push(Self::new(x,y,z+d,texture_index,side,light));
		vertices.push(Self::new(x,y+h,z+d,texture_index,side,light));

		vertices.push(Self::new(x,y+h,z+d,texture_index,side,light));
		vertices.push(Self::new(x,y+h,z,texture_index,side,light));
		vertices.push(Self::new(x,y,z,texture_index,side,light));
	}

	fn add_right(vertices:&mut Vec<Self>, x:u16, y:u16, z:u16, w:u16, h:u16, d:u16, texture_index:u8, light:u8) {
		let side:u8 = Side::Right.into();
		let x = x + w;
		vertices.push(Self::new(x,y,z,texture_index,side,light));
		vertices.push(Self::new(x,y+h,z,texture_index,side,light));
		vertices.push(Self::new(x,y+h,z+d,texture_index,side,light));

		vertices.push(Self::new(x,y+h,z+d,texture_index,side,light));
		vertices.push(Self::new(x,y,z+d,texture_index,side,light));
		vertices.push(Self::new(x,y,z,texture_index,side,light));
	}

	fn vertex_attrib_pointers() {
		let stride = std::mem::size_of::<Self>();
		unsafe {
			let offset = vertex_attrib_int_pointer(stride, 0, 0, gl::UNSIGNED_SHORT, 2);
			let offset = vertex_attrib_int_pointer(stride, 1, offset,gl::UNSIGNED_BYTE, 1);
			vertex_attrib_int_pointer(stride, 2, offset, gl::UNSIGNED_BYTE, 1);
		}
	}
}

impl BlockMesh {
	pub fn draw(&self) {
		unsafe {
			gl::BindVertexArray(self.vao);
			gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count.try_into().unwrap());
		}
	}

	fn new(vertices: &Vec<BlockVertex>) -> Result<Self, String> {
		let vbo_size = (vertices.len() * std::mem::size_of::<BlockVertex>()) as gl::types::GLsizeiptr;
		let vertex_count: GLuint = vertices.len().try_into().unwrap();
		let i:u32 = 0;

		let label = CString::new(format!("BlockMesh VAO {i}")).unwrap();
		let vao = unsafe {
			let mut vao = 0;
			gl::GenVertexArrays(1, &mut vao);
			gl::BindVertexArray(vao);
			gl::ObjectLabel(gl::ARRAY_BUFFER, vao, -1, label.as_ptr());
			vao
		};

		let label = CString::new(format!("BlockMesh VBO {i}")).unwrap();
		let vbo = unsafe {
			let mut vbo:GLuint = 0;
			gl::GenBuffers(1, &mut vbo);
			gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
			gl::BufferData(
				gl::ARRAY_BUFFER,
				vbo_size.try_into().unwrap(),
				vertices.as_ptr() as *const GLvoid,
				gl::STATIC_DRAW,
			);
			gl::ObjectLabel(gl::BUFFER, vbo, -1, label.as_ptr());
			BlockVertex::vertex_attrib_pointers();
			vbo
		};

		let _vbo_size:u16 = vbo_size.try_into().unwrap();
		let vertex_count:u16 = vertex_count.try_into().unwrap();
		Ok(Self { vao, vbo, vertex_count })
	}

	pub fn test_mesh() -> Self {
		let mut vertices:Vec<BlockVertex> = Vec::with_capacity(512);
		for y in 0..16 {
			for x in 0..16 {
				BlockVertex::add_front(&mut vertices, x, y, 0, 1, 1, 1, (x + (y * 16)).try_into().unwrap(), 0x0F);
				BlockVertex::add_back(&mut vertices, x, y, 16, 1, 1, 1, (x + (y * 16)).try_into().unwrap(), 0x0F);
				BlockVertex::add_top(&mut vertices, x, 0, y, 1, 1, 1, (x + (y * 16)).try_into().unwrap(), 0x0F);
				BlockVertex::add_bottom(&mut vertices, x, 16, y, 1, 1, 1, (x + (y * 16)).try_into().unwrap(), 0x0F);
				BlockVertex::add_left(&mut vertices, 16, x, y, 1, 1, 1, (x + (y * 16)).try_into().unwrap(), 0x0F);
				BlockVertex::add_right(&mut vertices, 0, x, y, 1, 1, 1, (x + (y * 16)).try_into().unwrap(), 0x0F);
			}
		}
		Self::new(&vertices).unwrap()
	}
}

impl Drop for BlockMesh {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteBuffers(1, &mut self.vbo);
			gl::DeleteVertexArrays(1, &mut self.vao);
		}
	}
}