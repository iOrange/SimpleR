use gl;
use gl::types::*;
use std;

pub enum BufferKind {
    Vertex,
    Index,
    Constants,
}

pub struct Buffer {
    kind: BufferKind,
    size: usize,
    id: GLuint,
    target: GLenum,
}

impl Buffer {
    pub fn create(kind: BufferKind, size: usize) -> Result<Buffer, String> {
        let gl_target = match kind {
            BufferKind::Vertex => gl::ARRAY_BUFFER,
            BufferKind::Index => gl::ELEMENT_ARRAY_BUFFER,
            BufferKind::Constants => gl::UNIFORM_BUFFER,
        };

        let mut buf_id: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut buf_id);
            gl::BindBuffer(gl_target, buf_id);
            gl::BufferData(
                gl_target,
                size as GLsizeiptr,
                std::ptr::null(),
                gl::DYNAMIC_DRAW,
            );
            gl::BindBuffer(gl_target, 0);
        }

        Ok(Buffer {
            kind: kind,
            size: size,
            id: buf_id,
            target: gl_target,
        })
    }

    pub fn set_data(&self, offset: usize, size: usize, data: *const GLvoid) {
        if self.id != 0 {
            unsafe {
                gl::BindBuffer(self.target, self.id);
                gl::BufferSubData(self.target, offset as GLintptr, size as GLsizeiptr, data);
                gl::BindBuffer(self.target, 0);
            }
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.target, self.id);
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        if self.id != 0 {
            unsafe {
                gl::DeleteBuffers(1, &self.id);
            }
        }
    }
}
