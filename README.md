# PGA Axioms
🗺️ A program for exploring the Huzita-Hatori axioms for origami, using projective geometric algebra (PGA).

## Description

### Huzita-Hatori Axioms
The Huzita-Hatori axioms are a set of 7 rules that describes ways in which one can fold a piece of paper. Every fold 
can be described by one of the 7 axioms. The axioms themselves are described in detail in the following [Wikipedia 
article](https://en.wikipedia.org/wiki/Huzita%E2%80%93Hatori_axioms#Axiom_7). As an example, axiom #1 states: "given 
two points `p0` and `p1`, there is a unique fold that passes through both of them." In this case, the desired crease 
is simple the line that passes through both points. 

This software attempts to turn each axiom into an "interactive sketch," where points and lines can be freely moved 
around the canvas ("paper").

### Projective Geometric Algebra

#### Introduction
The main purpose of this project was to explore an emerging field of mathematics known as projective geometric 
algebra or PGA. At a high-level, PGA is a different / fresh way of dealing with geometric problems that doesn't 
involve "standard" linear algebra. In this algebra, geometric objects like points, lines, and planes are treated as 
elements of the algebra. Loosely speaking, this means we can "operate on" these objects. Finding the intersection 
between two lines, for example, simply amounts to taking the wedge (or outer, exterior) product between the two line 
elements. The wedge product is one of many products available in geometric algebra, and others will be discussed 
later on in this document.

As alluded to in the previous paragraph, the primary benefit of using PGA is that computing things like
intersections, projections, rotations, etc. is *drastically* simplified (of course, after the up-front work of
learning PGA).

Mathematically, PGA is a graded algebra with signature `R*(2,0,1)`. Here, the numbers `(2,0,1)` denote the number of 
positive, negative, and zero dimensions in the algebra. Most readers are probably familiar with `R(3,0,0)`, which 
has 3 basis vectors (which we might call `e1`, `e2`, `e3`), each of which squares to 1. In PGA, we have 2 basis 
vectors that square to 1, which we will call `e1` and `e2`, and 1 basis vector that squares to 0, which we will call 
`e0`. Another familiar example might be the complex numbers, which have signature `R(0,1,0)`, with one basis element 
that squares to -1. We usually see this element written as `i`. From this simple set of rules, we build the entire algebra. 
The `*` in the signature means that we are working in the *dual* algebra (more on this later). 

The *projective* part of PGA comes from the fact that we are working in a "one-up" space: 2-dimensional PGA has 3 
(total) dimensions, used to represent 2-dimensional objects (points and lines). This is identical to the use of homogeneous 
coordinates in computer graphics, where a point in 3-space is actually represented by a 4-element vector `(x, y, z, w)` 
(where `w` is often implicitly set to 0 or 1). In projective space, objects that only differ by a scalar multiple 
represent the *same* object. For example, any point `n * (x, y, w)` (for `w != 0`) represents the same Euclidean 
point `(x, y)`. We recover the inhomogeneous coordinates of the point by dividing by `w`. When `w` is zero, the 
point is called an **ideal point** (or a point at infinity). This makes some mathematical sense, as dividing by zero 
produces infinity, suggesting that this point is "infinitely far away." Intuitively, ideal points are analogous to 
standard direction vectors in linear algebra. There also exists an **ideal line**, representing the line at infinity,
along which all ideal points lie.

The purpose of including these ideal elements is to avoid "special cases." For example, including the ideal points 
allows us to say (without any extra conditions) that two lines intersect at a point. In a non-projective setting, 
care must be taken to handle the case of parallel lines. However, in projective space, parallel lines actually 
intersect at an ideal point.

#### Products
##### Geometric Product
To compute the geometric product between two multivectors, we simply distribute the geometric product across each of 
the basis elements and simplify as necessary using the "rules" of our algebra (`e0` squares to 0, etc.). The 
geometric product is closed in the sense that the geometric product between any two multivectors `A` and `B` will 
also be an element of 2D PGA. However, in general, the geometric product between two elements of the algebra will be 
some mixed-grade object. For example, the geometric product between a point (grade-2) and a line (grade-1) is a 
multivector with a vector (grade-1) part and a trivector (grade-3) part.

##### Inner Product
The (symmetric) inner product of a k-vector and an s-vector is the grade-`|k - s|` part of their geometric product 
(or zero if it does not exist). For example, the inner product between a point (grade-2) and a line (grade-1) is 
another line (grade-1). There are other types of inner products (the left and right contractions, for example), but 
they are not used in this codebase. To calculate the inner product between two multivectors, we simply distribute 
the inner product across each of the k-vector / s-vector components of the operands. For example, in 2D PGA a 
multivector has a grade-0 part, a grade-1 part, a grade-2 part, and a grade-3 part. We compute the inner product 
between the first multivector's grade-0 part and each grade of the second multivector. We repeat this process for 
all parts of the first multivector. In the end, we are left which a handful of intermediate products, each of which 
will be (in general) some sort of multivector. We add these together and simplify to obtain the final inner product.

##### Outer Product
The outer (wedge, exterior) product of a k-vector and an s-vector is the grade-`|k + s|` part of their geometric 
product (or zero if it does not exist). For example, the outer product between two lines (grade-1) is a point 
(grade-2). The outer product between two points (grade-2) is zero (since grade `|2 + 2| = 4` elements do not exist 
in this 3-dimensional algebra). To calculate the outer product between two multivectors, we follow the same 
procedure outlined above for the inner product, except we use the outer product instead.

#### Meet and Join
The "meet" between two elements is defined as their outer product. This operator is primarily used to compute 
incidence relations (i.e. the point of intersection between two lines).

In this codebase, the "join" operator (or regressive product) of two multivectors is given by the dual of the outer 
product of their duals. For example, "joining" two points `p1 & p2` involves the following operations:

- Compute the dual of each point, which results in two lines (or projective planes)
- Intersect these lines via the "meet" operator
- Compute the dual of the point of intersection, which results in a line 

Intuitively, "joining" two points results in the line that connects them.

#### Duality
This codebase implements Poincaré duality. The dual of a basis element is simply whatever must be multiplied on the 
right in order to recover the unit pseudoscalar `e012`. For example, the dual of `e0` is `e12`, since `e0 * e12 = e012`.

#### Involutions
TODO

## Tested On
- Windows 10
- NVIDIA GeForce GTX 1660 Ti
- Rust compiler version `1.51.0`
- Chrome browser

## To Build
1. Clone this repo.
2. Make sure 🦀 [Rust](https://www.rust-lang.org/en-US/) installed and `cargo` is in your `PATH`.
3. Inside the repo, run: `cargo build --release`.

## To Use
TODO

## Future Directions
Currently, the software does **not** check whether the calculated crease *actually* lies within the bounds of the 
paper. Similarly, it doesn't check whether any of the "output geometry" lies within the bounds of the paper. For 
example, axiom #7 states: "given one point `p` and two lines `l0` and `l1`, there is a fold that places `p` onto 
`l0` and is perpendicular to `l1`." The software (in its current form) only checks that the reflected point `p'` 
lies *somewhere* along the line `l0` (even if it is "off the page"). In a sense, we assume that the sheet of paper
is infinitely large. This is the first issue I would like to address, as I believe it would be relatively simple to 
implement.

## To Do
- [ ] Finish all 7 axioms

## Credits
I am very much at the beginning stages of my PGA journey, but I would not have been able to learn this topic without 
the generous guidance of various members of the [Bivector](https://bivector.net/) community. In particular, I would 
like to thank @enki, @mewertX0rz, @bellinterlab, and @ninepoints for answering so many of my questions on Discord. 
If you are at all interested in geometric algebra, I encourage you to check out the Bivector Discord channel.

### License
[Creative Commons Attribution 4.0 International License](https://creativecommons.org/licenses/by/4.0/)