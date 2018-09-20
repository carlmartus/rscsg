use dim2::{Csg, Point};

pub struct LineStrip {
    items: Vec<Point>,
    enclosed: bool,
}

impl LineStrip {
    pub fn new() -> LineStrip {
        LineStrip {
            items: Vec::new(),
            enclosed: false,
        }
    }

    pub fn from_vec(items: Vec<Point>) -> LineStrip {
        LineStrip {
            items,
            enclosed: false,
        }
    }

    pub fn line_to(mut self, next: Point) -> LineStrip {
        self.items.push(next);
        self
    }

    pub fn enclose(mut self) -> LineStrip {
        self.enclosed = true;
        self
    }

    pub fn build(self) -> Csg {
        Csg::from_points(self.items)
    }
}
