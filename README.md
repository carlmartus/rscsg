# Rust CSG
Constructive Solid Geometry in Rust.
Create 3D objects using nothing but Rust code.
Combine boolean operations like *union*, *subtraction* and *intersection*.

This library started as a port of [pycsg](https://github.com/timknip/pycsg/).

## The *Csg* structure
The `core::Csg` consists of vertices bound together to form polygons.

The struct has basic transformations; *translate*, *rotate* and *scale*.

## Sample
Have a look at the [ray tracer sample code](samples/raytrace.rs).
This code will generate a small scene demonstrating some of the available
procedures.
