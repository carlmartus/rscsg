mod bounding_box;
mod dim2;
mod plane;

use self::bounding_box::BoundBox;
use dim3::{BspNode, Csg, Plane, Polygon, Vector, Vertex};

#[test]
fn types() {
    Csg::new();
    BspNode::new(Some(vec![]));
    Plane::from_points(Vector(0., 0., 0.), Vector(1., 0., 0.), Vector(0., 1., 0.));
    Polygon::new(vec![
        Vertex::new(Vector(0., 0., 0.), Vector(0., 0., 1.)),
        Vertex::new(Vector(1., 0., 0.), Vector(0., 0., 1.)),
        Vertex::new(Vector(0., 1., 0.), Vector(0., 0., 1.)),
    ]);
    Vector(0., 0., 0.);
    Csg::new();
}

/// Create a cube, make sure it's inside a bounding box.
#[test]
fn csg_cube() {
    let cube = Csg::cube(Vector(2., 2., 2.), true);
    let bb = BoundBox::from_csg(&cube);

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
        &Csg::cube(Vector(1., 1., 1.), true), // Small cube
        &Csg::cube(Vector(2., 2., 2.), true), // Big cube
    ).to_polygons();

    assert_eq!(0, polys.len());
}

/*
#[test]
fn csg_sphere() {
    let sphere = Csg::sphere(1.0, 20, 20);
    let bb = BoundBox::from_csg(&sphere);

    let (d_min, d_max) = bb.get_min_max_discreet(10.);

    assert_eq!(-10, d_min.0);
    assert_eq!(-10, d_min.1);
    assert_eq!(-10, d_min.2);
    assert_eq!(10, d_max.0);
    assert_eq!(10, d_max.1);
    assert_eq!(10, d_max.2);
}*/
