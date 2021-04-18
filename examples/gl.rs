extern crate glfw;
extern crate lingo;
extern crate rscsg;

use glfw::{Context, WindowEvent};
use lingo::{draw, gl};
use rscsg::dim3::{Csg, Vector};
use std::mem::size_of;
use std::sync::mpsc::Receiver;

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
    glfw_ctx: glfw::Glfw,
    win: glfw::Window,
    events: Receiver<(f64, WindowEvent)>,
    prog: draw::Program,
    location_mvp: draw::UniformLocation,
    scene: Scene,
    input: AppInput,
}

type SceneGenerator = fn(i32) -> Csg;
const SCENES: [(&str, SceneGenerator); 4] = [
    ("Cube", scene_cube as SceneGenerator),
    ("Cubes", scene_cubes as SceneGenerator),
    ("Cubes subtract", scene_cubes_difference as SceneGenerator),
    ("Cut cube", scene_cut_cube as SceneGenerator),
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

fn scene_cubes_difference(step: i32) -> Csg {
    let rotate = 30. + (step * 4) as f32;
    Csg::subtract(
        &Csg::cube(Vector(1., 1., 1.), true).rotate(Vector(1., 0., 0.), rotate),
        &Csg::cube(Vector(1., 1., 1.), false),
    )
}

fn scene_cut_cube(step: i32) -> Csg {
    let cut_x = 1. + 0.1 * step as f32;
    Csg::union(
        &Csg::cube(Vector(2., 2., 2.), true),
        &Csg::cube(Vector(1., 3., 3.), false).translate(Vector(cut_x, -1.5, -1.5)),
    )
}

fn main() {
    eprintln!(" - Rotate the view with the mouse.\n - Change between scenes by pressing the numbers [1-4].\n - Animate the scene with [j/k].");

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
        let glfw_ctx = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let (mut win, events) = glfw_ctx
            .create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        win.set_char_polling(true);
        win.set_cursor_pos_polling(true);
        win.set_mouse_button_polling(true);
        win.set_size_polling(true);
        win.make_current();

        gl::load_with(|s| win.get_proc_address(s) as *const _);
        draw::print_gl_error()?;

        // Make shader
        let prog = draw::Program::from_static(SHADER_VERT, SHADER_FRAG, &["at_loc", "at_color"])?;
        draw::print_gl_error()?;

        let scene = make_scene(scene_cube(0))?;

        // Make uniform location
        let location_mvp = prog.get_uniform_location("un_mvp");
        draw::print_gl_error()?;

        Ok(Application {
            glfw_ctx,
            win,
            events,
            prog,
            location_mvp,
            scene,
            input: Default::default(),
        })
    }

    pub fn run(mut self) {
        'gameloop: loop {
            self.glfw_ctx.wait_events();

            let event_iter = glfw::flush_messages(&self.events);
            for (_, event) in event_iter {
                eprintln!("Loop event");
                match event {
                    WindowEvent::Size(w, h) => unsafe {
                        eprintln!("Resize {}x{}", w, h);
                        gl::Viewport(0, 0, w, h);
                    },
                    WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                        self.win.set_should_close(true)
                    }
                    WindowEvent::Char(ch) => {
                        eprintln!("CHAR {}", ch);
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
                    WindowEvent::CursorPos(x, y) => {
                        let x = x as f32;
                        let y = y as f32;
                        eprintln!("Cursor @ {}, {}", x, y);
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
