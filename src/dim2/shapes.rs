use dim2::{Csg, LineStrip, Point};
use Unit;

pub fn rectangle(lo: Point, hi: Point) -> Csg {
    LineStrip::new(hi)
        .line_to(Point(hi.0, lo.1))
        .line_to(Point(lo.0, lo.1))
        .line_to(Point(lo.0, hi.1))
        .enclose()
        .build()
}

pub fn sphere(center: Point, radius: Unit, steps: usize) -> Csg {
    // TODO: Implement
}
