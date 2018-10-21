# Rust CSG
[![Build Status](https://travis-ci.org/carlmartus/rscsg.svg?branch=master)](https://travis-ci.org/carlmartus/rscsg)

Constructive Solid Geometry in Rust.
Create 2D and/or 3D objects using nothing but Rust code.
Combine boolean operations like *union*, *subtraction* and *intersection*.

This library started as a port of [pycsg](https://github.com/timknip/pycsg/).
With added operations for 2D objects.

## The *Csg* structure
The `rscsg::dim3::Csg` consists of vertices bound together to form polygons.

The struct has basic transformations; *translate*, *rotate* and *scale*.

## Sample
Have a look at the [ray tracer sample code](examples/raytrace.rs) in comination
with the [smallpt crate](https://crates.io/crates/smallpt).
This code will generate a small scene demonstrating some of the available
procedures.

execute the ray trace sample with:
```shell
cargo run --example raytrace
```

## Roadmap
- [X] Basic 3D CSG.
- [X] Basic 2D CSG.
- [X] Real time 3D sample.
- [ ] [2D to 3D extractions](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Using_the_2D_Subsystem#Linear_Extrude).
- [ ] Add shared date component to CSG obejcts.
- [ ] Export to known format(s).
