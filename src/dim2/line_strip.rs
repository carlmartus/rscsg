use dim2::{Csg, Line, Point};

pub struct LineStrip {
    points: Vec<Point>,
    enclosed: bool,
}

impl LineStrip {
    pub fn new() -> LineStrip {
        LineStrip {
            points: Vec::new(),
            enclosed: false,
        }
    }

    pub fn from_vec(points: Vec<Point>) -> LineStrip {
        LineStrip {
            points,
            enclosed: false,
        }
    }

    pub fn line_to(mut self, next: Point) -> LineStrip {
        self.points.push(next);
        self
    }

    pub fn enclose(mut self) -> LineStrip {
        self.enclosed = true;
        self
    }

    pub fn build(self) -> Csg {
        let mut lines: Vec<Line> = Vec::new();

        for i in 1..self.points.len() {
            lines.push(Line::new(self.points[i - 1], self.points[i]));
        }

        Csg::from_lines(lines)
    }
}
