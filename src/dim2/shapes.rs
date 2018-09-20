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

pub fn circle(center: Point, radius: Unit, steps: usize) -> Csg {
    (0..steps)
        .fold(LineStrip::new(), |ls, i| {
            let angle = ((i as f32) / (steps as f32)) * UNIT_PI * 2f32;

            ls.line_to(Point(
                center.0 + radius * angle.cos(),
                center.1 + radius * angle.sin(),
            ))
        })
        .enclose()
        .build()
}
