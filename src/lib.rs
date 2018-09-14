pub mod dim3;

#[macro_use]
extern crate bitflags;

#[cfg(test)]
mod tests;

use std::f32::consts::PI;

pub type Unit = f32;
pub const UNIT_PI: Unit = PI;

pub type IUnit = i32;
