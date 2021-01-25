use std::fmt::Display;
use std::ops::{Add, Mul, Sub};
use std::ops::{BitOr, BitXor, Not};

use crate::point::Point;

/// A line in 2D PGA.
#[derive(Copy, Clone, Debug)]
pub struct Line {
    pub e0: f32,
    pub e1: f32,
    pub e2: f32,
}

impl Line {
    /// Constructs a new line with the specified components.
    pub fn new(e0: f32, e1: f32, e2: f32) -> Line {
        Line { e0, e1, e2 }
    }

    /// Returns a new line representing the equation `y = mx + b`.
    pub fn from_slope_intercept(m: f32, b: f32) -> Line {
        Line::new(-b, -m, 1.0)
    }

    /// `c` in the equation for this line: `ax + by + c = 0`.
    pub fn c(&self) -> f32 {
        self.e0
    }

    /// `a` in the equation for this line: `ax + by + c = 0`.
    pub fn a(&self) -> f32 {
        self.e1
    }

    /// `b` in the equation for this line: `ax + by + c = 0`.
    pub fn b(&self) -> f32 {
        self.e2
    }

    /// Euclidean lines can be written as: `ax + by + c = 0`. Therefore, the slope is
    /// `-a / b`.
    pub fn slope(&self) -> f32 {
        -self.e1 / self.e2
    }

    /// Euclidean lines can be written as: `ax + by + c = 0`. Therefore, the y-intercept
    /// is `-c / b`.
    pub fn intercept(&self) -> f32 {
        -self.e0 / self.e2
    }

    /// Returns the direction orthogonal to this line, represented as an ideal point.
    /// Algebraically, this is the product `lI`, where `I` is the pseudoscalar `e012`.
    pub fn ortho(&self) -> Point {
        Point::ideal(self.e1, self.e2)
    }

    /// Returns the Euclidean norm of the line.
    ///
    /// The Euclidean norm of a line can be found via the formula $\sqrt{|l\bar{l}|}$,
    /// where $\bar{l}$ denotes the conjugate of l. This formula simplifies to
    /// $\sqrt{b^2 + c^2}$.
    pub fn norm(&self) -> f32 {
        (self.e1 * self.e1 + self.e2 * self.e2).sqrt()
    }

    /// The ideal norm of a line is ???
    pub fn ideal_norm(&self) -> f32 {
        unimplemented!()
    }

    /// Returns a normalized version of the line.
    pub fn normalized(&self) -> Self {
        // For ideal lines (i.e. points for which the e1 and e2 components are zero),
        // we don't need to do anything?
        let norm = self.norm();
        if norm < f32::EPSILON {
            return *self;
        }
        // This is a Euclidean line
        *self * (1.0 / self.norm())
    }
}

/// Add two lines element-wise.
impl Add for Line {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            e0: self.e0 + rhs.e0,
            e1: self.e1 + rhs.e1,
            e2: self.e2 + rhs.e2,
        }
    }
}

/// Subtract two lines element-wise.
impl Sub for Line {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            e0: self.e0 - rhs.e0,
            e1: self.e1 - rhs.e1,
            e2: self.e2 - rhs.e2,
        }
    }
}

/// Multiply a line by a scalar.
impl Mul<f32> for Line {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            e0: self.e0 * rhs,
            e1: self.e1 * rhs,
            e2: self.e2 * rhs,
        }
    }
}

/// Inner product between two lines `l1 | l2`.
impl BitOr for Line {
    type Output = f32;

    fn bitor(self, rhs: Self) -> Self::Output {
        // This is just the grade-0 part of the geometric product `l1 * l2`
        self.e1 * rhs.e1 + self.e2 * rhs.e2
    }
}

/// Inner product between a line and a point `l | p`.
impl BitOr<Point> for Line {
    type Output = Line;

    fn bitor(self, p: Point) -> Self::Output {
        // This is just the grade-1 part of the geometric product `l * p`
        Self::Output {
            e0: self.e2 * p.e20 - self.e1 * p.e01,
            e1: -self.e2 * p.e12,
            e2: self.e1 * p.e12,
        }
    }
}

/// "Meet" two lines at a point (wedge product) `l1 ^ l2`.
impl BitXor for Line {
    type Output = Point;

    fn bitxor(self, rhs: Self) -> Self::Output {
        // This is just the grade-2 part of the geometric product `l1 * l2`
        Self::Output {
            e12: self.e0 * rhs.e1 - self.e1 * rhs.e0,
            e20: self.e2 * rhs.e0 - self.e0 * rhs.e2,
            e01: self.e1 * rhs.e2 - self.e2 * rhs.e1,
        }
    }
}

/// Returns the point that is dual to this line `!l`.
impl Not for Line {
    type Output = Point;

    fn not(self) -> Self::Output {
        Self::Output {
            e12: self.e0,
            e20: self.e1,
            e01: self.e2,
        }
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "e0: {}, e1: {}, e2: {}", self.e0, self.e1, self.e2)
    }
}
