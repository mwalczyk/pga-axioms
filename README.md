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
intersections, projections, rotations, reflections, etc. is *drastically* simplified (of course, after the up-front 
work of learning a new mathematical framework).

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
represent the *same* object. For example, any point `λ * (x, y, w)` (for `w != 0`) represents the same Euclidean 
point `(x, y)`. We recover the inhomogeneous coordinates of the point by dividing by `w`. When `w` is zero, the 
point is called an **ideal point** (or a point at infinity). This makes some mathematical sense, as dividing by zero 
produces infinity, suggesting that this point is "infinitely far away." Intuitively, ideal points are analogous to 
standard direction vectors in linear algebra. There also exists an **ideal line**, representing the line at infinity,
along which all ideal points lie.

The purpose of including these ideal elements is to avoid "special cases." For example, including the ideal points 
allows us to say (without any extra conditions) that two lines intersect at a point. In a non-projective setting, 
care must be taken to handle the case of parallel lines. However, in projective space, parallel lines actually 
intersect at an ideal point.

For a more detailed treatment of "points at infinity", see the following [blog post](https://pointatinfinityblog.wordpress.com/2016/04/11/points-at-infinity-i-projective-geometry/).

#### Multivectors

The basis elements of 2D PGA are: `1, e0, e1, e2, e01, e20, e12, e012`. The particular choice of basis isn't 
important (for example, we could use `e02` instead of `e20`), but we choose this basis so that its dual doesn't 
introduce any sign changes (more on duality later) and because it is the same basis used by the Ganja.js code 
generator. The basis for 2D PGA is organized as follows:

- `1` is the **scalar** element
- `e0`, `e1`, and `e2` are the basis **vectors** 
- `e01`, `e20`, and `e12` are the basis **bivectors**
- `e012` is the basis **trivector** or **pseudoscalar**

A general element of 2D PGA is called a **multivector** and can be written as the sum of each of the basis elements 
(multiplied by some scalar coefficient): `A + B*e0 + C*e1 + D*e2 + E*e01 + F*e20 + G*e12 + H*e012`. This is 
analogous to how we might write a "traditional" vector with components `(x, y, z)` as the sum `x*e1 + y*e2 + z*e3` 
(where again, `e1`, `e2`, and `e3` are our 3 basis vectors - the x, y, and z axes). Multivectors are the building 
blocks of computation in this program.

#### Grade Selection

We can "select out" just the vector part of a multivector, `B*e0 + C*e1 + D*e2`, or perhaps just the bivector part 
`E*e01 + F*e20 + G*e12`. This is an operation known as **grade selection** and is often denoted `<A>ₙ` for some 
multivector `A` and grade `n`. The previous examples correspond to `<A>₁` and `<A>₂`, respectively.

#### Geometric Primitives

In PGA, geometric primitives are represented by vectors of different **grades**. In particular, lines are 1-vectors 
and points are 2-vectors (in 3D PGA, we would also have planes). For example, a line with equation `Ax + By + C = 0` 
is represented by the 1-vector `Ae1 + Be2 + Ce0`. A Euclidean point with coordinates `(A, B)` is represented by the 
2-vector `Be01 + Ae20 + e12`.

The reason *why* lines are represented this way stems from the following observations: let's say we have two lines 
with equations `Ax + By + C = 0` and `Dx + Ey + F = 0`. Assume that `A*A + B*B = 1` and `D*D + E*E = 1`. To compute 
the angle between the pair of lines, it is easy to show that `A*D + B*E = cosα`. It is clear from this example that 
the result *does not depend on the third coordinate of each line* (`C` and `F` in the example above). This makes 
sense: if you plot the pair of lines, changing the last coordinate has the effect of translating the line, which 
clearly does not change the result of our angle calculation. Thus, we assign `e0` to this third coordinate since it 
squares to zero.

#### Products

In order to perform computation with multivectors, we first need to define the ways in which we can operate on them. 
For this, geometric algebra defines various types of products. Note that the list below is not exhaustive: rather, 
it contains the products that were necessary for this project.

##### Geometric Product

The geometric product is derived from the simple set of rules outlined in the introduction. Namely, `e0` squares to 
0 (known as a **degenerate metric**), while `e1` and `e2` square to 1. We can multiply the basis vectors together to 
produce the basis bivectors: for example, the product `e0 * e1 = e01` cannot be further simplified. Note that we can 
"shuffle" the result at the cost of a sign change per move. Using the previous example, `e10 = -e01`. Using these 
rules, we can simplify more complicated products: `e01 * e12 = e0112 = e02 = -e20`, where in the third step, we have 
used the fact that `e1` squares to 1 to simplify the expression. 

To compute the geometric product between two multivectors, we simply distribute the geometric product across each of 
the basis elements and simplify as necessary using the "rules" of our algebra. The geometric product is closed in the 
sense that the geometric product between any two multivectors `A` and `B` will also be an element of 2D PGA. 

The geometric product between two k-vectors will be, in general, some mixed-grade object. For example, the geometric 
product between a point (grade-2) and a line (grade-1) is a multivector with a vector (grade-1) part and a trivector 
(grade-3) part.

The geometric product is the "main" product of geometric algebra, in the sense that the other products below are 
defined with respect to the geometric product.

##### Inner Product

The (symmetric) inner product of a k-vector and an s-vector is the grade-`|k - s|` part of their geometric product. 
For example, the inner product between a point (grade-2) and a line (grade-1) is another line (grade-1). There are 
other types of inner products (the left and right contractions, for example), but they are not used in this codebase. 
To calculate the inner product between two multivectors, we simply distribute the inner product across each of the 
k-vector / s-vector components of the operands. For example, in 2D PGA a multivector has a grade-0 part, a grade-1 part, 
a grade-2 part, and a grade-3 part. We compute the inner product between the first multivector's grade-0 part and each 
grade of the second multivector. We repeat this process for all parts of the first multivector. In the end, we are left 
with a handful of intermediate results, each of which will be, in general, some sort of multivector. We add these 
together and simplify to obtain the final inner product.

##### Outer Product

The outer (wedge, exterior) product of a k-vector and an s-vector is the grade-`|k + s|` part of their geometric 
product (or zero if it does not exist). For example, the outer product between two lines (grade-1) is a point 
(grade-2). The outer product between two points (grade-2) is zero (since grade `|2 + 2| = 4` elements do not exist 
in this 3-dimensional algebra). To calculate the outer product between two multivectors, we follow the same 
procedure outlined above for the inner product, except we use the outer product instead.

Aside: in many introductory texts, the geometric product is written as the sum of the inner and outer products. For 
example, for two vectors `u` and `v`, you might see: `uv = u•v + u^v`. However, for general k-vectors, the geometric 
product may contain other terms, so the formula does not necessarily apply.

#### Meet and Join

The "meet" between two elements is defined as their outer product. This operator is primarily used to compute 
incidence relations (i.e. the point of intersection between two lines).

In this codebase, the "join" operator (or regressive product) of two multivectors is given by the dual of the outer 
product of their duals. For example, "joining" two points `p1 & p2` involves the following operations:

- Compute the dual of each point, which results in two lines (or projective planes)
- Intersect these lines via the "meet" operator
- Compute the dual of the point of intersection, which results in a line 

Intuitively, "joining" two points results in the line that connects them. There is a bit of nuance here with regard
to orientation. To join `A` and `B`, we actually compute `!(!B ^ !A)`, i.e. the order of `A` and `B` appears to be 
swapped. This is the "fully oriented" approach presented in PGA4CS by Dorst (see links below). In 3D, there is 
actually both an "undual" and dual operator, and so the formula becomes `undual(dual(B) ^ dual(A))`, but in 2D, this 
isn't necessary: Poincaré duality works just fine (the "undual" and dual operators are equivalent). This is still a 
bit confusing to me and probably requires closer investigation, but for now, the math works. 

#### Duality

This codebase implements Poincaré duality. The dual of a basis element is simply whatever must be multiplied on the 
right in order to recover the unit pseudoscalar `e012`. For example, the dual of `e0` is `e12`, since `e0 * e12 = 
e012`. The dual of `e01` is `e2`, since `e01 * e2 = e012`. The dual of `e012` is 1, and the dual of 1 is `e012`. In 
2D PGA, lines and points are dual to one another.

#### Involutions

Any operation that multiplies each grade of a multivector by +/-1 is called a **conjugation**. Conjugations 
are [involutions](https://en.wikipedia.org/wiki/Involution_(mathematics)), since applying the operation twice will 
result in the original multivector. There are three classical operations of conjugation that are implemented in this 
codebase:

1. Reversion
2. Grade involution (or the main involution)
3. Clifford conjugation

These are explained in detail in the [following paper](https://arxiv.org/pdf/2005.04015.pdf), but basically, they 
are used for computing things like the inverse of a multivector under the geometric product and the sandwich product,
which is needed when working with rotors and translators.

#### Inverse
The inverse of a multivector `A` is the multivector `A^-1` such that `A * A^-1 = 1`. General multivector inverses 
are difficult to calculate, but for dimensions <=5, there are relatively simple formulas for doing so. The [following 
paper](http://repository.essex.ac.uk/17282/1/TechReport_CES-534.pdf) explains this process, but basically, inverses 
are computed by repeatedly applying involutions to the multivector.

## Tested On
- Windows 10
- Rust compiler version `1.51.0`
- Chrome browser

## To Build
1. Clone this repo.
2. Make sure 🦀 [Rust](https://www.rust-lang.org/en-US/) is installed and `cargo` is in your `PATH`.
3. Make sure [wasm-pack](https://rustwasm.github.io/wasm-pack/) is installed.
4. Inside the repo, run: `wasm-pack build`.
5. `cd` into the `site` subdirectory and run `npm install`.
6. Run `npm run serve` and go to `localhost:8080` to view the site.

## To Use
Once the site is loaded, press 1-7 to switch between the 7 axioms. Points and lines can be dragged around the canvas,
and the fold should update in real-time.

## Future Directions
Currently, the software does **not** check whether the calculated crease *actually* lies within the bounds of the 
paper. Similarly, it doesn't check whether any of the "output geometry" lies within the bounds of the paper. For 
example, axiom #7 states: "given one point `p` and two lines `l0` and `l1`, there is a fold that places `p` onto 
`l0` and is perpendicular to `l1`." The software (in its current form) only checks that the reflected point `p'` 
lies *somewhere* along the line `l0` (even if it is "off the page"). In a sense, we assume that the sheet of paper
is infinitely large. This is the first issue I would like to address, as I believe it would be relatively simple to 
implement.

Working with full multivectors is convenient and expressive, but at the user-level, it can be a bit cumbersome and 
confusing. Originally, I set out to replicate Klein's API (see the links below), where we instead represent points 
and lines at the struct level, rather than full multivectors. This introduces some additional type-safety, at the 
cost of flexibility and code unification: for example, the formula for reflecting a point across a line is the same 
as the formula for reflecting a line across a point (with the arguments swapped). Thus, by using full multivectors, 
we only have to write one function `reflect` that works in both cases. If, however, we use `Point` and `Line` 
structs, we would have to write two different functions that basically do the same thing: `reflect_point_line` and 
`reflect_line_point`. There is probably some middle ground here, which requires further investigation.

## To Do
- [ ] Finish axiom #6
- [ ] Combine and/or refactor functions in the `geometry` module, as necessary
- [ ] Explore a more "type-safe" approach to PGA

## Credits
I am very much at the beginning stages of my PGA journey, but I would not have been able to learn this topic without 
the generous guidance of various members of the [Bivector](https://bivector.net/) community. 

In particular, I would like to thank @enki, @mewertX0rz, @bellinterlab, and @ninepoints for answering so many of my 
questions on Discord. If you are at all interested in geometric algebra, I encourage you to check out the Bivector 
Discord channel.

Other notes and papers I found useful throughout the creation of this process include:

1. [Charles Gunn's SIGGRAPH notes on PGA](https://arxiv.org/pdf/2002.04509.pdf): along with PGA4CS, one of the most 
   extensive PGA resources available right now. This paper also contains the "PGA cheatsheet": a super helpful list 
   of formulas for operating on Euclidean geometry with PGA. All of Gunn's publications were very helpful to me while 
   learning about PGA.    
2. [PGA4CS](https://bivector.net/PGA4CS.html): an extension to Dorst's original text, "Geometric Algebra for 
   Computer Science" that is entirely focused on PGA.
3. [Ganja.js](https://github.com/enkimute/ganja.js): my initial multivector implementation was based on the Ganja.js 
   code generator. Additionally, I was able to verify most of my implementation against Ganja.js, which was super 
   useful. I recommend Ganja.js (and Coffeeshop) to anyone who is interested in getting started with GA.
4. [Klein](https://github.com/jeremyong/klein): an amazing, type-safe C++ library for 3D PGA. 

### License
[Creative Commons Attribution 4.0 International License](https://creativecommons.org/licenses/by/4.0/)