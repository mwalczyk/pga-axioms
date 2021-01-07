use std::ops::{Add, Div, Mul, Sub};
use std::ops::{BitAnd, BitOr, BitXor, Not};
use std::fmt::Display;

use crate::line::Line;

/// A point in 2D PGA.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    /// The z-coordinate of the homogeneous point, dual to e0
    pub e12: f32,

    /// The x-coordinate of the homogeneous point, dual to e1
    pub e20: f32,

    /// The y-coordinate of the homogeneous point, dual to e2
    pub e01: f32,
}

impl Point {

    /// Constructs a new point with the specified components.
    pub fn new(e12: f32, e20: f32, e01: f32) -> Self {
        Self {
            e12,
            e20,
            e01,
        }
    }

    /// Construct a new Euclidean point with homogeneous coordinates `(x, y, 1)`.
    pub fn euclidean(x: f32, y: f32) -> Self {
        Self {
            e12: 1.0,
            e20: x,
            e01: y,
        }
    }

    /// Construct a new ideal point with homogeneous coordinates `(x, y, 0)`.
    pub fn ideal(x: f32, y: f32) -> Self {
        Self {
            e12: 0.0,
            e20: x,
            e01: y,
        }
    }

    /// Returns the x-coordinate of the point.
    pub fn x(&self) -> f32 {
        self.e20
    }

    /// Returns the y-coordinate of the point.
    pub fn y(&self) -> f32 {
        self.e01
    }

    /// Returns the z-coordinate (homogeneous) of the point.
    pub fn z(&self) -> f32 {
        self.e12
    }

    /// Returns the Euclidean norm of the point.
    ///
    /// The Euclidean norm of a point can be found via the formula \sqrt{p\bar{p}},
    /// where \bar{p} denotes the conjugate of p. This formula simplifies to \sqrt{z^2}.
    pub fn norm(&self) -> f32 {
        // TODO: ideal norm (see formula above, from PGA cheatsheet)

        (self.e12 * self.e12).sqrt()
    }

    /// The ideal norm of a point is \sqrt{x^2 + y^2}.
    pub fn ideal_norm(&self) -> f32 {
        (self.e20 * self.e20 + self.e01 * self.e01).sqrt()
    }

    /// Returns a normalized version of the point (note that the point will be
    /// normalized with respect to its Euclidean norm).
    pub fn normalized(&self) -> Self {
        // For ideal points (i.e. points for which the e12 component is zero),
        // we don't need to do anything?
        let norm = self.norm();
        if norm < f32::EPSILON {
            return *self;
        }
        // This is a Euclidean point
        *self * (1.0 / self.norm())
    }


}

/// Add two points element-wise.
impl Add for Point {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            e12: self.e12 + rhs.e12,
            e20: self.e20 + rhs.e20,
            e01: self.e01 + rhs.e01,
        }
    }
}

/// Subtract two points element-wise.
impl Sub for Point {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            e12: self.e12 - rhs.e12,
            e20: self.e20 - rhs.e20,
            e01: self.e01 - rhs.e01,
        }
    }
}

/// Multiply a point by a scalar.
impl Mul<f32> for Point {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            e12: self.e12 * rhs,
            e20: self.e20 * rhs,
            e01: self.e01 * rhs,
        }
    }
}

/// "Join" two points in a line `p1 & p2`. Note that the order of the
/// arguments determines the "direction" of the line: `p1 & p2` results
/// in a line that "moves" from `p1` to `p2`.
impl BitAnd for Point {
    type Output = Line;

    fn bitand(self, rhs: Self) -> Self::Output {
        !(!self ^ !rhs)
    }
}

/// Inner product between a point and a line `p | l`.
impl BitOr<Line> for Point {
    type Output = Line;

    fn bitor(self, rhs: Line) -> Self::Output {
        // This is just the grade-1 part of the geometric product `p * l`
        Self::Output {
            e0: rhs.e1 * self.e01 - rhs.e2 * self.e20,
            e1: rhs.e2 * self.e12,
            e2: -rhs.e1 * self.e12,
        }
    }
}

/// Returns the line that is dual to this point `!p`.
impl Not for Point {
    type Output = Line;

    fn not(self) -> Self::Output {
        Self::Output {
            e0: self.e12,
            e1: self.e20,
            e2: self.e01,
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "e12: {}, e20: {}, e01: {}", self.e12, self.e20, self.e01)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inner_product() {
        let p = Point::new(1.0, 2.0, 3.0);
        let l = Line::new(4.0, 5.0, 6.0);
        let result = p | l;
        println!("p | l = {:?}", result);
    }
}