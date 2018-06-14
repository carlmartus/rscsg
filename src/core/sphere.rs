use core::Csg;
use geom::{Polygon, Unit, Vector, Vertex, UNIT_PI};

impl Csg {
    pub fn sphere(cen: Vector, radius: Unit, slices: usize, stacks: usize) -> Csg {
        fn make_vertex(cen: Vector, radius: Unit, theta: Unit, phi: Unit) -> Vertex {
            let d = Vector(
                theta.cos() * phi.sin(),
                theta.sin() * phi.cos(),
                theta.cos(),
            );

            Vertex::new(cen + d * radius, d)
        };

        let mut polys: Vec<Polygon> = Vec::new();
        let delta_theta = UNIT_PI * 2.0 / (slices as Unit);
        let delta_phi = UNIT_PI / (stacks as Unit);

        // Bottom and top
        for i in 0..slices {
            let i0 = i as Unit;
            let i1 = i0 + 1.0;

            // Bottom
            //  +--+
            //  | /
            //  |/
            //  +
            polys.push(Polygon::new(vec![
                make_vertex(cen, radius, i0 * delta_theta, 0.0),
                make_vertex(cen, radius, i0 * delta_theta, delta_phi),
                make_vertex(cen, radius, i1 * delta_theta, delta_phi),
            ]));

            // Top
            //  +
            //  |\
            //  | \
            //  +--+
            let j0 = (stacks as Unit) - 1.0;
            let j1 = j0 + 1.0;

            polys.push(Polygon::new(vec![
                make_vertex(cen, radius, i0 * delta_theta, j0 * delta_phi),
                make_vertex(cen, radius, i1 * delta_theta, j0 * delta_phi),
                make_vertex(cen, radius, i0 * delta_theta, j1 * delta_phi),
            ]));
        }

        for j in 1..stacks - 1 {
            let j0 = j as Unit;
            let j1 = j0 + 0.5;
            let j2 = j0 + 1.0;

            // Between top and bottom
            //  +---+
            //  |\ /|
            //  | x |
            //  |/ \|
            //  +---+

            for i in 0..slices {
                let i0 = i as Unit;
                let i1 = i0 + 0.5;
                let i2 = i0 + 1.0;

                polys.push(Polygon::new(vec![
                    make_vertex(cen, radius, i1 * delta_theta, j1 * delta_phi),
                    make_vertex(cen, radius, i2 * delta_theta, j2 * delta_phi),
                    make_vertex(cen, radius, i0 * delta_theta, j2 * delta_phi),
                ]));

                polys.push(Polygon::new(vec![
                    make_vertex(cen, radius, i1 * delta_theta, j1 * delta_phi),
                    make_vertex(cen, radius, i0 * delta_theta, j0 * delta_phi),
                    make_vertex(cen, radius, i2 * delta_theta, j0 * delta_phi),
                ]));

                polys.push(Polygon::new(vec![
                    make_vertex(cen, radius, i1 * delta_theta, j1 * delta_phi),
                    make_vertex(cen, radius, i0 * delta_theta, j2 * delta_phi),
                    make_vertex(cen, radius, i0 * delta_theta, j0 * delta_phi),
                ]));

                polys.push(Polygon::new(vec![
                    make_vertex(cen, radius, i1 * delta_theta, j1 * delta_phi),
                    make_vertex(cen, radius, i2 * delta_theta, j0 * delta_phi),
                    make_vertex(cen, radius, i2 * delta_theta, j2 * delta_phi),
                ]));
            }
        }

        Csg::new() // TODO placeholder
    }
}
