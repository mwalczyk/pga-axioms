use crate::line::Line;
use crate::point::Point;

pub struct Translator {
    d: f32,
    p: Point,
}

impl Translator {
    /// Construct a translator that represents a translation along vector `<x, y>` by
    /// a distance `dist`.
    pub fn new(dist: f32, x: f32, y: f32) -> Translator {
        Translator::from_dist_and_point(dist, &Point::ideal(x, y))
    }

    pub fn from_dist_and_point(dist: f32, p: &Point) -> Translator {
        // Note that we ignore the e12 (homogeneous) component of the point below
        // if the point isn't already an ideal point
        Translator {
            d: dist * 0.5,
            p: Point::ideal(p.e20, p.e01)
        }
    }

    /// Translates the given point.
    pub fn move_point(&self, rhs: &Point) -> Point {
        unimplemented!();
    }

    /// Translates the given line.
    pub fn move_line(&self, rhs: &Line) -> Line {
        unimplemented!();
    }
}