use dim3::{Csg, IVector, Vector};
use Unit;

pub struct BoundBox {
    min: Vector,
    max: Vector,
    vertex_len: usize,
}

impl BoundBox {
    /// Create bounding box with origo `(0, 0, 0)` as starting point for maximum and minimum
    /// borders.
    pub fn origo() -> BoundBox {
        BoundBox {
            min: Vector(0., 0., 0.),
            max: Vector(0., 0., 0.),
            vertex_len: 0,
        }
    }

    fn on_vector(v: Vector) -> BoundBox {
        BoundBox {
            min: v,
            max: v,
            vertex_len: 1,
        }
    }

    pub fn from_csg(csg: &Csg) -> BoundBox {
        let polys = csg.to_polygons();

        if polys.len() > 0 && polys[0].vertices.len() > 0 {
            let mut bb = BoundBox::on_vector(polys[0].vertices[0].position);

            for poly in polys.iter().skip(1) {
                for vert in &poly.vertices {
                    let pos = vert.position;
                    bb.stretch(pos);
                }
            }

            bb
        } else {
            BoundBox::origo()
        }
    }

    pub fn stretch(&mut self, v: Vector) {
        // Min
        if v.0 < self.min.0 {
            self.min.0 = v.0
        }
        if v.1 < self.min.1 {
            self.min.1 = v.1
        }
        if v.2 < self.min.2 {
            self.min.2 = v.2
        }

        // Max
        if v.0 > self.max.0 {
            self.max.0 = v.0
        }
        if v.1 > self.max.1 {
            self.max.1 = v.1
        }
        if v.2 > self.max.2 {
            self.max.2 = v.2
        }

        self.vertex_len = self.vertex_len + 1;
    }

    pub fn get_min_max_discreet(&self, div: Unit) -> (IVector, IVector) {
        (self.min.discreet(div), self.max.discreet(div))
    }
}
