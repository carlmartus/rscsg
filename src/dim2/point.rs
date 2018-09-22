use std::ops::{Add, Div, Mul, Neg, Sub};
use Unit;

#[derive(Clone, Copy)]
pub struct Point(pub Unit, pub Unit);

impl Point {
    pub fn negate(&self) -> Point {
        Point(-self.0, -self.1)
    }

    pub fn dot(&self, other: Point) -> Unit {
        self.0 * other.0 + self.1 * other.1
    }

    pub fn length(&self) -> Unit {
        self.dot(*self).sqrt()
    }

    pub fn orthogonal(&self) -> Point {
        Point(-self.1, self.0)
    }

    pub fn normalize(&self) -> Point {
        *self / self.length()
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point(self.0 - other.0, self.1 - other.1)
    }
}

impl Neg for Point {
    type Output = Point;
    fn neg(self) -> Point {
        self.negate()
    }
}

impl Mul<Unit> for Point {
    type Output = Point;
    fn mul(self, rhs: Unit) -> Point {
        Point(self.0 * rhs, self.1 * rhs)
    }
}

impl Div<Unit> for Point {
    type Output = Point;
    fn div(self, rhs: Unit) -> Point {
        let inv = (1 as Unit) / rhs;
        Point(self.0 * inv, self.1 * inv)
    }
}
