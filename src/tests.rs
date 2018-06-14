use core::Csg;
use geom::{IVector, Unit, Vector};

struct BoundBox {
    min: Vector,
    max: Vector,
}

#[test]
fn types() {
    Vector(0., 0., 0.);
}

#[test]
fn csg_cube() {
    let cube = Csg::cube(Vector(0., 0., 0.), Vector(1.0, 1.0, 1.0));
    let polys = cube.to_polygons();
    let mut bb = BoundBox::origo();

    assert_eq!(6, polys.len());

    for poly in &polys {
        assert_eq!(4, poly.vertices.len());

        for vert in &poly.vertices {
            let pos = vert.position;
            bb.stretch(pos);
        }
    }

    // Get ivectors of bounding box, coords snapped to closest 0.1
    let (d_min, d_max) = bb.get_min_max_discreet(10.);

    assert_eq!(-10, d_min.0);
    assert_eq!(-10, d_min.1);
    assert_eq!(-10, d_min.2);
    assert_eq!( 10, d_max.0);
    assert_eq!( 10, d_max.1);
    assert_eq!( 10, d_max.2);
}

impl BoundBox {
    /// Create bounding box with origo `(0, 0, 0)` as starting point for maximum and minimum
    /// borders.
    pub fn origo() -> BoundBox {
        BoundBox {
            min: Vector(0., 0., 0.),
            max: Vector(0., 0., 0.),
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
        if v.0 > self.min.0 {
            self.max.0 = v.0
        }
        if v.1 > self.min.1 {
            self.max.1 = v.1
        }
        if v.2 > self.min.2 {
            self.max.2 = v.2
        }
    }

    pub fn get_min_max_discreet(&self, div: Unit) -> (IVector, IVector) {
        (self.min.discreet(div), self.max.discreet(div))
    }
}
