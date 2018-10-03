extern crate rscsg;
extern crate lingo;

use rscsg::dim3::{Csg, Vector};

use lingo::window::{Window, Command};
use lingo::shader::{Program, UniformLocation};
use lingo::hwbuf::{HwBuf, Usage};
use lingo::attribute::{Attribute, PrimitiveType, DataType};
use lingo::projection::{Matrix4x4, Vec3};
use lingo::{gl, error};

const SHADER_VERT: &'static str = r#"
#version 100
precision mediump float;

attribute vec3 at_loc;
uniform mat4 un_mvp;

void main() {
    gl_Position = un_mvp * vec4(at_loc, 1);
}
"#;

const SHADER_FRAG: &'static str = r#"
#version 100
precision mediump float;

void main() {
    gl_FragColor = vec4(1, 0, 0, 1);
}
"#;

struct Vertex(f32, f32, f32);

struct Application {
    win: Window,
    prog: Program,
    verts: HwBuf<Vertex>,
    attribs: Attribute,
    location_mvp: UniformLocation,
    vertex_count: usize,
}

fn generate_csg_scene() -> Csg {
    Csg::cube(Vector(1., 1., 1.), true)
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
        let win = Window::new("dialog")?;
        error::print_gl_error()?;

        println!("Make shader");
        let prog = Program::from_static(SHADER_VERT, SHADER_FRAG, &["at_loc"])?;
        error::print_gl_error()?;

        println!("Make verts");
        let triangles = generate_csg_scene().get_triangles();
        let vertex_count = triangles.len()*3;
        let mut verts = HwBuf::new(vertex_count, Usage::Static)?;
        for triangle in triangles {
            for p in triangle.positions.iter() {
                println!("Vertex {}, {}, {}", p.0, p.1, p.2);
                verts.push(Vertex(p.0, p.1, p.2));
            }
        }
        verts.prepear_graphics();
        error::print_gl_error()?;

        println!("Make attributes");
        let mut attribs = Attribute::new(4, PrimitiveType::Triangles)?;
        attribs.push_buffer(verts.get_gl_id());
        attribs.push_attribute(0, 3, DataType::F32, false);
        error::print_gl_error()?;

        println!("Make uniform location");
        let location_mvp = prog.get_uniform_location("un_mvp");
        error::print_gl_error()?;

        Ok(Application {
            win, prog, verts, attribs, location_mvp, vertex_count,
        })
    }

    pub fn run(&mut self) {
        'gameloop: loop {
            self.win.poll_events();
            while let Some(c) = self.win.next_command() {
                match c {
                    Command::Quit =>
                        break 'gameloop,
                    _ => (),
                }
            }

            unsafe {
                gl::ClearColor(0.3, 0.4, 0.5, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            }

            let mut mat = Matrix4x4::new();
            mat.camera_3d(1.3f32, 1.3333f32, 0.1f32, 20f32,
                          Vec3(2f32, 1f32, 1f32), // Eye
                          Vec3(0f32, 0f32, 0f32), // At
                          Vec3(0f32, 0f32, 1f32)); // Center

            self.prog.use_program();
            self.prog.set_uniform(&self.location_mvp, |loc| {
                unsafe {
                    gl::UniformMatrix4fv(loc, 1, gl::FALSE,
                                         mat.values.as_ptr());
                }
            });

            self.verts.bind();
            self.attribs.draw(self.vertex_count);
            self.win.swap_buffers();
            error::print_gl_error().unwrap();
        }
    }
}
