mod bounding_box;

use self::bounding_box::BoundBox;
use core::Csg;
use geom::Vector;

#[test]
fn types() {
    Vector(0., 0., 0.);
}

/// Create a cube, make sure it's inside a bounding box.
#[test]
fn csg_cube() {
    let cube = Csg::cube(Vector(0., 0., 0.), Vector(1.0, 1.0, 1.0));
    let polys = cube.to_polygons();
    let mut bb = BoundBox::origo();

    assert_eq!(6, polys.len());

    for poly in &polys {
        assert_eq!(4, poly.vertices.len());

        for vert in &poly.vertices {
            let pos = vert.position;
            bb.stretch(pos);
        }
    }

    // Get ivectors of bounding box, coords snapped to closest 0.1
    let (d_min, d_max) = bb.get_min_max_discreet(10.);

    assert_eq!(-10, d_min.0);
    assert_eq!(-10, d_min.1);
    assert_eq!(-10, d_min.2);
    assert_eq!(10, d_max.0);
    assert_eq!(10, d_max.1);
    assert_eq!(10, d_max.2);
}

/// Big cube will subtract itself onto a smaller cube, removing everything.
#[test]
fn csg_total_subtraction() {
    let polys = Csg::subtract(
        &Csg::cube(Vector(0., 0., 0.), Vector(1., 1., 1.)), // Small cube
        &Csg::cube(Vector(0., 0., 0.), Vector(2., 2., 2.)), // Big cube
    ).to_polygons();

    assert_eq!(0, polys.len());
}
