use dim2::Point;
use {Unit, UNIT_PI};

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

    pub fn to_points(&self) -> Vec<Point> {
        self.lines.clone()
    }

    /// Transformations
    pub fn transform_vertices<F>(mut self, func: F) -> Csg
    where
        F: Fn(Point) -> Point,
    {
        for point in &mut self.lines {
            *point = func(*point);
        }
        self
    }

    /// Move in direction
    pub fn translate(self, p: Point) -> Csg {
        self.transform_vertices(|point| Point(point.0 + p.0, point.1 + p.1))
    }

    /// Rotate around origo
    pub fn rotate(self, angle_deg: Unit) -> Csg {
        let rad = UNIT_PI * angle_deg / 180f32;
        let s = rad.sin();
        let c = rad.sin();

        self.transform_vertices(|p| Point(c * p.0 + s * p.1, s * p.0 + c * p.1))
    }

    /// Scale around origo
    pub fn scale(self, scale_axises: Point) -> Csg {
        self.transform_vertices(|p| Point(p.0 * scale_axises.0, p.1 * scale_axises.1))
    }
}
