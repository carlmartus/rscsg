use geom::{Unit, UNIT_PI};
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::hash::{Hash, Hasher};

/// Represents a 3D vector.
///
/// Example usage:
///
/// ```
/// use rscsg::geom::Vector;
/// Vector(1f32, 2f32, 3f32);
/// ```

#[derive(Clone, Copy)]
pub struct Vector(pub Unit, pub Unit, pub Unit);

impl Vector {
    pub fn negate(&self) -> Vector {
        Vector(-self.0, -self.1, -self.2)
    }

    pub fn dot(&self, other: Vector) -> Unit {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    /// Lerp. Linear interpolation from `self` to `other`
    pub fn lerp(&self, other: Vector, t: Unit) -> Vector {
        let me = self.clone();
        me + (other - me) * t
    }

    pub fn length(&self) -> Unit {
        self.dot(*self).sqrt()
    }

    /// Normalize length of vector to 1.
    pub fn normalize(&self) -> Vector {
        *self / self.length()
    }

    /// Cross product with another vector.
    pub fn cross(&self, other: Vector) -> Vector {
        Vector(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn rotate(&self, axis: Vector, angle_deg: Unit) -> Vector {
        let va: Unit = self.dot(axis);
        let vprep = *self - axis*va;
        let vprep_len = vprep.length();

        if vprep_len == 0. {
            *self
        } else {

            let cos_angle = (UNIT_PI * angle_deg / 180.).cos();
            let sin_angle = (UNIT_PI * angle_deg / 180.).sin();

            let u0 = vprep.normalize();
            let u1 = u0.cross(axis);
            let vcos = vprep_len * cos_angle;
            let vsin = vprep_len * sin_angle;
            axis*va + u0*vcos + u1*vsin
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        self.negate()
    }
}

impl Mul<Unit> for Vector {
    type Output = Vector;

    fn mul(self, rhs: Unit) -> Vector {
        Vector(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Div<Unit> for Vector {
    type Output = Vector;

    fn div(self, rhs: Unit) -> Vector {
        let inv = (1 as Unit) / rhs;
        Vector(self.0 * inv, self.1 * inv, self.2 * inv)
    }
}

impl Hash for Vector {
    fn hash<H: Hasher>(&self, hashsum: &mut H) {
        [self.0, self.1, self.2].iter().for_each(|coord| {
            let simplified = (coord * 100000.).round() as i32;
            simplified.hash(hashsum);
        });
    }
}

impl PartialEq for Vector {
    fn eq(&self, rhs: &Vector) -> bool {
        false
    }
}
