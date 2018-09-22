use dim2::{Plane, Line};

#[derive(Clone)]
pub struct BspNode {
    pub plane: Option<Plane>,
    pub front: Option<Box<BspNode>>,
    pub back: Option<Box<BspNode>>,
    pub lines: Vec<Line>,
}

impl BspNode {
    pub fn new(lines: Option<Vec<Line>>) -> BspNode {
        let mut bsp = BspNode {
            plane: None,
            front: None,
            back: None,
            lines: Vec::new(),
        };

        match lines {
            Some(polygons) => bsp.build(polygons),
            _ => (),
        };
        bsp
    }

    pub fn invert(&mut self) {
        for p in self.polygons.iter_mut() {
            p.flip();
        }

        if self.plane.is_some() {
            self.plane.as_mut().unwrap().flip();
        }

        if self.front.is_some() {
            self.front.as_mut().unwrap().invert();
        }

        if self.back.is_some() {
            self.back.as_mut().unwrap().invert();
        }

        {
            let temp = self.front.clone();
            self.front = self.back.clone();
            self.back = temp;
        }
    }

    pub fn clip_lines(&mut self, lines: &Vec<Line>) -> Vec<Line> {
        if self.plane.is_none() {
            return self.lines.clone();
        }

        let mut front: Vec<Line> = Vec::new();
        let mut back: Vec<Line> = Vec::new();

        for mut line in lines {
            let mut second_front: Vec<Line> = Vec::new();
            let mut second_back: Vec<Line> = Vec::new();
            self.plane.as_mut().unwrap().split_polygon(
                &mut line,
                &mut front,
                &mut back,
                &mut second_front,
                &mut second_back,
            );
            front.append(&mut second_front);
            back.append(&mut second_back);
        }

        let mut front = if self.front.is_some() {
            self.front.as_mut().unwrap().clip_polygons(&mut front)
        } else {
            front
        };

        let mut back = if self.back.is_some() {
            self.back.as_mut().unwrap().clip_polygons(&mut back)
        } else {
            Vec::new()
        };

        front.append(&mut back);
        front
    }
}
