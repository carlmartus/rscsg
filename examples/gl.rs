extern crate lingo;
extern crate rscsg;

use lingo::{draw, gl, window};
use rscsg::dim3::{Csg, Vector};
use std::mem::size_of;

const SHADER_VERT: &'static str = r#"
#version 100
precision mediump float;

attribute vec3 at_loc;
attribute vec4 at_color;

uniform mat4 un_mvp;
varying vec4 va_color;

void main() {
    va_color = at_color;
    gl_Position = un_mvp * vec4(at_loc, 1);
}
"#;

const SHADER_FRAG: &'static str = r#"
#version 100
precision mediump float;

varying vec4 va_color;

void main() {
    float delta = pow(length(va_color), 3.0);
    gl_FragColor = vec4(delta, delta, delta, 1);
}
"#;

#[repr(C, packed)]
struct Vertex(f32, f32, f32, u8, u8, u8, u8);

struct Application {
    win: window::Window,
    prog: draw::Program,
    verts: draw::HwBuf<Vertex>,
    pipeline: draw::Pipeline,
    location_mvp: draw::UniformLocation,
    vertex_count: usize,
}

fn generate_csg_scene() -> Csg {
    Csg::sphere(1.0, 8, 8)
    /*
    Csg::union(
        &Csg::cube(Vector(1., 1., 1.), true),
        &Csg::cube(Vector(1., 1., 1.), false))
        */
}

fn main() {
    match Application::new() {
        Ok(mut a) => a.run(),
        Err(msg) => eprintln!("Error at start: {}", msg),
    }
}

impl Application {
    pub fn new() -> Result<Application, String> {
        println!("Make window");
        let win = window::WindowBuilder::new()
            .with_title("dialog".to_string())
            .build()?;
        draw::print_gl_error()?;

        println!("Make shader");
        let prog = draw::Program::from_static(SHADER_VERT, SHADER_FRAG, &["at_loc", "at_color"])?;
        draw::print_gl_error()?;

        println!("Make verts");
        let triangles = generate_csg_scene().get_triangles();
        let vertex_count = triangles.len() * 3;
        let mut verts = draw::HwBuf::new(vertex_count, draw::Usage::Static)?;
        for triangle in triangles {
            let [p0, p1, p2] = triangle.positions;
            verts.push(Vertex(p0.0, p0.1, p0.2, 1, 0, 0, 0));
            verts.push(Vertex(p1.0, p1.1, p1.2, 0, 1, 0, 0));
            verts.push(Vertex(p2.0, p2.1, p2.2, 0, 0, 1, 0));
        }
        verts.prepear_graphics();
        draw::print_gl_error()?;

        println!("Make attributes");
        let mut pipeline = draw::Pipeline::new(draw::PrimitiveType::Triangles)?;
        let buf_id = pipeline.push_buffer(&verts, size_of::<Vertex>());
        pipeline.push_attribute(buf_id, 3, draw::DataType::F32, false);
        pipeline.push_attribute(buf_id, 4, draw::DataType::U8, false);
        draw::print_gl_error()?;

        println!("Make uniform location");
        let location_mvp = prog.get_uniform_location("un_mvp");
        draw::print_gl_error()?;

        Ok(Application {
            win,
            prog,
            verts,
            pipeline,
            location_mvp,
            vertex_count,
        })
    }

    pub fn run(&mut self) {
        'gameloop: loop {
            self.win.poll_events();
            while let Some(c) = self.win.next_command() {
                match c {
                    window::Command::Quit => break 'gameloop,
                    _ => (),
                }
            }

            unsafe {
                gl::Enable(gl::DEPTH_TEST);
                gl::ClearColor(0.3, 0.4, 0.5, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            }

            let mut mat = draw::Matrix4x4::new();
            mat.camera_3d(
                1.3f32,
                1.3333f32,
                0.1f32,
                20f32,
                draw::Vec3(-1.8, 1., 1.4), // Eye
                draw::Vec3(0., 0., 0.), // At
                draw::Vec3(0f32, 0f32, 1f32),
            ); // Center

            self.prog.use_program();
            self.prog.set_uniform(&self.location_mvp, |loc| unsafe {
                gl::UniformMatrix4fv(loc, 1, gl::FALSE, mat.values.as_ptr());
            });

            self.verts.bind();
            self.pipeline.draw(self.vertex_count);
            self.win.swap_buffers();
            draw::print_gl_error().unwrap();
        }
    }
}
