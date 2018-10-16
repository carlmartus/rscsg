use dim3::{Polygon, Vector, Vertex};
use {Unit, EPSILON};

bitflags! {
    struct Location: u32 {
        const NONE = 0;
        const COPLANAR = 0;
        const FRONT = 1;
        const BACK = 2;
        const SPANNING = 3;
    }
}

type Collector = Vec<Polygon>;

/// Represents a plane in 3D space.
#[derive(Clone, Debug)]
pub struct Plane(pub Vector, pub Unit);

impl Plane {
    pub fn from_points(v0: Vector, v1: Vector, v2: Vector) -> Plane {
        let n = (v1 - v0).cross(v2 - v0).normalize();
        Plane(n, n.dot(v0))
    }

    pub fn flip(&self) -> Plane {
        Plane(self.0.negate(), -self.1)
    }

    /// Split `polygon` by this plane if needed, then put the polygon or polygon fragments in the
    /// appropriate lists. Coplanar polygons go into either `coplanarFront` or `coplanarBack`
    /// depending on their orientation with respect to this plane. Polygons in front or in back of
    /// this plane go into either `front` or `back`
    pub fn split_polygon(
        &self,
        poly: &Polygon,
        coplane_front: &mut Collector,
        coplane_back: &mut Collector,
        front: &mut Collector,
        back: &mut Collector,
    ) {
        let mut polygon_type = Location::NONE;
        let mut vertex_locs: Vec<Location> = Vec::with_capacity(poly.vertices.len());
        let vertices_num = poly.vertices.len();

        for v in poly.vertices.iter() {
            let t = self.0.dot(v.position) - self.1;

            let loc = {
                if t < -EPSILON {
                    Location::BACK
                } else if t > EPSILON {
                    Location::FRONT
                } else {
                    Location::COPLANAR
                }
            };

            polygon_type = polygon_type | loc;
            vertex_locs.push(loc);
        }

        match polygon_type {
            Location::COPLANAR => {
                if self.0.dot(poly.plane.0) > (0 as Unit) {
                    coplane_front.push(poly.clone());
                } else {
                    coplane_back.push(poly.clone());
                }
            }
            Location::FRONT => {
                front.push(poly.clone());
            }
            Location::BACK => {
                back.push(poly.clone());
            }
            Location::SPANNING => {
                let mut f: Vec<Vertex> = Vec::new();
                let mut b: Vec<Vertex> = Vec::new();

                for (i, v) in poly.vertices.iter().enumerate() {
                    let j = (i + 1) % vertices_num;
                    let ti = vertex_locs[i];
                    let tj = vertex_locs[j];
                    let vi = v.clone();
                    let vj = poly.vertices[j];

                    if ti != Location::BACK {
                        f.push(vi);
                    }

                    if ti != Location::FRONT {
                        b.push(vi);
                    }

                    if (ti | tj) == Location::SPANNING {
                        let t = (self.1 - self.0.dot(vi.position))
                            / self.0.dot(vj.position - vi.position);

                        let v = vi.interpolate(vj, t);
                        f.push(v);
                        b.push(v);
                    }
                }

                if f.len() >= 3 {
                    front.push(Polygon::new(f));
                }

                if b.len() >= 3 {
                    back.push(Polygon::new(b));
                }
            }
            _ => (),
        }
    }
}
