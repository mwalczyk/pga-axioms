use crate::line::Line;
use crate::point::Point;
use crate::multivector::Multivector;

/// Returns the distance between two points `p1` and `p2`. Algebraically, this is the
/// length (norm) of the line between the two points, which can be found via the
/// regressive ("vee") product `p1 & p2`.
pub fn dist_point_to_point(p1: &Multivector, p2: &Multivector) -> f32 {
    let p1 = p1.normalized();
    let p2 = p2.normalized();

    (p1 & p2).norm()
}

/// Returns the distance between point `p` and line `l`. Algebraically, this is
/// the highest grade part of the geometric product `p * l`, or equivalently, `p ^ l`.
pub fn dist_point_to_line(p: &Multivector, l: &Multivector) -> f32 {
    let p = p.normalized();
    let l = l.normalized();

    // Technically, this is a trivector e012, but we simply return a scalar
    (p ^ l).e012()
}

/// Returns the angle between two lines `l1` and `l2`. Algebraically, the cosine of the
/// angle between the two lines is given by their inner product `l1 | l2`.
pub fn angle(l1: &Multivector, l2: &Multivector) -> f32 {
    let l1 = l1.normalized();
    let l2 = l2.normalized();

    let theta = (l1 | l2).scalar();
    theta.acos()
}

/// Returns the angle bisector of two lines `l1` and `l2`.
pub fn bisector(l1: &Multivector, l2: &Multivector) -> Multivector {
    let l1 = l1.normalized();
    let l2 = l2.normalized();

    l1 + l2
}


/// Projects multivector `a` onto multivector `b`. The geometric meaning depends
/// on the grade and order of the arguments. For example:
///
/// `project(p, l)` with a point `p` and line `l`:
///
/// Computes the product `(p | l) * l`, i.e. the projection of point `p` onto
/// line `l`. The result is a new point that lies on `l`. The perpendicular to
/// `l` that runs through this new point will also pass through `p`.
///
/// `project(l, p)` with a line `l` and point `p`:
///
/// Computes the product `(l | p) * p`, i.e. the projection of line `l` onto
/// point `p`. The result is a new line that runs parallel to `l` and passes
/// through `p`.
pub fn project(a: &Multivector, b: &Multivector) -> Multivector {
    // Note how this does not depend at all on the e0 component of the line,
    // which makes sense (when we are projecting a line onto a point)
    ((*a) | (*b)) * (*b)
}

/// Computes the line orthogonal to line `l` that passes through point `p`. Algebraically,
/// this is simply the inner product `p | l`, or alternatively, the lowest grade part of
/// the product `p * l`.
pub fn orthogonal(p: &Multivector, l: &Multivector) -> Multivector {
    (*p) | (*l)
}

/// Reflects the multivector `a` across the multivector `b`. For example, if `a` is a point
/// and `b` is a line, the result will be a new point reflected across the line.
pub fn reflect(a: &Multivector, b: &Multivector) -> Multivector {
    b * a * b
}

/// Rotates the multivector by `angle` radians about the point `<x, y>`. Algebraically,
/// this is equivalent to computing the "sandwich product" `R * m * ~R`.
#[allow(non_snake_case)]
pub fn rotate(m: &Multivector, angle: f32, x: f32, y: f32) -> Self {
    let R = Multivector::rotor(angle, x, y);
    R * (*m) * R.conjugation()
}

/// Translates the multivector by an amount `<x, y>`. Algebraically, this is equivalent to
/// computing the "sandwich product" `T * m * ~T`.
#[allow(non_snake_case)]
pub fn translate(m: &Multivector, x: f32, y: f32) -> Self {
    let T = Multivector::translator(x, y);
    T * (*m) * T.conjugation()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_projections() {
        let mut p = Multivector::point(1.0, 2.0);
        p[6] = 3.0;
        let l = Multivector::line(4.0, 5.0, 6.0);
        let result = project(&p, &l);
        println!("Projection of p onto l, (p | l) * l = {:?}", result);
        // Should be: Multivector { coeff: [0.0, 0.0, 0.0, 0.0, -78.0, -87.0, 123.0, 0.0] }

        let result = project(&l, &p);
        println!("Projection of l onto p, (p | l) * p = {:?}", result);
        // Should be: Multivector { coeff: [0.0, 42.0, -36.0, -45.0, 0.0, 0.0, 0.0, 0.0] }
    }
}
