use dim3::{Plane, Polygon, Vector, Vertex};

#[test]
fn split_triangle() {
    let poly = Polygon::new(vec![
        Vertex::new(Vector(0., 0., 0.), Vector(0., 0., 1.)),
        Vertex::new(Vector(2., 0., 0.), Vector(0., 0., 1.)),
        Vertex::new(Vector(0., 1., 0.), Vector(0., 0., 1.)),
    ]);

    let plane = Plane::from_points(Vector(1., 0., 0.), Vector(1., 1., 0.), Vector(1., 0., 1.));

    let mut coplane_front = Vec::new();
    let mut coplane_back = Vec::new();
    let mut front = Vec::new();
    let mut back = Vec::new();

    plane.split_polygon(
        &poly,
        &mut coplane_front,
        &mut coplane_back,
        &mut front,
        &mut back,
    );

    // Assert amount of polygons
    assert_eq!(coplane_front.len(), 0);
    assert_eq!(coplane_back.len(), 0);
    assert_eq!(front.len(), 1);
    assert_eq!(back.len(), 1);

    // Assert right amount of vertices per polygon
    let poly_front = &front[0];
    let poly_back = &back[0];
    assert_eq!(poly_front.vertices.len(), 3);
    assert_eq!(poly_back.vertices.len(), 4);

    // Assert normals
    assert!(poly_front.plane.0 .2 > 0.9);
    assert!(poly_back.plane.0 .2 > 0.9);
}

#[test]
fn missed_split_triangle() {
    let poly = Polygon::new(vec![
        Vertex::new(Vector(0., 0., 0.), Vector(0., 0., 1.)),
        Vertex::new(Vector(2., 0., 0.), Vector(0., 0., 1.)),
        Vertex::new(Vector(0., 1., 0.), Vector(0., 0., 1.)),
    ]);

    let plane = Plane::from_points(
        Vector(-1., 0., 0.),
        Vector(-1., 1., 0.),
        Vector(-1., 0., 1.),
    );

    let mut coplane_front = Vec::new();
    let mut coplane_back = Vec::new();
    let mut front = Vec::new();
    let mut back = Vec::new();

    plane.split_polygon(
        &poly,
        &mut coplane_front,
        &mut coplane_back,
        &mut front,
        &mut back,
    );

    // Assert amount of polygons
    assert_eq!(coplane_front.len(), 0);
    assert_eq!(coplane_back.len(), 0);
    assert_eq!(front.len(), 1);
    assert_eq!(back.len(), 0);

    // Assert same as input polygon
    let new_poly = &front[0];

    assert_eq!(3, new_poly.vertices.len());
    assert!(new_poly.plane.0 .2 > 0.9);
}
