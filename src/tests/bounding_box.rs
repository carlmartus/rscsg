use geom::{IVector, Unit, Vector};

pub struct BoundBox {
    min: Vector,
    max: Vector,
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
