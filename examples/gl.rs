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

struct Scene {
    verts: draw::HwBuf<Vertex>,
    pipeline: draw::Pipeline,
    vertex_count: usize,
}

#[derive(Default)]
struct AppInput {
    button: bool,
    last_x: f32,
    last_y: f32,
    cam_x: f32,
    cam_y: f32,
    scene_id: usize,
    step: i32,
}

struct Application {
    win: window::Window,
    prog: draw::Program,
    location_mvp: draw::UniformLocation,
    scene: Scene,
    input: AppInput,
}

type SceneGenerator = fn(i32) -> Csg;
const SCENES: [(&str, SceneGenerator); 2] = [
    ("Cube", scene_cube as SceneGenerator),
    ("Cubes", scene_cubes as SceneGenerator),
];

fn scene_cube(_step: i32) -> Csg {
    Csg::cube(Vector(1., 1., 1.), true)
}

fn scene_cubes(step: i32) -> Csg {
    let rotate = 30. + (step * 4) as f32;
    Csg::union(
        &Csg::cube(Vector(1., 1., 1.), true).rotate(Vector(1., 0., 0.), rotate),
        &Csg::cube(Vector(1., 1., 1.), false),
    )
}

fn main() {
    match Application::new() {
        Ok(a) => a.run(),
        Err(msg) => eprintln!("Error at start: {}", msg),
    }
}

fn make_scene(csg: Csg) -> Result<Scene, String> {
    let triangles = csg.get_triangles();
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

    let mut pipeline = draw::Pipeline::new(draw::PrimitiveType::Triangles)?;
    let buf_id = pipeline.push_buffer(&verts, size_of::<Vertex>());
    pipeline.push_attribute(buf_id, 3, draw::DataType::F32, false);
    pipeline.push_attribute(buf_id, 4, draw::DataType::U8, false);
    draw::print_gl_error()?;

    Ok(Scene {
        verts,
        pipeline,
        vertex_count,
    })
}

impl Application {
    pub fn new() -> Result<Application, String> {
        // Make window
        let win = window::WindowBuilder::new()
            .with_title("dialog".to_string())
            .build()?;
        draw::print_gl_error()?;

        // Make shader
        let prog = draw::Program::from_static(SHADER_VERT, SHADER_FRAG, &["at_loc", "at_color"])?;
        draw::print_gl_error()?;

        let scene = make_scene(scene_cube(0))?;

        // Make uniform location
        let location_mvp = prog.get_uniform_location("un_mvp");
        draw::print_gl_error()?;

        Ok(Application {
            win,
            prog,
            location_mvp,
            scene,
            input: Default::default(),
        })
    }

    pub fn run(mut self) {
        'gameloop: loop {
            self.win.poll_events();

            while let Some(c) = self.win.next_command() {
                match c {
                    window::Command::Quit => break 'gameloop,
                    window::Command::TypeCharacter(ch) => {
                        if ch >= '1' && (ch as usize) < ('1' as usize + SCENES.len()) {
                            let id = (ch as usize - '1' as usize) as usize;
                            self.load_scene(id);
                        } else {
                            match ch {
                                'q' => break 'gameloop,
                                'j' => {
                                    self.input.step += 1;
                                    let scene_id = self.input.scene_id;
                                    self.load_scene(scene_id);
                                }
                                'k' => {
                                    self.input.step -= 1;
                                    let scene_id = self.input.scene_id;
                                    self.load_scene(scene_id);
                                }
                                _ => (),
                            };
                        }
                    }
                    _ => (),
                }
            }

            while let Some(p) = self.win.next_peripheral() {
                match p.event {
                    window::PeripheralEvent::MousePosition(x, y) => {
                        if self.input.button {
                            let dx = x - self.input.last_x;
                            let dy = y - self.input.last_y;

                            self.input.cam_x += dx * 0.01;
                            self.input.cam_y += dy * 0.01;

                            const Y_LIMIT: f32 = std::f32::consts::PI / 2.0 - 0.01;
                            if self.input.cam_y > Y_LIMIT {
                                self.input.cam_y = Y_LIMIT;
                            } else if self.input.cam_y < -Y_LIMIT {
                                self.input.cam_y = -Y_LIMIT;
                            }
                        }
                        self.input.last_x = x;
                        self.input.last_y = y;
                    }
                    window::PeripheralEvent::Button(id, press) => {
                        if let window::ButtonId::Mouse(_) = id {
                            self.input.button = press;
                        }
                    }
                    _ => (),
                }
            }

            unsafe {
                gl::Enable(gl::DEPTH_TEST);
                gl::ClearColor(0.3, 0.4, 0.5, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            }

            self.update_camera();

            self.scene.verts.bind();
            self.scene.pipeline.draw(self.scene.vertex_count);
            self.win.swap_buffers();
            draw::print_gl_error().unwrap();
        }
    }

    fn change_csg(&mut self, csg: Csg) {
        self.scene = make_scene(csg).unwrap();
    }

    fn load_scene(&mut self, id: usize) {
        let (name, generator) = &SCENES[id];
        let step = self.input.step;
        let csg = generator(step);
        let triangle_count = csg.get_triangles_count();

        println!(
            "Loading scene \"{}\", with step {} and {} triangles",
            name, step, triangle_count
        );
        self.change_csg(csg);
        self.input.scene_id = id;
    }

    fn update_camera(&mut self) {
        let mut mat = draw::Matrix4x4::new();

        let z_mul = self.input.cam_y.cos();
        mat.camera_3d(
            1.3f32,
            1.3333f32,
            0.1f32,
            20f32,
            draw::Vec3(
                4.0 * self.input.cam_x.cos() * z_mul,
                4.0 * self.input.cam_x.sin() * z_mul,
                4.0 * self.input.cam_y.sin(),
            ), // Eye
            draw::Vec3(0., 0., 0.),       // At
            draw::Vec3(0f32, 0f32, 1f32), // Center
        );

        self.prog.use_program();
        self.prog.set_uniform(&self.location_mvp, |loc| unsafe {
            gl::UniformMatrix4fv(loc, 1, gl::FALSE, mat.values.as_ptr());
        });
    }
}
