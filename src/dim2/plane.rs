use dim2::{Line, LineStrip, Point};
use {Unit, EPSILON};

bitflags! {
    struct Location: u32 {
        const NONE = 0;
        const COPLANAR = 0;
        const FRONT = 1;
        const BACK = 2;
        const SPANNING = 3;
    }
}

type Collector = Vec<Line>;

#[derive(Clone)]
pub struct Plane(pub Point, pub Unit);

impl Plane {
    pub fn from_points(p0: Point, p1: Point) -> Plane {
        let n = (p1 - p0).orthogonal().normalize();
        Plane(n, n.dot(p0))
    }

    pub fn flip(&self) -> Plane {
        Plane(self.0.negate(), -self.1)
    }

    pub fn split_lines(
        &self,
        line: Line,
        coplane_front: &mut Collector,
        coplane_back: &mut Collector,
        front: &mut Collector,
        back: &mut Collector,
    ) {
        let mut polygon_type = Location::NONE;
        let mut point_locs = vec![Location::NONE; 2];

        for (i, &point) in [line.0, line.1].iter().enumerate() {
            let t = self.0.dot(point) - self.1;

            let loc = {
                if t < -EPSILON {
                    Location::BACK
                } else if t > EPSILON {
                    Location::FRONT
                } else {
                    Location::COPLANAR
                }
            };

            polygon_type |= loc;
            point_locs[i] = loc;
        }

        match polygon_type {
            Location::COPLANAR => {
                if self.0.dot(line.0) > (0 as Unit) {
                    coplane_front.push(line);
                } else {
                    coplane_back.push(line);
                }
            }
            Location::FRONT => front.push(line),
            Location::BACK => back.push(line),
            Location::SPANNING => {
                let mut f: Vec<Point> = Vec::new();
                let mut b: Vec<Point> = Vec::new();

                for (i, (p0, p1)) in [(line.0, line.1), (line.1, line.0)].iter().enumerate() {
                    let j = (i + 1) & 0b1;
                    if point_locs[i] != Location::BACK {
                        f.push(line.0);
                    }

                    if point_locs[i] != Location::FRONT {
                        b.push(line.0);
                    }

                    if (point_locs[i] | point_locs[j]) == Location::SPANNING {
                        let t = (self.1 - self.0.dot(*p0)) / self.0.dot(*p1 - *p0);

                        let v = p0.interpolate(p1, t);
                        f.push(v);
                        b.push(v);
                    }
                }

                if f.len() >= 2 {
                    front.append(&mut LineStrip::from_points(f).build_lines());
                }

                if b.len() >= 2 {
                    back.append(&mut LineStrip::from_points(b).build_lines());
                }
            }
            _ => (),
        }
    }
}
