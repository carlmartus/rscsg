extern crate png;
extern crate smallpt;

use smallpt::{saturate, tonemap, trace, Camera, Float3, Material, Rectangle, Scene, BSDF};

const WIDTH: usize = 256;
const HEIGHT: usize = WIDTH;

struct BitmapColor(u8, u8, u8);

fn main() {
    println!("Ray trace sample");

    let mut backbuffer = vec![Float3::zero(); WIDTH * HEIGHT];
    let mut scene = Scene::init();

    scene.add(Box::new(Rectangle::new(
        Float3::new(50.0, 81.5, 50.0),
        Float3::new(0.0, -1.0, 0.0),
        Float3::new(1.0, 0.0, 0.0),
        Float3::new(0.0, 0.0, 1.0),
        33.0,
        33.0,
        Material::new(Float3::new(12.0, 12.0, 12.0), Float3::zero(), BSDF::Diffuse),
    )));

    let aperture = 0.5135;
    let camera_origin = Float3::new(50.0, 50.0, 300.0);
    let camera_direction = Float3::new(0.0, -0.05, -1.0).normalize();
    let camera_right = Float3::new(WIDTH as f32 * aperture / HEIGHT as f32, 0.0, 0.0);
    let camera_up = camera_right.cross(camera_direction).normalize() * aperture;

    let camera = Camera::new(camera_origin, camera_direction, camera_right, camera_up);

    let mut num_rays = 0;
    trace(
        &scene,
        &camera,
        WIDTH,
        HEIGHT,
        40,
        &mut backbuffer,
        &mut num_rays,
    );

    //let bitmap: [u8; WIDTH * HEIGHT * 4] = backbuffer
    let bitmap: Vec<u8> = backbuffer
        .iter()
        .flat_map(|&comp| {
            let adjust = saturate(tonemap(comp));
            [
                (adjust.get_x() * 255.0).round() as u8,
                (adjust.get_y() * 255.0).round() as u8,
                (adjust.get_z() * 255.0).round() as u8,
                0xffu8,
            ].into_iter()
        })
        .map(|&x| x)
        .collect();
}
