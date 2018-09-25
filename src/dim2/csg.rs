use dim2::{BspNode, Line, Point};
use {Unit, UNIT_PI};

#[derive(Clone)]
pub struct Csg {
    pub lines: Vec<Line>,
}

impl Csg {
    pub fn new() -> Csg {
        Csg { lines: Vec::new() }
    }

    pub fn from_lines(lines: Vec<Line>) -> Csg {
        Csg { lines }
    }

    pub fn to_lines(&self) -> Vec<Line> {
        self.lines.clone()
    }

    /// Transformations
    pub fn transform_points<F>(mut self, func: F) -> Csg
    where
        F: Fn(Point) -> Point,
    {
        for line in &mut self.lines {
            let p0 = func(line.p0);
            let p1 = func(line.p1);
            *line = Line::new(p0, p1);
        }
        self
    }

    /// Move in direction
    pub fn translate(self, p: Point) -> Csg {
        self.transform_points(|point| Point(point.0 + p.0, point.1 + p.1))
    }

    /// Rotate around origo
    pub fn rotate(self, angle_deg: Unit) -> Csg {
        let rad = UNIT_PI * angle_deg / 180f32;
        let s = rad.sin();
        let c = rad.sin();

        self.transform_points(|p| Point(c * p.0 + s * p.1, s * p.0 + c * p.1))
    }

    /// Scale around origo
    pub fn scale(self, scale_axises: Point) -> Csg {
        self.transform_points(|p| Point(p.0 * scale_axises.0, p.1 * scale_axises.1))
    }

    pub fn union(&self, other: &Csg) -> Csg {
        let mut a = BspNode::new(Some(self.lines.clone()));
        let mut b = BspNode::new(Some(other.lines.clone()));

        a.clip_to(&mut b);
        b.clip_to(&mut a);
        b.invert();
        b.clip_to(&mut a);
        b.invert();
        a.build(b.all_lines());

        Csg::from_lines(a.all_lines())
    }
}
