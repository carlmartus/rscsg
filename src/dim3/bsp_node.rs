use dim3::{Plane, Polygon};

/// Holds a node in a BSP tree. A BSP tree is built from a collection of polygons by picking a
/// polygon to split along. That polygon (and all other coplanar polygons) are added directly to
/// that node and the other polygons are added to the front and/or back subtrees. This is not a
/// leafy BSP tree since there is no distinction between internal and leaf nodes.

#[derive(Clone)]
pub struct BspNode {
    pub plane: Option<Plane>,
    pub front: Option<Box<BspNode>>,
    pub back: Option<Box<BspNode>>,
    pub polygons: Vec<Polygon>,
}

impl BspNode {
    pub fn new(polygons: Option<Vec<Polygon>>) -> BspNode {
        let mut bsp = BspNode {
            plane: None,
            front: None,
            back: None,
            polygons: Vec::new(),
        };

        match polygons {
            Some(polygons) => bsp.build(polygons),
            _ => (),
        }
        bsp
    }

    /// Convert solid space to empty space and empty space to solid space.
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

        std::mem::swap(&mut self.front, &mut self.back);
    }

    /// Recursively remove all polygons in `polygons` that are inside this BSP tree.
    pub fn clip_polygons(&self, polygons: &Vec<Polygon>) -> Vec<Polygon> {
        if self.plane.is_none() {
            return self.polygons.clone();
        }

        let mut front: Vec<Polygon> = Vec::new();
        let mut back: Vec<Polygon> = Vec::new();

        for poly in polygons {
            let mut second_front: Vec<Polygon> = Vec::new();
            let mut second_back: Vec<Polygon> = Vec::new();
            self.plane.as_ref().unwrap().split_polygon(
                &poly,
                &mut front,
                &mut back,
                &mut second_front,
                &mut second_back,
            );
            front.append(&mut second_front);
            back.append(&mut second_back);
        }

        let mut front = if self.front.is_some() {
            self.front.as_ref().unwrap().clip_polygons(&mut front)
        } else {
            front
        };

        let mut back = if self.back.is_some() {
            self.back.as_ref().unwrap().clip_polygons(&mut back)
        } else {
            Vec::new()
        };

        front.append(&mut back);
        front
    }

    pub fn clip_to(&mut self, bsp: &BspNode) {
        self.polygons = bsp.clip_polygons(&self.polygons);

        if self.front.is_some() {
            self.front.as_mut().unwrap().clip_to(bsp);
        }

        if self.back.is_some() {
            self.back.as_mut().unwrap().clip_to(bsp);
        }
    }

    pub fn all_polygons(&self) -> Vec<Polygon> {
        let mut polys: Vec<Polygon> = Vec::new();
        self.fill_polygons(&mut polys);
        polys
    }

    fn fill_polygons(&self, polys: &mut Vec<Polygon>) {
        polys.append(&mut self.polygons.clone());

        if self.front.is_some() {
            self.front.as_ref().unwrap().fill_polygons(polys);
        }

        if self.back.is_some() {
            self.back.as_ref().unwrap().fill_polygons(polys);
        }
    }

    /// Build a BSP tree out of `Vec<Polygon>`. When called on an existing tree, the new polygons
    /// are filtered down to the bottom of the tree and become new nodes there. Each set of
    /// polygons is partitioned using the first polygon (no heuristic is used to pick a good
    /// split).
    pub fn build(&mut self, polygons: Vec<Polygon>) {
        if polygons.len() == 0 {
            return;
        }

        if self.plane.is_none() {
            self.plane = Some(polygons[0].plane.clone());
        }

        let plane = self.plane.clone().unwrap();

        self.polygons.push(polygons[0].clone());
        let mut front: Vec<Polygon> = Vec::new();
        let mut back: Vec<Polygon> = Vec::new();

        for poly in polygons.iter().skip(1) {
            let mut second: Vec<Polygon> = Vec::new();

            plane.split_polygon(
                &poly,
                &mut self.polygons,
                &mut second,
                &mut front,
                &mut back,
            );
            self.polygons.append(&mut second);
        }

        // Recursively build the BSP tree

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
