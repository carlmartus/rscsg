use dim3::Vector;
use Unit;

/// Represents a vertex of a polygon. Use your own vertex class instead of this one to provide
/// additional features like texture coordinates and vertex colors. Custom vertex classes need to
/// provide a `pos` property and `clone()`, `flip()`, and `interpolate()` methods that behave
/// analogous to the ones defined by `Vertex`. This class provides `normal` so convenience
/// functions like `CSG.sphere()` can return a smooth vertex normal, but `normal` is not used
/// anywhere else.

#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: Vector,
    pub normal: Vector,
}

impl Vertex {
    pub fn new(position: Vector, normal: Vector) -> Vertex {
        Vertex { position, normal }
    }

    /// Invert all orientation-specific data (e.g. vertex normal). Called when the orientation of a
    /// polygon is flipped.
    pub fn flip(&mut self) -> Vertex {
        Vertex::new(self.position, self.normal.negate())
    }

    /// Create a new vertex between this vertex and `other` by linearly interpolating all
    /// properties using a parameter of `t`. Subclasses should override this to interpolate
    /// additional properties.
    pub fn interpolate(&self, other: Vertex, t: Unit) -> Vertex {
        Vertex::new(
            self.position.lerp(other.position, t),
            self.normal.lerp(other.normal, t),
        )
    }
}
