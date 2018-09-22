use dim2::{Plane, Point};

#[derive(Clone)]
pub struct Line(pub Point, pub Point, pub Plane);

impl Line {
    pub fn new(p0: Point, p1: Point) -> Line {
        Line(p0, p1, Plane::from_points(p0, p1))
    }

    pub fn flip(&self) -> Line {
        Line(self.1, self.0, self.2.flip())
    }
}
