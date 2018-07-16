/*
 * Example using *RsCSG* with *smallpt* (raytracer).
 *
 * Warning, the output of this example is not good looking =/
 */

extern crate png;
extern crate rscsg;
extern crate smallpt;

use rscsg::core::Csg;
use rscsg::geom::Vector;

use png::HasParameters;
use smallpt::{
    saturate, tonemap, trace, Camera, Float3, Material, Rectangle, Scene, Triangle, BSDF,
};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

const WIDTH: usize = 80;
const HEIGHT: usize = WIDTH;

const PNG_OUTPUT: &str = "/tmp/rscsg_raytrace.png";

fn main() {
    println!("Creating scene...");

    let mut backbuffer = vec![Float3::zero(); WIDTH * HEIGHT];
    let scene = create_scene();

    let aperture = 0.5135;
    let camera_origin = Float3::new(50.0, 50.0, 300.0);
    let camera_direction = Float3::new(0.0, -0.05, -1.0).normalize();
    let camera_right = Float3::new(WIDTH as f32 * aperture / HEIGHT as f32, 0.0, 0.0);
    let camera_up = camera_right.cross(camera_direction).normalize() * aperture;

    let camera = Camera::new(camera_origin, camera_direction, camera_right, camera_up);

    let mut num_rays = 0;
    println!("Ray tracing...");
    trace(
        &scene,
        &camera,
        WIDTH,
        HEIGHT,
        800,
        &mut backbuffer,
        &mut num_rays,
    );

    println!("Creating bitmap...");
    let bitmap: Vec<u8> = backbuffer
        .iter()
        .flat_map(|&comp| {
            let adjust = saturate(tonemap(comp));
            vec![
                (adjust.get_x() * 255.0).round() as u8,
                (adjust.get_y() * 255.0).round() as u8,
                (adjust.get_z() * 255.0).round() as u8,
                0xffu8,
            ]
        })
        .collect();

    println!("Exporting png ({})...", PNG_OUTPUT);
    let path = Path::new(PNG_OUTPUT);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, WIDTH as u32, HEIGHT as u32);
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&bitmap).unwrap();

    println!("Done");
}

fn create_scene() -> Scene {
    let cube = Csg::cube(Vector(80., 80., 80.), true);

    let mut scene = Scene::init();

    let triangle_material =
        Material::new(Float3::zero(), Float3::new(0.4, 0.4, 1.0), BSDF::Diffuse);

    cube.iter_triangles(|tri| {
        scene.add(Box::new(Triangle::new(
            vec_to_float3(tri.positions[0]),
            vec_to_float3(tri.positions[1]),
            vec_to_float3(tri.positions[2]),
            triangle_material,
        )));
    });

    // Light
    scene.add(Box::new(Rectangle::new(
        Float3::new(50.0, 81.5, 50.0),
        Float3::new(0.0, -1.0, 0.0),
        Float3::new(1.0, 0.0, 0.0),
        Float3::new(0.0, 0.0, 1.0),
        33.0,
        33.0,
        Material::new(Float3::new(12.0, 12.0, 12.0), Float3::zero(), BSDF::Diffuse),
    )));

    return scene;
}

fn vec_to_float3(v: Vector) -> Float3 {
    Float3::new(v.0, v.1, v.2)
}
