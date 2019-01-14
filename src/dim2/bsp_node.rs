use dim2::{Line, Plane};

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
            Some(lines) => bsp.build(lines),
            _ => (),
        };
        bsp
    }

    pub fn invert(&mut self) {
        for l in self.lines.iter_mut() {
            l.flip();
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
            self.plane.as_mut().unwrap().split_lines(
                line.clone(),
                &mut front,
                &mut back,
                &mut second_front,
                &mut second_back,
            );
            front.append(&mut second_front);
            back.append(&mut second_back);
        }

        let mut front = if self.front.is_some() {
            self.front.as_mut().unwrap().clip_lines(&mut front)
        } else {
            front
        };

        let mut back = if self.back.is_some() {
            self.back.as_mut().unwrap().clip_lines(&mut back)
        } else {
            Vec::new()
        };

        front.append(&mut back);
        front
    }

    pub fn clip_to(&mut self, bsp: &mut BspNode) {
        self.lines = bsp.clip_lines(&self.lines);

        if self.front.is_some() {
            self.front.as_mut().unwrap().clip_to(bsp);
        }

        if self.back.is_some() {
            self.back.as_mut().unwrap().clip_to(bsp);
        }
    }

    pub fn all_lines(&self) -> Vec<Line> {
        let mut lines: Vec<Line> = self.lines.clone();
        self.fill_lines(&mut lines);
        lines
    }

    fn fill_lines(&self, lines: &mut Vec<Line>) {
        lines.append(&mut self.lines.clone());

        if self.front.is_some() {
            self.front.as_ref().unwrap().fill_lines(lines);
        }

        if self.back.is_some() {
            self.back.as_ref().unwrap().fill_lines(lines);
        }
    }

    pub fn build(&mut self, lines: Vec<Line>) {
        if lines.len() == 0 {
            return;
        }

        if self.plane.is_none() {
            self.plane = Some(lines[0].plane.clone());
        }

        let plane = self.plane.clone().unwrap();

        self.lines.push(lines[0].clone());
        let mut front: Vec<Line> = Vec::new();
        let mut back: Vec<Line> = Vec::new();

        for line in lines.iter().skip(1) {
            let mut second: Vec<Line> = Vec::new();

            plane.split_lines(
                line.clone(),
                &mut self.lines,
                &mut second,
                &mut front,
                &mut back,
            );
            self.lines.append(&mut second);
        }

        if !front.is_empty() {
            if self.front.is_none() {
                self.front = Some(Box::new(BspNode::new(None)));
            }

            self.front.as_mut().unwrap().build(front);
        }

        if !back.is_empty() {
            if self.back.is_none() {
                self.back = Some(Box::new(BspNode::new(None)));
            }

            self.back.as_mut().unwrap().build(back);
        }
    }
}
