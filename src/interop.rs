use crate::axioms;
use crate::geometry;
use crate::multivector::Multivector;
use crate::utils;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// Convenience line struct for passing data to-from WASM. Represents the line
/// `ax + by + c = 0`.
#[wasm_bindgen]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Line {
    pub a: f32,
    pub b: f32,
    pub c: f32,
}

#[wasm_bindgen]
impl Line {
    #[wasm_bindgen(constructor)]
    pub fn new(a: f32, b: f32, c: f32) -> Self {
        Self { a, b, c }
    }
}

impl Into<Multivector> for Line {
    fn into(self) -> Multivector {
        Multivector::line(self.a, self.b, self.c)
    }
}

impl From<Multivector> for Line {
    fn from(multivector: Multivector) -> Self {
        Self::new(multivector.e1(), multivector.e2(), multivector.e0())
    }
}

/// Convenience point struct for passing data to-from WASM.
#[wasm_bindgen]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[wasm_bindgen]
impl Point {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Into<Multivector> for Point {
    fn into(self) -> Multivector {
        Multivector::point(self.x, self.y)
    }
}

impl From<Multivector> for Point {
    fn from(multivector: Multivector) -> Self {
        Self::new(multivector.e20(), multivector.e01())
    }
}

#[derive(Serialize, Deserialize)]
pub struct AxiomResult {
    pub line: Line,
    positive: Vec<Point>,
    negative: Vec<Point>,
}

impl AxiomResult {
    pub fn new(line: &Line, positive: &Vec<Point>, negative: &Vec<Point>) -> Self {
        Self {
            line: *line,
            positive: positive.clone(),
            negative: negative.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Paper {
    pub ul: Point,
    pub ur: Point,
    pub lr: Point,
    pub ll: Point,
}
#[wasm_bindgen]
impl Paper {
    /// When the application starts, it will construct a new instance of a `Paper` object
    /// on the Javascript side, based on the dimensions of the canvas.
    #[wasm_bindgen(constructor)]
    pub fn new(ul: Point, ur: Point, lr: Point, ll: Point) -> Self {
        Self { ul, ur, lr, ll }
    }
}

impl Paper {
    fn points(&self) -> Vec<Point> {
        vec![self.ul, self.ur, self.lr, self.ll]
    }

    pub fn intersect(&self, crease: &Multivector) -> (Vec<Point>, Vec<Point>) {
        // Convert points to full multivectors before continuing
        let mut vertices: Vec<Multivector> =
            self.points().iter().map(|&vertex| vertex.into()).collect();

        // Which side of the crease is each corner on?
        let signs = vertices
            .iter()
            .map(|p| utils::sign_with_tolerance(geometry::dist_point_to_line(p, crease)))
            .collect::<Vec<_>>();

        let mut cut_points = Vec::new();

        for vertex_index in 0..vertices.len() {
            // The index of the other vertex that forms this edge of the paper
            let pair_index = (vertex_index + 1) % vertices.len();

            // Always include the corner points
            cut_points.push(vertices[vertex_index]);

            // Check if the two vertices that form this edge are on opposite sides of the crease
            // (and not *exactly* incident to it)
            if signs[vertex_index] != 0.0
                && signs[pair_index] != 0.0
                && (signs[vertex_index] != signs[pair_index])
            {
                // Insert cut point (where this face's edge intersects the crease)
                let edge = vertices[vertex_index].join(&vertices[pair_index]);
                let intersection = edge.meet(crease);

                cut_points.push(intersection)
            }
        }

        // Which side of the crease is each cut point on (recalculate?
        let signs = cut_points
            .iter()
            .map(|p| utils::sign_with_tolerance(geometry::dist_point_to_line(p, crease)))
            .collect::<Vec<_>>();

        let mut positive = Vec::new();
        let mut negative = Vec::new();

        for (index, sign) in signs.into_iter().enumerate() {
            // Normalize the point
            let mut point = cut_points[index].normalized();

            // In both cases below, we normalize the point and divide by its e12
            // (homogeneous coordinate) before returning - the only difference is,
            // for one set of cut points, we reflect them across the crease first
            // (to simulate folding behavior)
            if sign <= 0.0 || sign.abs() < 0.001 {
                point = geometry::reflect(&point, crease);
                point /= point.e12();
                negative.push(point.into());
            }

            if sign >= 0.0 || sign.abs() < 0.001 {
                point /= point.e12();
                positive.push(point.into());
            }
        }

        (positive, negative)
    }
}

pub fn bundle_results(paper: &Paper, crease: &Multivector) -> JsValue {
    // Find where the crease intersects the paper and return
    let (positive, negative) = paper.intersect(crease);
    let line = Line::new(crease.e1(), crease.e2(), crease.e0());
    let result = AxiomResult::new(&line, &positive, &negative);

    JsValue::from_serde(&result).unwrap()
}

#[wasm_bindgen]
pub fn axiom_1(paper: &Paper, p0: Point, p1: Point) -> JsValue {
    let crease = axioms::axiom_1(&p0.into(), &p1.into());
    bundle_results(paper, &crease)
}

#[wasm_bindgen]
pub fn axiom_2(paper: &Paper, p0: Point, p1: Point) -> JsValue {
    let crease = axioms::axiom_2(&p0.into(), &p1.into());
    bundle_results(paper, &crease)
}

#[wasm_bindgen]
pub fn axiom_3(
    paper: &Paper,
    l0_src: Point,
    l0_dst: Point,
    l1_src: Point,
    l1_dst: Point,
) -> JsValue {
    let l0 = Into::<Multivector>::into(l0_src) & Into::<Multivector>::into(l0_dst);
    let l1 = Into::<Multivector>::into(l1_src) & Into::<Multivector>::into(l1_dst);
    let crease = axioms::axiom_3(&l0, &l1);

    // Sometimes, this axiom will return a line at infinity (i.e. a line whose only non-zero
    // coefficient is e0) - while mathematically correct, we want to avoid passing this to the
    // drawing application
    //
    // This occurs when, for example, the two lines are parallel but oriented in opposite
    // directions
    if crease.norm().abs() < 0.001 {
        return JsValue::null();
    }
    bundle_results(paper, &crease)
}

#[wasm_bindgen]
pub fn axiom_4(paper: &Paper, p0: Point, l0_src: Point, l0_dst: Point) -> JsValue {
    // Join the two segment endpoints to form the line between them
    let l = Into::<Multivector>::into(l0_src) & Into::<Multivector>::into(l0_dst);
    let crease = axioms::axiom_4(&p0.into(), &l);
    bundle_results(paper, &crease)
}

#[wasm_bindgen]
pub fn axiom_5(paper: &Paper, p0: Point, p1: Point, l0_src: Point, l0_dst: Point) -> JsValue {
    // Join the two segment endpoints to form the line between them
    let l = Into::<Multivector>::into(l0_src) & Into::<Multivector>::into(l0_dst);
    let maybe_crease = axioms::axiom_5(&p0.into(), &p1.into(), &l);

    if let Some(crease) = maybe_crease {
        return bundle_results(paper, &crease);
    }

    JsValue::null()
}
