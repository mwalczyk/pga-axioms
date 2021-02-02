use crate::geometry;
use crate::multivector::*;
use web_sys::console::dir;

/// Given two points `p0` and `p1`, there is a unique fold that passes through both of them.
pub fn axiom_1(p0: &Multivector, p1: &Multivector) -> Multivector {
    let mut crease = (*p0) & (*p1);
    crease.normalized()
}

/// Given two points `p0` and `p1`, there is a unique fold that places `p0` onto `p1`.
pub fn axiom_2(p0: &Multivector, p1: &Multivector) -> Multivector {
    let l = (*p0) & (*p1);
    let midpoint = (*p0 + *p1).normalized();
    let crease = l | midpoint;
    crease.normalized()
}

/// Given two lines `l0` and `l1`, there is a fold that places `l0` onto `l1`.
pub fn axiom_3(l0: &Multivector, l1: &Multivector) -> Multivector {
    let crease = l0.normalized() + l1.normalized();
    crease.normalized()

    // There are two possible solutions (two angle bisectors) - the one above or:
    // let crease = crease | (l0 ^ l1);
}

/// Given a point `p` and a line `l`, there is a unique fold perpendicular to `l` that passes
/// through point `p`.
pub fn axiom_4(p: &Multivector, l: &Multivector) -> Multivector {
    // Simply take the inner product between l and p to construct the perpendicular that passes
    // through p
    let crease = (*l) | (*p);
    crease.normalized()
}

/// Given two points `p0` and `p1` and a line `l`, there is a fold that places `p0` onto `l` and
/// passes through `p1`.
pub fn axiom_5(p0: &Multivector, p1: &Multivector, l: &Multivector) -> Option<Multivector> {
    // First, calculate distance between `p0` and `p1` (i.e. the radius of the circle centered
    // on `p1` that is tangent to `p0`)
    let r = geometry::dist_point_to_point(p0, p1);

    // Then, calculate the (shortest) distance from the line to the center of the circle
    let dist_from_line_to_center = geometry::dist_point_to_line(p1, &l);

    // Exit early if no intersection is found
    if dist_from_line_to_center > r {
        return None;
    }

    // The product `(l * p1)` constructs a line perpendicular to `l` that passes through `p1` - we
    // then "meet" this line with the original line to calculate the point of intersection
    let ortho = geometry::orthogonal(p1, &l);
    let mut perp = geometry::intersect_lines(&ortho, &l).normalized();

    // "Flip" x/y if e12 is negative
    perp = perp * perp.e12();

    // Pythagoras' theorem: find the length of the third side of the triangle
    // whose hypotenuse is `r` and other side is `dist_from_line_to_center`
    //
    // We don't need to take the absolute value of the value inside of the sqrt operation
    // (as in Enki's original code) since we check above that `dist_from_line_to_center`
    // is less than (or equal to) the radius `r`
    let d = (r * r - dist_from_line_to_center * dist_from_line_to_center).sqrt();

    // Multiplying a line by e012 has the effect of "pulling out" its direction vector,
    // represented by an ideal point (i.e. a point at infinity) = this is also known as
    // metric polarity
    let mut direction = (*l) * e012;
    direction /= direction.ideal_norm();

    // If l isn't normalized, we have to do the above or just:
    // direction = l.normalize() * e012;

    // If there are 2 intersections (i.e., the line "pierces through" the circle), then you
    // can choose either point of intersection (both are valid) - simply change the `+` to a
    // `-` (or vice-versa)
    direction *= d;
    let T = Multivector::translator(direction.e20(), direction.e01());
    //let T = direction * (d / 2.0) + 1.0;

    // The point of intersection can be found by translating the point `perp` along the line
    // `l` by an amount `d` (in either direction, in the case of 2 intersections)
    let intersection = geometry::translate(&perp, direction.e20(), direction.e01()); //
                                                                                     //let intersection = T * perp * T.conjugation();

    // A new line joining the point of intersection and p0
    let m = intersection.join(p0);

    // A line perpendicular to m that passes through p1: note that this line should always
    // pass through the midpoint of the line segment `intersection - p0`
    let crease = geometry::orthogonal(p1, &m);

    Some(crease.normalized())
}

/// Given two points `p0` and `p1` and two lines `l0` and `l1`, there is a fold that places `p0` onto
/// `l0` and `p1` onto `l1`.
pub fn axiom_6(
    p0: &Multivector,
    p1: &Multivector,
    l0: &Multivector,
    l1: &Multivector,
) -> Multivector {
    unimplemented!();
}

/// Given one point `p` and two lines `l0` and `l1`, there is a fold that places `p` onto `l0`
/// and is perpendicular to `l1`.
pub fn axiom_7(p: &Multivector, l0: &Multivector, l1: &Multivector) -> Multivector {
    unimplemented!();
}
