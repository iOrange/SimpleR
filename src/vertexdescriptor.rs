use gl;
use gl::types::*;

struct VertexElement {
    location: GLuint,
    size: GLint,
    gl_type: GLenum,
    offset: usize,
}

pub struct VertexDescriptor {
    id: GLuint,
    stride: usize,
    elements: Vec<VertexElement>,
}

impl VertexDescriptor {
    pub fn create(stride: usize) -> Result<VertexDescriptor, String> {
        let mut vao: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }

        Ok(VertexDescriptor {
            id: vao,
            stride: stride,
            elements: Vec::new(),
        })
    }

    pub fn add_element(&mut self, location: u32, size: u32, gl_type: GLenum, offset: usize) {
        self.elements.push(VertexElement {
            location: location,
            size: size as GLint,
            gl_type: gl_type,
            offset: offset,
        });
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn setup(&self) {
        for (_, elem) in self.elements.iter().enumerate() {
            unsafe {
                gl::EnableVertexAttribArray(elem.location);
                gl::VertexAttribPointer(
                    elem.location,
                    elem.size,
                    elem.gl_type,
                    gl::FALSE,
                    self.stride as GLsizei,
                    elem.offset as *const GLvoid,
                );
            }
        }
    }
}

impl Drop for VertexDescriptor {
    fn drop(&mut self) {
        if self.id != 0 {
            unsafe {
                gl::DeleteVertexArrays(1, &self.id);
            }
        }
    }
}
