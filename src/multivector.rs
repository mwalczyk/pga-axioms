#![allow(non_upper_case_globals)]
use std::fmt::Display;
use std::ops::{
    Add, BitAnd, BitOr, BitXor, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Not, Sub,
};

/// A string representation of all of the basis elements of 2D PGA.
pub const BASIS_ELEMENTS: &'static [&'static str] =
    &["1", "e0", "e1", "e2", "e01", "e20", "e12", "e012"];

/// The total number of basis elements in 2D PGA (i.e. the size of the algebra).
pub const BASIS_COUNT: usize = BASIS_ELEMENTS.len();

/// Basis elements are available as global constants.
pub const e0: Multivector = Multivector::basis(1, 1.0);
pub const e1: Multivector = Multivector::basis(2, 1.0);
pub const e2: Multivector = Multivector::basis(3, 1.0);
pub const e01: Multivector = Multivector::basis(4, 1.0);
pub const e20: Multivector = Multivector::basis(5, 1.0);
pub const e12: Multivector = Multivector::basis(6, 1.0);
pub const e012: Multivector = Multivector::basis(7, 1.0);

/// We also include the various permutations of the basis elements above as global constants.
pub const e10: Multivector = Multivector::basis(4, -1.0);
pub const e02: Multivector = Multivector::basis(5, -1.0);
pub const e21: Multivector = Multivector::basis(6, -1.0);
pub const e021: Multivector = Multivector::basis(7, -1.0);
pub const e102: Multivector = Multivector::basis(7, -1.0);
pub const e210: Multivector = Multivector::basis(7, -1.0);
pub const e120: Multivector = Multivector::basis(7, 1.0);
pub const e201: Multivector = Multivector::basis(7, 1.0);

/// An enum representing the grade of a part of a multivector in 2D PGA.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Grade {
    Scalar = 0,
    Vector,
    Bivector,
    Trivector,
}

impl Grade {
    pub fn relevant_blade_indices(&self) -> Vec<usize> {
        match *self {
            Grade::Scalar => vec![0],
            Grade::Vector => vec![1, 2, 3],
            Grade::Bivector => vec![4, 5, 6],
            Grade::Trivector => vec![7],
        }
    }

    /// Given the index of a particular basis blade, return the grade of that
    /// element. For example, indices 1, 2, and 3 would all return `Vector`,
    /// since these correspond to the grade-1 elements e0, e1, and e2,
    /// respectively.
    pub fn from_blade_index(index: usize) -> Result<Self, &'static str> {
        match index {
            0 => Ok(Grade::Scalar),
            1..=3 => Ok(Grade::Vector),
            4..=6 => Ok(Grade::Bivector),
            7 => Ok(Grade::Trivector),
            _ => Err("Invalid index: should be between 0-7 (inclusive)"),
        }
    }
}

/// All of the possible grades in 2D PGA.
const GRADES: [Grade; 4] = [
    Grade::Scalar,
    Grade::Vector,
    Grade::Bivector,
    Grade::Trivector,
];

/// A multivector is a general element of the algebra R(2, 0, 1), i.e. 2D projective geometric
/// algebra (PGA). For all intents and purposes, it can be thought of as an 8-element array of
/// coefficients with "special" functionality. The coefficients correspond to the 8 basis
/// elements of 2D PGA. For example, let the coefficients be denoted `[A, B, C, D, E, F, G, H]`.
/// Then, the corresponding multivector can be written as:
///
///     `A + B*e0 + C*e1 + D*e2 + E*e01 + F*e20 + G*e12 + H*e012`
///
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Multivector {
    /// The coefficients of this multivector.
    coeff: [f32; BASIS_COUNT],
}

impl Multivector {
    /// Constructs a new multivector with the specified coefficients.
    pub fn with_coefficients(coeff: &[f32; BASIS_COUNT]) -> Self {
        Self {
            coeff: coeff.clone(),
        }
    }

    /// Constructs the zero multivector (i.e. a multivector with all coefficients set to zero).
    pub const fn zeros() -> Self {
        Self {
            coeff: [0.0; BASIS_COUNT],
        }
    }

    /// Constructs a multivector with every coefficient set to 1.
    pub const fn ones() -> Self {
        Self {
            coeff: [1.0; BASIS_COUNT],
        }
    }

    /// In PGA, the origin is represented by the e12 bivector.
    pub const fn origin() -> Self {
        e12
    }

    /// Equivalent to `Multivector::zeros()`.
    pub const fn new() -> Self {
        Self::zeros()
    }

    /// Constructs a multivector representing a basis element of 2D PGA.
    pub const fn basis(index: usize, coeff: f32) -> Self {
        let mut multivector = Self::zeros();
        multivector.coeff[index] = coeff;
        multivector
    }

    /// Constructs a multivector that represents a Euclidean point (grade-2 element) with
    /// coordinates `<x, y>`.
    pub fn point(x: f32, y: f32) -> Self {
        let mut multivector = Self::zeros();
        multivector[4] = y; // e01, which is dual to e2
        multivector[5] = x; // e20, which is dual to e1
        multivector[6] = 1.0;
        multivector
    }

    /// Constructs a multivector that represents an ideal point (i.e. a point at infinity,
    /// grade-2 element) with ideal coordinates `<x, y>`. This can (for all intents and purposes)
    /// be thought of as a 2D "vector" in traditional linear algebra.
    pub fn ideal_point(x: f32, y: f32) -> Self {
        let mut multivector = Self::zeros();
        multivector[4] = y; // e01, which is dual to e2
        multivector[5] = x; // e20, which is dual to e1
                            // Technically, this is unnecessary, but we show it for illustration purposes
        multivector[6] = 0.0;
        multivector
    }

    /// Construct a multivector that represents a line (grade-1 element) with the equation:
    /// `ax + by + c = 0`.
    pub fn line(a: f32, b: f32, c: f32) -> Self {
        let mut multivector = Self::zeros();
        multivector[1] = c; // e0
        multivector[2] = a; // e1
        multivector[3] = b; // e2
        multivector
    }

    /// Returns a multivector that represents a rotor that performs a rotation by `angle`
    /// radians about the Euclidean point `<cx, cy>` (`c` for "center" of rotation).
    pub fn rotor(angle: f32, cx: f32, cy: f32) -> Self {
        let point = Self::point(cx, cy);
        let half_angle = angle * 0.5;
        point * (half_angle).sin() + (half_angle).cos()
    }

    /// Returns a multivector that represents a translator that performs a translation by
    /// `<delta_x, delta_y>` units.
    pub fn translator(delta_x: f32, delta_y: f32) -> Self {
        // Use the formula: 1 + (d / 2) * P_inf - note, however, that this constructs
        // a translator that translates objects in a direction orthogonal to P_inf, which
        // is why we construct T with the negative y-coordinate below
        let direction = Self::ideal_point(delta_x, -delta_y);
        let _amount = direction.ideal_norm();

        // This simplifies to the final return statement:
        // (direction / amount) * (amount / 2.0) + 1.0
        direction * 0.5 + 1.0
    }

    /// Returns the scalar part of the multivector.
    pub fn scalar(&self) -> f32 {
        self[0]
    }

    /// Returns the e0 part of the multivector.
    pub fn e0(&self) -> f32 {
        self[1]
    }

    /// Returns the e1 part of the multivector.
    pub fn e1(&self) -> f32 {
        self[2]
    }

    /// Returns the e2 part of the multivector.
    pub fn e2(&self) -> f32 {
        self[3]
    }

    /// Returns the e01 part of the multivector.
    pub fn e01(&self) -> f32 {
        self[4]
    }

    /// Returns the e20 part of the multivector.
    pub fn e20(&self) -> f32 {
        self[5]
    }

    /// Returns the e12 part of the multivector.
    pub fn e12(&self) -> f32 {
        self[6]
    }

    /// Returns the e012 part of the multivector.
    pub fn e012(&self) -> f32 {
        self[7]
    }

    /// Returns the grade-`n` part of the multivector. For example, calling this function
    /// with `Grade::Vector` will return a new multivector with all of its coefficients
    /// set to zero except those corresponding to e0, e1, and e2 (the grade-1 parts of
    /// the multivector).
    pub fn grade_selection(&self, grade: Grade) -> Self {
        let mut multivector = self.clone();

        // Figure out which indices to "keep" (i.e. the coefficients that are part
        // of the desired grade)
        let indices_to_keep = grade.relevant_blade_indices();

        // Set all other coefficients to zero
        for index in 0..BASIS_COUNT {
            if !indices_to_keep.contains(&index) {
                multivector[index] = 0.0;
            }
        }

        multivector
    }

    /// Applies a function to each element of the multivector.
    pub fn apply(&mut self, f: fn(f32) -> f32) {
        for element in self.coeff.iter_mut() {
            *element = f(*element);
        }
    }

    /// Applies a function to each of the elements that make up the grade-`n` part of the
    /// multivector.
    pub fn apply_to_grade(&mut self, grade: Grade, f: fn(f32) -> f32) {
        for index in grade.relevant_blade_indices() {
            self[index] = f(self[index]);
        }
    }

    /// Computes the Clifford conjugate of the multivector. This is the superposition
    /// of the grade involution and reversion operations. It is defined as
    /// `(-1)^(k * (k + 1) / 2) * a_k`.
    pub fn conjugation(&self) -> Self {
        // Negate all but the scalar and trivector parts of the multivector
        let mut multivector = self.clone();
        multivector[1] = -self[1];
        multivector[2] = -self[2];
        multivector[3] = -self[3];
        multivector[4] = -self[4];
        multivector[5] = -self[5];
        multivector[6] = -self[6];
        multivector
    }

    /// Computes the grade involution (also know as the "main involution") of
    /// the multivector, which is defined as `(-1)^k * a_k`. In the case of 2D PGA,
    /// this means that the vector and trivector parts of the multivector get negated,
    /// while the scalar and bivector parts remain the same. The grade involution
    /// operator is often denoted with the symbol `^` (above the multivector).
    pub fn grade_involution(&self) -> Self {
        // Note how only the odd graded parts of the multivector are negated
        let mut multivector = self.clone();
        // Grade-1 part
        multivector[1] = -self[1];
        multivector[2] = -self[2];
        multivector[3] = -self[3];
        // Grade-3 part
        multivector[7] = -self[7];
        multivector
    }

    /// Reverses each element of the multivector. For example, `e20 = e2 * e0` becomes
    /// `e02 = e0 * e2`. Effectively, this negates the bivector and trivector parts
    /// of the multivector, leaving all other parts the same. The reversion operator
    /// is often denoted with the symbol `~`.
    ///
    /// The formula for reversion is `(-1)^(k * (k - 1) / 2) * a_k `.
    pub fn reversion(&self) -> Self {
        // Using the formula above, we see that only the grade-2 and grade-3 parts
        // of the multivector are affected (which makes sense - the reverse of a
        // scalar or vector is just the scalar or vector itself)
        let mut multivector = self.clone();
        multivector[4] = -self[4];
        multivector[5] = -self[5];
        multivector[6] = -self[6];
        multivector[7] = -self[7];
        multivector
    }

    /// Computes the inverse `A^-1` of this multivector under the geometric product, such
    /// that `A * A^-1 = 1`. The inverse is calculated by taking repeated involutions
    /// until the denominator becomes a scalar. This is identical to the process of
    /// finding the inverse of a complex number but with more steps (i.e. involutions).
    ///
    /// Note that `A * A^-1 = A^-1 * A = 1` (i.e. we can multiply by the inverse on either
    /// the left or the right side - it doesn't matter).
    ///
    /// Reference: http://repository.essex.ac.uk/17282/1/TechReport_CES-534.pdf
    pub fn inverse(&self) -> Self {
        // Note that in the calculations below, `den` will always be a scalar
        let num = self.conjugation() * self.grade_involution() * self.reversion();
        let den = (*self) * num;
        let inverse = num / den.scalar();
        inverse
    }

    /// An alternative, more verbose way of calculating the dual of this multivector.
    pub fn dual(&self) -> Self {
        !(*self)
    }

    /// Computes the join of two multivectors, which is the dual of the outer product
    /// of the duals of the original two multivectors: `!(!A ^ !B)`. The join operation
    /// can be used, for example, to construct the line (grade-1 element) that "joins"
    /// (i.e. passes through) two points (grade-2 elements).
    ///
    /// Notationally, we are working in a *dual* projectivized space, so the "wedge"
    /// operator corresponds to "meet" and the "vee" operator corresponds to "join".
    ///
    /// Also note that this version of the "join" operator is orientation preserving
    /// and follows the formulas laid out in Dorst's "PGA4CS" book. In particular,
    /// the order of the arguments is swapped. In 2D, we don't have to worry about
    /// any extraneous sign-flips (as mentioned in the book), since the "dual" and
    /// "undual" operations are the exact same, i.e. `!(!a) = a` for any multivector
    /// in 2D PGA.
    pub fn join(&self, rhs: &Self) -> Self {
        let a = *self;
        let b = *rhs;
        !(!b ^ !a)
    }

    /// Computes the meet of two multivectors. This can be used to compute incidence
    /// relations. For example, the meet of two lines is their point of intersection
    /// (which will be an ideal "point at infinity" if the lines are parallel).
    pub fn meet(&self, rhs: &Self) -> Self {
        let a = *self;
        let b = *rhs;
        a ^ b
    }

    /// Returns the norm of the multivector.
    ///
    /// The norm is `|A| = √⟨A * ~A⟩₀`, where `~` is the reversion (or conjugation)
    /// operator. The reversion operator makes similar elements cancel each other with
    /// a positive or zero scalar, so the square root always exists. For example, a
    /// point times itself may, in general, be a negative number. But a point times
    /// its reversal will always be non-negative scalar.
    ///
    /// For a multivector, we compute the square root of the sum of the squares
    /// of the norm of each component blade. This leads to the formula above.
    ///
    /// A k-vector that is normalized will square to +/- 1.
    pub fn norm(&self) -> f32 {
        // TODO: is the `abs()` necessary here? Maybe it only matters for algebras with
        //   one or more negative dimensions (like CGA)
        let multivector = (*self) * self.conjugation();
        multivector.scalar().abs().sqrt()
    }

    /// Returns the ideal norm of the multivector.
    pub fn ideal_norm(&self) -> f32 {
        self.dual().norm()
    }

    /// Returns a normalized version of the multivector.
    pub fn normalized(&self) -> Self {
        (*self) / self.norm()
    }

    /// Normalizes the multivector (in-place).
    pub fn normalize(&mut self) {
        *self /= self.norm();
    }
}

/// Returns an immutable reference to the multivector's coefficient at `index`.
/// For example, `a[2]` would correspond to the e1 component and `a[7]` would
/// correspond to the e012 component.
impl Index<usize> for Multivector {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.coeff[index]
    }
}

/// Returns a mutable reference to the multivector's coefficient at `index`.
/// For example, `a[2]` would correspond to the e1 component and `a[7]` would
/// correspond to the e012 component.
impl IndexMut<usize> for Multivector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.coeff[index]
    }
}

/// Computes the join between two multivectors `A & B`.
impl BitAnd for Multivector {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.join(&rhs)
    }
}

/// Computes the inner product between two multivectors `A | B`. This can
/// be calculated by distributing the geometric product across the k-vectors
/// of each multivector (rather than each basis blade). For example, we
/// multiply the grade-0 part of `A` with the grade-0, grade-1, grade-2,
/// and grade-3 parts of `B` (and repeat for the other 3 parts of `A`).
/// For each such pairing, the inner product is the `|k - s|` grade part
/// of the result. Summing together all of these intermediate results
/// gives us the full inner product between `A` and `B`.
///
/// In the literature, this is sometimes referred to as the "symmetric
/// inner product" (to distinguish it from left or right contractions,
/// for example).
impl BitOr for Multivector {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let a = self[0];
        let b = self[1];
        let c = self[2];
        let d = self[3];
        let e = self[4];
        let f = self[5];
        let g = self[6];
        let h = self[7];

        let i = rhs[0];
        let j = rhs[1];
        let k = rhs[2];
        let l = rhs[3];
        let m = rhs[4];
        let n = rhs[5];
        let o = rhs[6];
        let p = rhs[7];

        let mut multivector = Self::zeros();
        multivector[0] = a * i + c * k + d * l - g * o;
        multivector[1] = b * i + a * j + e * k - f * l + d * n - c * m - h * o - g * p; // e0
        multivector[2] = c * i + a * k + g * l - d * o; // e1
        multivector[3] = d * i + a * l - g * k + c * o;
        multivector[4] = e * i + h * l + a * m + d * p; // e01
        multivector[5] = f * i + h * k + a * n + c * p; // e20
        multivector[6] = g * i + a * o; // e12
        multivector[7] = h * i + a * p; // e012
        multivector
    }
}

/// Computes the outer product between two multivectors `A ^ B`. This can
/// be calculated by distributing the geometric product across the k-vectors
/// of each multivector (rather than each basis blade). For example, we
/// multiply the grade-0 part of `A` with the grade-0, grade-1, grade-2,
/// and grade-3 parts of `B` (and repeat for the other 3 parts of `A`).
/// For each such pairing, the outer product is the `|k + s|` grade part
/// of the result. Summing together all of these intermediate results
/// gives us the full outer product between `A` and `B`.
///
/// In the literature, this is sometimes referred to as the "exterior" or
/// "wedge product."
impl BitXor for Multivector {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let a = self[0];
        let b = self[1];
        let c = self[2];
        let d = self[3];
        let e = self[4];
        let f = self[5];
        let g = self[6];
        let h = self[7];

        let i = rhs[0];
        let j = rhs[1];
        let k = rhs[2];
        let l = rhs[3];
        let m = rhs[4];
        let n = rhs[5];
        let o = rhs[6];
        let p = rhs[7];

        let mut multivector = Self::zeros();
        multivector[0] = a * i;
        multivector[1] = b * i + a * j;
        multivector[2] = c * i + a * k;
        multivector[3] = d * i + a * l;
        multivector[4] = e * i + b * k - c * j + a * m;
        multivector[5] = f * i + d * j - b * l + a * n;
        multivector[6] = g * i + c * l - d * k + a * o;
        multivector[7] = h * i + e * l + f * k + g * j + b * o + c * n + d * m + a * p;
        multivector
    }
}

/// Adds two multivectors component-wise `A + B`.
impl Add for Multivector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut multivector = Self::zeros();
        for i in 0..BASIS_COUNT {
            multivector[i] = self[i] + rhs[i];
        }
        multivector
    }
}

/// Adds a scalar to the multivector.
impl Add<f32> for Multivector {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        let mut multivector = self.clone();
        multivector[0] += rhs;
        multivector
    }
}

/// Multiplies a multivector by another multivector's inverse under the
/// geometric product `A * B^-1`.
impl Div for Multivector {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inverse()
    }
}

/// Divides the multivector by a scalar.
impl Div<f32> for Multivector {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        let mut multivector = self.clone();
        multivector.coeff.iter_mut().for_each(|elem| *elem /= rhs);
        multivector
    }
}

/// Divides the multivector by a scalar (in-place).
impl DivAssign<f32> for Multivector {
    fn div_assign(&mut self, rhs: f32) {
        self.coeff.iter_mut().for_each(|elem| *elem /= rhs);
    }
}

/// Computes the full geometric product between two multivectors `A * B`.
/// This can be calculated by distributing the geometric product across
/// all of the individual basis blades. For example, we multiply the
/// e0 component of `A` with the scalar, e0, e1, e2, ..., e012 components
/// of `B`, and so on. We combine all of the intermediate results (each
/// of which will be, in general, a multivector) to create the full,
/// complete multivector `A * B`.
impl Mul for Multivector {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let a = self[0];
        let b = self[1];
        let c = self[2];
        let d = self[3];
        let e = self[4];
        let f = self[5];
        let g = self[6];
        let h = self[7];

        let i = rhs[0];
        let j = rhs[1];
        let k = rhs[2];
        let l = rhs[3];
        let m = rhs[4];
        let n = rhs[5];
        let o = rhs[6];
        let p = rhs[7];

        let mut multivector = Self::zeros();
        multivector[0] = a * i + c * k + d * l - g * o;
        multivector[1] = a * j + b * i - c * m + d * n - g * p - f * l + e * k - h * o;
        multivector[2] = a * k + c * i - d * o + g * l;
        multivector[3] = a * l + c * o - g * k + d * i;
        multivector[6] = a * o + c * l - d * k + g * i;
        multivector[5] = a * n - b * l + c * p + d * j + g * m + f * i - e * o + h * k;
        multivector[4] = a * m + b * k - c * j + d * p - g * n + f * o + e * i + h * l;
        multivector[7] = a * p + b * o + c * n + d * m + g * j + f * k + e * l + h * i;
        multivector
    }
}

/// Multiplies the multivector by a scalar.
impl Mul<f32> for Multivector {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut multivector = self.clone();
        multivector.coeff.iter_mut().for_each(|elem| *elem *= rhs);
        multivector
    }
}

/// Multiplies the multivector by a scalar (in-place).
impl MulAssign<f32> for Multivector {
    fn mul_assign(&mut self, rhs: f32) {
        self.coeff.iter_mut().for_each(|elem| *elem *= rhs);
    }
}

/// Negates all components of the multivector.
impl Neg for Multivector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut multivector = self.clone();
        multivector.coeff.iter_mut().for_each(|elem| *elem *= -1.0);
        multivector
    }
}

/// Computes the Poincare dual of this multivector. For example, points
/// and lines are dual to one another in 2D PGA.
impl Not for Multivector {
    type Output = Self;

    fn not(self) -> Self::Output {
        let mut multivector = Self::zeros();
        for i in 0..BASIS_COUNT {
            // Set:
            // Element 0 to element 7
            // Element 1 to element 6
            // ..etc.
            multivector[i] = self[BASIS_COUNT - i - 1];
        }
        multivector
    }
}

/// Subtracts two multivectors component-wise `A - B`.
impl Sub for Multivector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut multivector = Self::zeros();
        for i in 0..BASIS_COUNT {
            multivector[i] = self[i] - rhs[i];
        }
        multivector
    }
}

/// Subtracts a scalar from the multivector.
impl Sub<f32> for Multivector {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        let mut multivector = self.clone();
        multivector[0] -= rhs;
        multivector
    }
}

/// Credit: Ganja.js codegen engine features this implementation.
impl Display for Multivector {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let eps = 0.00001;
        let mut n = 0;
        let ret = self
            .coeff
            .iter()
            .enumerate()
            .filter_map(|(i, &coeff)| {
                if coeff > eps || coeff < -eps {
                    n = 1;
                    Some(format!(
                        "{}{}",
                        format!("{:.*}", 7, coeff)
                            .trim_end_matches('0')
                            .trim_end_matches('.'),
                        if i > 0 { BASIS_ELEMENTS[i] } else { "" }
                    ))
                } else {
                    None
                }
            })
            .collect::<Vec<String>>()
            .join(" + ");
        if n == 0 {
            write!(f, "0")
        } else {
            write!(f, "{}", ret)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructors() {
        let a = Multivector::with_coefficients(&[0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]);
        let b = Multivector::zeros();
        let c = Multivector::ones();
        let d = e0;
    }

    #[test]
    fn test_display() {
        let a = Multivector::with_coefficients(&[0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]);
        let b = Multivector::zeros();
        let c = e0;
        println!("{}", a);
        println!("{}", b);
        println!("{}", c);
    }

    #[test]
    fn test_basis_elements() {
        // Should be 0
        let result = e0 * e0;
        println!("e0 * e0 = {}", result);

        // Should be 1
        let result = e1 * e1;
        println!("e1 * e1 = {}", result);

        // Should be 1
        let result = e2 * e2;
        println!("e2 * e2 = {}", result);

        // Should be -1
        let result = e12 * e12;
        println!("e12 * e12 = {}", result);

        // Should be 0
        let result = e20 * e20;
        println!("e20 * e20 = {}", result);

        // Should be 0
        let result = e01 * e01;
        println!("e01 * e01 = {}", result);
    }

    #[test]
    fn test_inverse() {
        // First, try with a simple point (grade-2 element)
        let p = Multivector::point(1.0, 2.0);
        let p_inv = p.inverse();
        let result = p * p_inv;
        println!("p * p_inv = {}", result);

        // Then, try with a full multivector
        let a = Multivector::with_coefficients(&[0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]);
        let a_inv = a.inverse();
        let result = a * a_inv;
        println!("a * a_inv = {}", result);
    }

    #[test]
    fn test_geometric_product() {
        // Should be: 23 + 108e0 + -6e1 + -8e2 + -74e01 + -60e20 + -14e12 + -120e012
        let a = Multivector::with_coefficients(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
        let b = Multivector::with_coefficients(&[-1.0, -2.0, -3.0, -4.0, -5.0, -6.0, -7.0, -8.0]);
        let result = a * b;
        println!("a * b = {}", result);
    }

    #[test]
    fn test_inner_product() {
        // Should be: 23 + 108e0 + -6e1 + -8e2 + -74e01 + -60e20 + -14e12 + -16e012
        let a = Multivector::with_coefficients(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
        let b = Multivector::with_coefficients(&[-1.0, -2.0, -3.0, -4.0, -5.0, -6.0, -7.0, -8.0]);
        let result = a | b;
        println!("a | b = {}", result);
    }

    #[test]
    fn test_outer_product() {
        // Should be: -1 + -4e0 + -6e1 + -8e2 + -10e01 + -12e20 + -14e12 + -120e012
        let a = Multivector::with_coefficients(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
        let b = Multivector::with_coefficients(&[-1.0, -2.0, -3.0, -4.0, -5.0, -6.0, -7.0, -8.0]);
        let result = a ^ b;
        println!("a ^ b = {}", result);
    }

    #[test]
    fn test_join_and_meet() {
        // Should be the Euclidean point: <1, -2>
        let l1 = Multivector::line(1.0, 2.0, 3.0);
        let l2 = Multivector::line(4.0, 5.0, 6.0);
        let mut result = l1 ^ l2;
        result /= result.e12();
        let x = result.e20();
        let y = result.e01();
        println!(
            "l1 ^ l2 = {} or the point <{}, {}> where l1 and l2 meet",
            result, x, y
        );

        // Should be the line: x - y + 1 = 0
        let p1 = Multivector::point(1.0, 2.0);
        let p2 = Multivector::point(3.0, 4.0);
        let mut result = p1.join(&p2);
        //result /= result.e0();
        let a = result.e1();
        let b = result.e2();
        let c = result.e0();
        println!(
            "p1 & p2 = {} or the line {}x + {}y + {} = 0 that joins p1 and p2",
            result, a, b, c
        );
    }

    #[test]
    fn test_rotors_and_translators() {
        // Should be the Euclidean point: <3, 4>
        let p = Multivector::point(1.0, 2.0);
        let T = Multivector::translator(2.0, 2.0);
        let mut result = T * p * T.conjugation();
        result /= result.e12();
        let x = result.e20();
        let y = result.e01();
        println!(
            "T * p * ~T = {} or the translated point <{}, {}>",
            result, x, y
        );

        let p = Multivector::point(1.0, 2.0);
        let R = Multivector::rotor(45.0f32.to_radians(), 0.0, 0.0);
        let result = R * p * R.conjugation();
        println!("R * p * ~R = {}", result);
    }

    #[test]
    fn test_norm() {
        // Should be ~5 (arbitrary)
        let a = Multivector::with_coefficients(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
        let b = Multivector::with_coefficients(&[-1.0, -2.0, -3.0, -4.0, -5.0, -6.0, -7.0, -8.0]);
        println!("Norm of A: {}", a.norm());
        println!("Norm of B: {}", b.norm());

        // Should always be +/- 1
        println!("After normalization: {}", a.normalized().norm());
        println!("After normalization: {}", b.normalized().norm());
    }

    #[test]
    fn test_pga_cheatsheet_formulas() {
        // The line: x = 0
        let l = Multivector::line(1.0, 0.0, 0.0);
        let p = Multivector::point(-3.0, 2.0);

        // Should be the Euclidean point: <3, 2>
        let mut result = l * p * l;
        result = result / result.e12();
        let x = result.e20();
        let y = result.e01();
        println!(
            "l * p * l = {} or the point <{}, {}> reflected across l",
            result, x, y
        );
    }
}
