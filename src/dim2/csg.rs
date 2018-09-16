use dim2::Point;

#[derive(Clone)]
pub struct Csg {
    pub lines: Vec<Point>,
}

impl Csg {
    pub fn new() -> Csg {
        Csg { lines: Vec::new() }
    }

    pub fn from_points(lines: Vec<Point>) -> Csg {
        Csg { lines }
    }
}
