use gl;
use gl::types::*;
use std;
use std::ffi::{CStr, CString};

pub struct Shader {
    id: GLuint,
}

impl Shader {
    pub fn from_sources(source_vs: &CStr, source_fs: &CStr) -> Result<Shader, String> {
        let prog_id = unsafe { gl::CreateProgram() };

        let vs = compile_shader(source_vs, gl::VERTEX_SHADER).unwrap();
        let fs = compile_shader(source_fs, gl::FRAGMENT_SHADER).unwrap();

        unsafe {
            gl::AttachShader(prog_id, vs);
            gl::AttachShader(prog_id, fs);

            gl::LinkProgram(prog_id);

            gl::DeleteShader(vs);
            gl::DeleteShader(fs);
        }

        let mut success: GLint = 1;
        unsafe {
            gl::GetProgramiv(prog_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(prog_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    prog_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        Ok(Shader { id: prog_id })
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        if self.id != 0 {
            unsafe {
                gl::DeleteShader(self.id);
            }
        }
    }
}

fn compile_shader(source: &CStr, kind: GLenum) -> Result<GLuint, String> {
    let id = unsafe { gl::CreateShader(kind) };

    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    let mut success: GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl::GetShaderInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut GLchar);
        }

        return Err(error.to_string_lossy().into_owned());
    }

    Ok(id)
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}
