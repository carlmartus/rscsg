use dim2::{Csg, LineStrip, Point};
use {Unit, UNIT_PI};

pub fn rectangle(lo: Point, hi: Point) -> Csg {
    LineStrip::new()
        .line_to(hi)
        .line_to(Point(hi.0, lo.1))
        .line_to(Point(lo.0, lo.1))
        .line_to(Point(lo.0, hi.1))
        .enclose()
        .build()
}

pub fn sphere(center: Point, radius: Unit, steps: usize) -> Csg {
    /*
    for i in 0..steps {
        let angle = ((i as f32) / (steps as f32)) * UNIT_PI*2f32;
    }*/

    Csg::new() // TODO Placeholder
}
