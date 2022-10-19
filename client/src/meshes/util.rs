use crate::can_use_object_labels;
use gl::types::{GLuint, GLvoid};
use std::ffi::CString;

#[derive(Debug, Default)]
pub struct Vbo {
    id: GLuint,
}

#[derive(Debug, Default)]
pub struct Vao {
    id: GLuint,
    vbo: Vbo,
}

impl Vbo {
    pub fn buffer_data(vertices: *const GLvoid, vbo_size: u32) {
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,             // target
                vbo_size.try_into().unwrap(), // size of data in bytes
                vertices,                     // pointer to data
                gl::STATIC_DRAW,              // usage
            )
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    pub fn new(label: &str, vertices: *const GLvoid, vbo_size: u32) -> Self {
        let id: GLuint = unsafe {
            let mut vbo: GLuint = 0;
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            vbo
        };
        let label = CString::new(format!("{} VBO {}", label, id)).unwrap();
        if can_use_object_labels() {
            unsafe {
                gl::ObjectLabel(gl::BUFFER, id, -1, label.as_ptr());
            }
        }
        Self::buffer_data(vertices, vbo_size);
        Self { id }
    }
}

impl Vao {
    pub fn new(label: &str, vertices: *const GLvoid, vbo_size: u32) -> Self {
        let id: GLuint = unsafe {
            let mut vao: GLuint = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            vao
        };
        let vao_label = CString::new(format!("{} VAO {}", label, id)).unwrap();
        if can_use_object_labels() {
            unsafe {
                gl::ObjectLabel(gl::VERTEX_ARRAY, id, -1, vao_label.as_ptr());
            }
        }
        let vbo = Vbo::new(label, vertices, vbo_size);
        Self { id, vbo }
    }

    pub fn new_empty(label: &str) -> Self {
        Self::new(label, std::ptr::null::<GLvoid>(), 0)
    }

    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.id) }
        self.vbo.bind();
    }

    pub fn draw(&self, vertex_count: u32) {
        self.bind();
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, vertex_count.try_into().unwrap());
        }
    }
}

impl Drop for Vao {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &self.id) }
    }
}
impl Drop for Vbo {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.id) }
    }
}

use gl;

pub unsafe fn vertex_attrib_pointer(
    stride: usize,
    location: usize,
    offset: usize,
    components: usize,
    data_type: u32,
    size: usize,
    normalized: bool,
) -> usize {
    gl::EnableVertexAttribArray(location as gl::types::GLuint);
    gl::VertexAttribPointer(
        location as gl::types::GLuint,
        components as i32, // the number of components per generic vertex attribute
        data_type,         // data type
        if normalized { gl::TRUE } else { gl::FALSE }, // normalized (int-to-float conversion)
        stride as gl::types::GLint,
        offset as *const gl::types::GLvoid,
    );
    offset + (size * components)
}

pub unsafe fn vertex_attrib_int_pointer(
    stride: usize,
    location: usize,
    offset: usize,
    data_type: u32,
    size: usize,
) -> usize {
    gl::EnableVertexAttribArray(location as gl::types::GLuint);
    gl::VertexAttribIPointer(
        location as gl::types::GLuint,
        1,
        data_type, // data type
        stride as gl::types::GLint,
        offset as *const gl::types::GLvoid,
    );
    offset + size
}
