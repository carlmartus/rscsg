mod bsp_node;
mod plane;
mod polygon;
mod vector;
mod vertex;

pub use self::bsp_node::BspNode;
pub use self::plane::Plane;
pub use self::polygon::Polygon;
pub use self::vector::Vector;
pub use self::vertex::Vertex;

use std::f32::consts::PI;

pub type Unit = f32;
pub const UNIT_PI: Unit = PI;
