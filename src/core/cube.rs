use core::Csg;
use geom::{Polygon, Unit, Vector, Vertex};

impl Csg {
    /// * `cen` -  Center of cube
    /// * `dim` - Dimensions of cube
    pub fn cube(cen: Vector, dim: Vector) -> Csg {
        fn dim_coord(cen: Unit, dim: Unit, test: i32) -> Unit {
            cen + if test > 0 { dim } else { -dim }
        }

        Csg::from_polygons(
            [
                ([0b000, 0b100, 0b110, 0b010], Vector(-1., 0., 0.)),
                ([0b001, 0b011, 0b111, 0b101], Vector(1., 0., 0.)),
                ([0b000, 0b001, 0b101, 0b100], Vector(0., -1., 0.)),
                ([0b010, 0b110, 0b111, 0b011], Vector(0., 1., 0.)),
                ([0b000, 0b010, 0b011, 0b001], Vector(0., 0., -1.)),
                ([0b100, 0b101, 0b111, 0b110], Vector(0., 0., 1.)),
            ].iter()
                .map(|(bit, normal)| {
                    let verts: Vec<Vertex> = bit.iter()
                        .map(|bit_coord| {
                            let position = Vector(
                                dim_coord(cen.0, dim.0, bit_coord & 0b001),
                                dim_coord(cen.1, dim.1, bit_coord & 0b010),
                                dim_coord(cen.2, dim.2, bit_coord & 0b100),
                            );

                            Vertex::new(position, *normal)
                        })
                        .collect();

                    Polygon::new(verts)
                })
                .collect(),
        )
    }
}
