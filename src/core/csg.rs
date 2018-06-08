use geom::{BspNode, Polygon, Unit, Vector, Vertex};
//use std::collections::HashMap;

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

    pub fn translate(&mut self, v: Vector) {
        for poly in &mut self.polygons {
            for vert in &mut poly.vertices {
                let e = vert.position;
                vert.position = e + v;
            }
        }
    }

    pub fn rotate(&mut self, axis: Vector, angle_deg: Unit) {
        for poly in &mut self.polygons {
            for vert in &mut poly.vertices {
                vert.position = vert.position.rotate(axis, angle_deg);
                if vert.normal.length() > 0. {
                    vert.normal = vert.normal.rotate(axis, angle_deg);
                }
            }
        }
    }

    // TODO: Needed for VTK
    /*
    pub fn to_vertices_and_polygons(&self) -> (Vec<Vertex>, Vec<Polygon>, usize) {
        let mut verts: Vec<Vertex> = Vec::new();
        let mut polys: Vec<Polygon> = Vec::new();

        let mut vert_index: HashMap<IVector, usize> = HashMap::new();

        for poly in &self.polygons {

            let cell:Vec<Vertex> = Vec::new();
            for vert in &poly.vertices {
                cell.push(
            }

            polys.push(Polygon::new(cell);
        }

        // TODO

        (verts, polys, 0)
    }
        */

    pub fn union(&self, other: &Csg) -> Csg {
        let mut a = BspNode::new(Some(self.polygons.clone()));
        let mut b = BspNode::new(Some(other.polygons.clone()));

        a.clip_to(&mut b);
        b.clip_to(&mut a);
        b.invert();
        b.clip_to(&mut a);
        b.invert();
        a.build(b.all_polygons());

        Csg::from_polygons(a.all_polygons())
    }

    pub fn subtract(&self, other: &Csg) -> Csg {
        let mut a = BspNode::new(Some(self.polygons.clone()));
        let mut b = BspNode::new(Some(other.polygons.clone()));

        a.invert();
        a.clip_to(&mut b);
        b.clip_to(&mut a);
        b.invert();
        b.clip_to(&mut a);
        b.invert();
        a.build(b.all_polygons());
        a.invert();

        Csg::from_polygons(a.all_polygons())
    }

    pub fn intersect(&self, other: &Csg) -> Csg {
        let mut a = BspNode::new(Some(self.polygons.clone()));
        let mut b = BspNode::new(Some(other.polygons.clone()));

        a.invert();
        b.clip_to(&mut a);
        b.invert();
        a.clip_to(&mut b);
        b.clip_to(&mut a);
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

    /// * `cen` -  Center of cube
    /// * `dim` - Dimensions of cube
    pub fn cube(cen: Vector, dim: Vector) -> Csg {
        fn dim_coord(cen: Unit, dim: Unit, test: i32) -> Unit {
            cen + if test > 0 { dim * 2. } else { -dim }
        }

        Csg::from_polygons(
            [
                ([0b000, 0b100, 0b011, 0b010], Vector(-1., 0., 0.)),
                ([0b001, 0b011, 0b111, 0b101], Vector(1., 0., 0.)),
                ([0b000, 0b001, 0b011, 0b100], Vector(0., -1., 0.)),
                ([0b010, 0b110, 0b111, 0b011], Vector(0., 1., 0.)),
                ([0b000, 0b010, 0b011, 0b001], Vector(0., 0., -1.)),
                ([0b100, 0b101, 0b111, 0b110], Vector(0., 0., 1.)),
            ].iter()
                .map(|(bit, normal)| {
                    let verts: Vec<Vertex> = bit.iter()
                        .map(|bit_coord| {
                            let position = Vector(
                                dim_coord(cen.0, dim.0, bit_coord & 0b001),
                                dim_coord(cen.1, dim.1, bit_coord & 0b010),
                                dim_coord(cen.2, dim.2, bit_coord & 0b100),
                            );

                            Vertex::new(position, *normal)
                        })
                        .collect();

                    Polygon::new(verts)
                })
                .collect(),
        )
    }
}
