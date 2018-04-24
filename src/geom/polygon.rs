use geom::{Vertex, Plane};

#[derive(Clone)]
pub struct Polygon {
    //vertices: [Vertex; 3],
    pub vertices: Vec<Vertex>,
    pub plane: Plane,
}

impl Polygon {
    pub fn new(vertices: Vec<Vertex>) -> Polygon {
        let plane = Plane::from_points(
            vertices[0].position,
            vertices[1].position,
            vertices[2].position);

        Polygon {
            vertices, plane,
        }
    }

    pub fn flip(&mut self) {
        // Reverse slice item order
        let swap_0 = self.vertices[0].clone();
        self.vertices[0] = self.vertices[2].clone();
        self.vertices[2] = swap_0;

        for v in self.vertices.iter_mut() {
            *v = v.flip();
        }
    }
}
