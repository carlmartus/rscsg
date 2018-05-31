use geom::{Polygon, Vector, Vertex, Unit};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Csg {
    polygons: Vec<Polygon>,
}

impl Csg {
    pub fn new() -> Csg {
        Csg {
            polygons: Vec::new(),
        }
    }

    pub fn from_polygons(&mut self, polygons: Vec<Polygon>) -> Csg {
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

            let mid_pos = poly.vertices
                .iter()
                .fold(Vector(0f32, 0f32, 0f32), |acc, v| acc + v.position)
                / (poly.vertices.len() as f32);
            let mid_nor = poly.vertices[0].normal;

            let mid_vert = Vertex::new(mid_pos, mid_nor);

            let len_verts = poly.vertices.len();
            // TODO Concat with poly.vertices
            let mut gen_verts: Vec<Vertex> = (0..len_verts)
                .map(|i| poly.vertices[i].interpolate(poly.vertices[(i + 1) % len_verts], 0.5))
                .collect();

            //let new_verts = append(&mut poly.vertices.clone());
            let mut new_verts = poly.vertices.clone();
            new_verts.append(&mut gen_verts);
            new_verts.push(mid_vert);

            for i in 0..len_verts {
                let vs = vec![
                    new_verts[i],
                    new_verts[i+len_verts],
                    new_verts[2*len_verts],
                    new_verts[len_verts+i-1]];

                    let new_poly = Polygon::new(vs);
                    new_csg.polygons.push(new_poly);
            }
        }

        new_csg
    }

    pub fn translate(&mut self, v: Vector) {
        for poly in &mut self.polygons {
            for vert in &mut poly.vertices {
                let e = vert.position;
                vert.position = e + v;
            }
        }
    }

    pub fn rotate (&mut self, axis: Vector, angle_deg: Unit) {
        for poly in &mut self.polygons {
            for vert in &mut poly.vertices {
                vert.position = vert.position.rotate(axis, angle_deg);
                if vert.normal.length() > 0. {
                    vert.normal = vert.normal.rotate(axis, angle_deg);
                }
            }
        }
    }

    pub fn to_vertices_and_polygons(&self) -> (Vec<Vertex>, Vec<Polygon>, usize) {
        let mut verts: Vec<Vertex> = Vec::new();
        let mut polys: Vec<Polygon> = Vec::new();

        let mut vert_index: HashMap<Vector, usize> = HashMap::new();

        for poly in &self.polygons {
            for vert in &poly.vertices {
            }
        }

        // TODO

        (verts, polys, 0)
    }
}
