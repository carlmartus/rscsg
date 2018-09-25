use dim2::{Plane, Point};

#[derive(Clone)]
pub struct Line {
    pub p0: Point,
    pub p1: Point,
    pub plane: Plane,
}

impl Line {
    pub fn new(p0: Point, p1: Point) -> Line {
        Line {
            p0,
            p1,
            plane: Plane::from_points(p0, p1),
        }
    }

    pub fn flip(&self) -> Line {
        Line {
            p0: self.p1,
            p1: self.p0,
            plane: self.plane.flip(),
        }
    }
}
