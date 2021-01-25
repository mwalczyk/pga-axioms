use crate::line::Line;
use crate::point::Point;

/// Returns the distance between two points `p1` and `p2`. Algebraically, this is the
/// length (norm) of the line between the two points, which can be found via the
/// regressive ("vee") product `p1 & p2`.
pub fn dist_point_to_point(p1: &Point, p2: &Point) -> f32 {
    let p1 = p1.normalized();
    let p2 = p2.normalized();

    (p1 & p2).norm()
}

/// Returns the distance between point `p` and line `l`. Algebraically, this is
/// the highest grade part of the geometric product `p * l`, or equivalently, `p ^ l`.
pub fn dist_point_to_line(p: &Point, l: &Line) -> f32 {
    let p = p.normalized();
    let l = l.normalized();

    // Technically, this is a trivector e012, but we simply return a scalar
    p.e12 * l.e0 + p.e20 * l.e1 + p.e01 * l.e2
}

/// Returns the angle between two lines `l1` and `l2`. Algebraically, the cosine of the
/// angle between the two lines is given by their inner product `l1 | l2`.
pub fn angle(l1: &Line, l2: &Line) -> f32 {
    let l1 = l1.normalized();
    let l2 = l2.normalized();

    let theta = l1 | l2;
    theta.acos()
}

/// Returns the angle bisector of two lines `l1` and `l2`.
pub fn bisector(l1: &Line, l2: &Line) -> Line {
    let l1 = l1.normalized();
    let l2 = l2.normalized();

    l1 + l2
}

/// Computes the product `(p | l) * l`, i.e. the projection of point `p` onto
/// line `l`. The result is a new point that lies on `l`. The perpendicular to
/// `l` that runs through this new point will also pass through `p`.
pub fn project_point_onto_line(p: &Point, l: &Line) -> Point {
    Point::new(
        (l.e1 * l.e1 + l.e2 * l.e2) * p.e12,
        (l.e2 * p.e20 - l.e1 * p.e01) * l.e2 - l.e0 * l.e1 * p.e12,
        (l.e1 * p.e01 - l.e2 * p.e20) * l.e1 - l.e0 * l.e2 * p.e12,
    )
}

/// Computes the product `(p | l) * p`, i.e. the projection of line `l` onto
/// point `p`. The result is a new line that runs parallel to `l` and passes
/// through `p`.
pub fn project_line_onto_point(l: &Line, p: &Point) -> Line {
    // Note how this does not depend at all on the e0 component of the line,
    // which makes sense!
    Line::new(
        (l.e1 * p.e20 + l.e2 * p.e01) * -p.e12,
        l.e1 * p.e12 * p.e12,
        l.e2 * p.e12 * p.e12,
    )
}

/// Computes the line orthogonal to line `l` that passes through point `p`. Algebraically,
/// this is simply the inner product `p | l`, or alternatively, the lowest grade part of
/// the product `p * l`.
pub fn ortho(p: &Point, l: &Line) -> Line {
    *p | *l
}

pub fn reflect_point_across_line(p: &Point, l: &Line) -> Point {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_projections() {
        let p = Point::new(1.0, 2.0, 3.0);
        let l = Line::new(4.0, 5.0, 6.0);
        let result = project_point_onto_line(&p, &l);
        println!("Projection of p onto l, (p | l) * l = {:?}", result);
        // Should be: Point { 61, -38, -9 }

        let result = project_line_onto_point(&l, &p);
        println!("Projection of l onto p, (p | l) * p = {:?}", result);
        // Should be: Line { -28, 5, 6  }
    }
}
