use dim3::{BspNode, Polygon, Triangle, Vector, Vertex, Plane};
use Unit;

#[derive(Clone)]
pub struct Csg {
    pub polygons: Vec<Polygon>,
}

impl Csg {
    pub fn new() -> Csg {
        Csg {
            polygons: Vec::new(),
        }
    }

    pub fn from_polygons(polygons: Vec<Polygon>) -> Csg {
        Csg { polygons }
    }

    pub fn to_polygons(&self) -> Vec<Polygon> {
        self.polygons.clone()
    }

    pub fn refine(&self) -> Csg {
        let mut new_csg = Csg::new();

        for poly in &self.polygons {
            if poly.vertices.len() == 0 {
                continue;
            }

            let mid_pos = poly
                .vertices
                .iter()
                .fold(Vector(0f32, 0f32, 0f32), |acc, v| acc + v.position)
                / (poly.vertices.len() as f32);
            let mid_nor = poly.vertices[0].normal;

            let mid_vert = Vertex::new(mid_pos, mid_nor);

            let len_verts = poly.vertices.len();
            let mut gen_verts: Vec<Vertex> = (0..len_verts)
                .map(|i| poly.vertices[i].interpolate(poly.vertices[(i + 1) % len_verts], 0.5))
                .collect();

            let mut new_verts = poly.vertices.clone();
            new_verts.append(&mut gen_verts);
            new_verts.push(mid_vert);

            for i in 0..len_verts {
                let vs = vec![
                    new_verts[i],
                    new_verts[i + len_verts],
                    new_verts[2 * len_verts],
                    new_verts[len_verts + i - 1],
                ];

                let new_poly = Polygon::new(vs);
                new_csg.polygons.push(new_poly);
            }
        }

        new_csg
    }

    // Read triangles
    pub fn iter_triangles<F>(&self, mut func: F)
    where
        F: FnMut(Triangle),
    {
        for poly in &self.polygons {
            for i in 1..(poly.vertices.len() - 1) {
                let v0 = poly.vertices[0].position;
                let v1 = poly.vertices[i].position;
                let v2 = poly.vertices[i + 1].position;

                let tri = Triangle {
                    positions: [v0, v1, v2],
                    normal: poly.plane.0,
                };

                func(tri);
            }
        }
    }

    pub fn get_triangles(&self) -> Vec<Triangle> {
        let mut result = Vec::new();

        for poly in &self.polygons {
            for i in 1..(poly.vertices.len() - 1) {
                let v0 = poly.vertices[0].position;
                let v1 = poly.vertices[i].position;
                let v2 = poly.vertices[i + 1].position;

                result.push(Triangle {
                    positions: [v0, v1, v2],
                    normal: poly.plane.0,
                });
            }
        }

        result
    }

    pub fn get_triangles_count(&self) -> usize {
        let mut sum = 0;

        for poly in &self.polygons {
            sum += poly.vertices.len() - 2;
        }

        sum
    }

    // Transformations
    pub fn transform_vertices<F>(mut self, func: F) -> Csg
    where
        F: Fn(Vertex) -> Vertex,
    {
        for poly in &mut self.polygons {
            for vert in &mut poly.vertices {
                *vert = func(*vert);
            }

            poly.plane = Plane::from_points(
                poly.vertices[0].position,
                poly.vertices[1].position,
                poly.vertices[2].position
            )
        }
        self
    }

    pub fn translate(self, v: Vector) -> Csg {
        self.transform_vertices(|vert| Vertex {
            position: vert.position + v,
            ..vert
        })
    }

    pub fn rotate(self, axis: Vector, angle_deg: Unit) -> Csg {
        self.transform_vertices(|vert| Vertex {
            position: vert.position.rotate(axis, angle_deg),
            normal: if vert.normal.length() > 0. {
                vert.normal.rotate(axis, angle_deg)
            } else {
                vert.normal
            },
            ..vert
        })
    }

    pub fn scale(self, v: Vector) -> Csg {
        self.transform_vertices(|vert| Vertex {
            position: Vector(
                vert.position.0 * v.0,
                vert.position.1 * v.1,
                vert.position.2 * v.2,
            ),
            ..vert
        })
    }

    pub fn union(a: &Csg, b: &Csg) -> Csg {
        let mut a = BspNode::new(Some(a.polygons.clone()));
        let mut b = BspNode::new(Some(b.polygons.clone()));

        a.clip_to(&b);
        b.clip_to(&a);
        b.invert();
        b.clip_to(&a);
        b.invert();
        a.build(b.all_polygons());

        Csg::from_polygons(a.all_polygons())
    }

    pub fn subtract(a: &Csg, b: &Csg) -> Csg {
        let mut a = BspNode::new(Some(a.polygons.clone()));
        let mut b = BspNode::new(Some(b.polygons.clone()));

        a.invert();
        a.clip_to(&b);
        b.clip_to(&a);
        b.invert();
        b.clip_to(&a);
        b.invert();
        a.build(b.all_polygons());
        a.invert();

        Csg::from_polygons(a.all_polygons())
    }

    pub fn intersect(a: &Csg, b: &Csg) -> Csg {
        let mut a = BspNode::new(Some(a.polygons.clone()));
        let mut b = BspNode::new(Some(b.polygons.clone()));

        a.invert();
        b.clip_to(&a);
        b.invert();
        a.clip_to(&b);
        b.clip_to(&a);
        a.build(b.all_polygons());
        a.invert();
        Csg::from_polygons(a.all_polygons())
    }

    pub fn inverse(&self) -> Csg {
        let mut csg = self.clone();
        for poly in csg.polygons.iter_mut() {
            poly.flip();
        }
        csg
    }
}
