extern crate gl;
extern crate glutin;

use glutin::dpi::*;
use glutin::GlContext;

use std::ffi::CString;
use std::mem::size_of;

mod shader;
use shader::Shader;

mod buffer;
use buffer::{Buffer, BufferKind};

mod vertexdescriptor;
use vertexdescriptor::VertexDescriptor;

const SCREEN_WIDTH: i32 = 1280;
const SCREEN_HEIGHT: i32 = 720;

struct Vertex {
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,

    pub color_x: f32,
    pub color_y: f32,
    pub color_z: f32,
}

impl Vertex {
    fn new(px: f32, py: f32, pz: f32, cx: f32, cy: f32, cz: f32) -> Vertex {
        Vertex {
            pos_x: px,
            pos_y: py,
            pos_z: pz,
            color_x: cx,
            color_y: cy,
            color_z: cz,
        }
    }
}

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("SimpleR")
        .with_dimensions(LogicalSize::new(SCREEN_WIDTH as f64, SCREEN_HEIGHT as f64));

    let context = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3)))
        .with_gl_profile(glutin::GlProfile::Core)
        .with_vsync(true);

    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    unsafe {
        gl_window.make_current().unwrap();
    }

    unsafe {
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

        gl::ClearColor(0.412, 0.796, 1.0, 1.0);
        gl::Viewport(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);
    }

    let vertices: Vec<Vertex> = vec![
        Vertex::new(-0.5, -0.5, 0.0, 1.0, 0.0, 0.0),
        Vertex::new(0.5, -0.5, 0.0, 0.0, 1.0, 0.0),
        Vertex::new(0.0, 0.5, 0.0, 0.0, 0.0, 1.0),
    ];
    let vb_size = vertices.len() * size_of::<Vertex>();

    let vb = Buffer::create(BufferKind::Vertex, vb_size).unwrap();

    vb.set_data(0, vb_size, vertices.as_ptr() as *const gl::types::GLvoid);

    let mut vao = VertexDescriptor::create(size_of::<Vertex>()).unwrap();
    vao.add_element(0, 3, gl::FLOAT, 0);
    vao.add_element(1, 3, gl::FLOAT, 3 * size_of::<f32>());

    let shader = Shader::from_sources(
        &CString::new(include_str!("triangle_vs.glsl")).unwrap(),
        &CString::new(include_str!("triangle_fs.glsl")).unwrap(),
    )
    .unwrap();

    shader.bind();

    let mut running = true;
    while running {
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => running = false,
                glutin::WindowEvent::Resized(logical_size) => {
                    let dpi_factor = gl_window.get_hidpi_factor();
                    gl_window.resize(logical_size.to_physical(dpi_factor));
                }
                _ => (),
            },
            _ => (),
        });

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);

            vao.bind();
            vb.bind();
            vao.setup();

            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        gl_window.swap_buffers().unwrap();
    }
}
