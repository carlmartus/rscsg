use geom::{Unit, Vector};

#[derive(Clone)]
pub struct Vertex {
    position: Vector,
    normal: Vector,
}

impl Vertex {
    pub fn new(position: Vector, normal: Vector) -> Vertex {
        Vertex { position, normal }
    }

    pub fn flip(&mut self) {
        self.normal = self.normal.negate();
    }

    pub fn interpolate(&self, other: Vertex, t: Unit) -> Vertex {
        Vertex::new(
            self.position.lerp(other.position, t),
            self.normal.lerp(other.normal, t))
    }
}
