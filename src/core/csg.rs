use geom::{Unit, Vector, Polygon, Vertex};

#[derive(Clone)]
struct Csg {
    polygons: Vec<Polygon>,
}

impl Csg {
    pub fn new() -> Csg {
        Csg {
            polygons: Vec::new(),
        }
    }

    pub fn from_polygons(&mut self, polygons) -> Csg {
        Csg { polygons
        
        }
    }
}
