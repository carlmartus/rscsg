use dim3::Csg;
use dim3::{Polygon, Vector, Vertex};
use Unit;

impl Csg {
    /// * `dim` - Dimensions of cube
    pub fn cube(dim: Vector, center: bool) -> Csg {
        fn dim_coord(dim: Unit, test: i32, center: bool) -> Unit {
            let offset = if test > 0 { dim } else { 0.0 };

            offset + if center { -dim * 0.5 } else { 0.0 }
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
                                dim_coord(dim.0, bit_coord & 0b001, center),
                                dim_coord(dim.1, bit_coord & 0b010, center),
                                dim_coord(dim.2, bit_coord & 0b100, center),
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
