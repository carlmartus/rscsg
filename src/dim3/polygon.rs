use dim3::{Plane, Vertex};

/// Represents a convex polygon. The vertices used to initialize a polygon must be coplanar and
/// form a convex loop. They do not have to be `Vertex` instances but they must behave similarly
/// (duck typing can be used for customization).
///
/// Each convex polygon has a `shared` property, which is shared between all polygons that are
/// clones of each other or were split from the same polygon.  This can be used to define
/// per-polygon properties (such as surface color).

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
            vertices[2].position,
        );

        Polygon { vertices, plane }
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
