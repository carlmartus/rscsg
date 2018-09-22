use dim2::Point;
use Unit;

#[derive(Clone)]
pub struct Plane(pub Point, pub Unit);

impl Plane {
    pub fn from_points(p0: Point, p1: Point) -> Plane {
        let n = (p1 - p0).orthogonal().normalize();
        Plane(n, n.dot(p0))
    }

    pub fn flip(&self) -> Plane {
        Plane(self.0.negate(), -self.1)
    }

    //pub fn split_lines(&self, // TODO How is this comparable to a 3D plane?
}
