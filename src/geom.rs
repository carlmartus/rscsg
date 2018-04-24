use std::ops::{Add, Div, Mul, Neg, Sub};

type Unit = f32;

#[derive(Clone, Copy)]
pub struct Vector(pub Unit, pub Unit, pub Unit);

impl Vector {
    pub fn negate(&self) -> Vector {
        Vector(-self.0, -self.1, -self.2)
    }

    pub fn dot(&self, other: Vector) -> Unit {
        self.0*other.0 + self.1*other.1 + self.2*other.2
    }

    pub fn lerp(&self, other: Vector, t: Unit) -> Vector {
        let me = self.clone();
        me + (other - me) * t
    }

    pub fn length(&self) -> Unit {
        self.dot(*self).sqrt()
    }

    pub fn normalize(&self) -> Vector {
        *self / self.length()
    }

    pub fn cross(&self, other: Vector) -> Vector {
        Vector(
            self.1*other.2 - self.2*other.1,
            self.2*other.0 - self.0*other.2,
            self.0*other.1 - self.1*other.0)
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2)
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector(
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2)
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
        Vector(
            self.0 * rhs,
            self.1 * rhs,
            self.2 * rhs)
    }
}

impl Div<Unit> for Vector {
    type Output = Vector;

    fn div(self, rhs: Unit) -> Vector {
        let inv = (1 as Unit) / rhs;
        Vector(
            self.0 * inv,
            self.1 * inv,
            self.2 * inv)
    }
}
