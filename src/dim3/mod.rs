mod bsp_node;
mod csg;
mod cube;
mod plane;
mod polygon;
mod sphere;
mod vector;
mod vertex;

pub use self::bsp_node::BspNode;
pub use self::csg::Csg;
pub use self::plane::Plane;
pub use self::polygon::Polygon;
pub use self::vector::{IVector, Vector};
pub use self::vertex::Vertex;

pub struct Triangle {
    pub positions: [Vector; 3],
    pub normal: Vector,
}
