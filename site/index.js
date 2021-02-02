import { SVG } from '@svgdotjs/svg.js'
import '@svgdotjs/svg.draggable.js'

// import {
//     axiom_1
// } from '../pkg';


function radiansToDegrees(radians) {
    return radians * (180.0 / Math.PI);
}

function degreesToRadians(degrees) {
    return degrees * (Math.PI / 180.0);
}

import('../pkg/pga_axioms_wasm.js').then((wasm) => {

    // Setup the canvas
    const w = 500;
    const h = 500;
    const scx = w * 0.5;
    const scy = h * 0.5;
    const draw = SVG().addTo('body').size(w, h);
    const paperSize = Math.min(w, h) * 0.75;
    const pointRadius = 10.0;
    const lineWidth = 3.0;
    const lineExtrema = [-w * 2.0, 0, w * 2.0, 0];

    // Create the "paper" and grab a reference to its bounding box
    const paper = draw.rect(paperSize, paperSize)
        .attr('fill', '#ffeeee')
        .attr('stroke', '#000000')
        .center(w * 0.5, h * 0.5)
        .addClass('paper');
    const paperBounds = paper.bbox();

    // The WASM code needs to know about the dimensions of our paper, which is what
    // this object represents
    const paperStruct = new wasm.Paper(
        new wasm.Point(scx - paperSize * 0.5, scy - paperSize * 0.5), // Upper-left
        new wasm.Point(scx + paperSize * 0.5, scy - paperSize * 0.5), // Upper-right
        new wasm.Point(scx + paperSize * 0.5, scy + paperSize * 0.5), // Lower-right
        new wasm.Point(scx - paperSize * 0.5, scy + paperSize * 0.5)  // Lower-left
    );

    const axiomSpecifications = [
        {
            'description': 'Given two points <b>p0</b> and <b>p1</b>, there is a unique fold that passes through both of them.',
            'inputs': {
                'points': [
                    [scx, scy - paperSize * 0.25], 
                    [scx, scy + paperSize * 0.25]
                ],
                'lines': []
           
            },
            'function': wasm.axiom_1
        },
        {
            'description': 'Given two points <b>p0</b> and <b>p1</b>, there is a unique fold that places <b>p0</b> onto <b>p1</b>.',
            'inputs': {
                'points': [
                    [scx, scy - paperSize * 0.25], 
                    [scx, scy + paperSize * 0.25]
                ],
                'lines': []
           
            },
            'function': wasm.axiom_2
        },
        {
            'description': 'Given two lines <b>l0</b> and <b>l1</b>, there is a fold that places <b>l0</b> onto <b>l1</b>.',
            'inputs': {
                'points': [],
                'lines': [
                    [scx - paperSize * 0.25, scy - paperSize * 0.25, scx + paperSize * 0.25, scy - paperSize * 0.25],
                    [scx - paperSize * 0.25, scy + paperSize * 0.25, scx + paperSize * 0.25, scy + paperSize * 0.25]
                ]
           
            },
            'function': wasm.axiom_3
        },
        {
            'description': 'Given a point <b>p</b> and a line <b>l</b>, there is a unique fold perpendicular to <b>l</b> that passes through point <b>p</b>.',
            'inputs': {
                'points': [
                    [scx, scy - paperSize * 0.25]
                ],
                'lines': [
                    [scx - paperSize * 0.25, scy, scx + paperSize * 0.25, scy]
                ]
           
            },
            'function': wasm.axiom_4
        },
        {
            'description': '',
            'inputs': {
                'points': [
                    [scx - paperSize * 0.25, scy - paperSize * 0.125],
                    [scx + paperSize * 0.25, scy - paperSize * 0.125]
                ],
                'lines': [
                    [scx - paperSize * 0.125, scy + paperSize * 0.125, scx + paperSize * 0.125, scy + paperSize * 0.125]
                ]
           
            },
            'function': wasm.axiom_5
        },

    ];

    let currentAxiom = axiomSpecifications[0];









    let crease = drawLineFromCoeffs(1.0, 2.0, 0.0)
        .attr('stroke-width', lineWidth)
        .attr('stroke', '#000000')
        .attr('stroke-dasharray', "2");

    let positive = draw.polygon([])
        .attr({
            'fill': '#c4903d',
            'fill-opacity': 0.75,
            'stroke': '#000000',
            'stroke-width': lineWidth * 0.5
        });

    let negative = draw.polygon([])
        .attr(positive.attr());

    positive.insertAfter(paper);
    negative.insertAfter(positive);
    crease.insertAfter(negative)




    

    /// Given the coefficients of a 1-vector in 2D PGA, draw a line.
    function drawLineFromCoeffs(a, b, c) {
        const yIntercept = -c; 
        const rotation = -radiansToDegrees(Math.atan2(a, b));

        // Set x-coords to somewhere far off the page ("infinity")
        const line = draw.line(-w * 2.0, yIntercept, w * 2.0, yIntercept)
            .attr({
                'stroke-width': lineWidth,
                'stroke': '#ff0000',
                'stroke-dasharray': '4'
            })
            .rotate(rotation, 0.0, 0.0);

        return line;
    }
















    // Keeps the element within the bounds of the paper.
    function checkPaperBoundaries(e) {
        const { handler, box } = e.detail
        e.preventDefault()

        // Keep the point(s) inside the paper bounds
        let { x, y } = box;

        if (x < paperBounds.x) {
            x = paperBounds.x
        }
        if (y < paperBounds.y) {
            y = paperBounds.y
        }
        if (box.x2 > paperBounds.x2) {
            x = paperBounds.x2 - box.w
        }
        if (box.y2 > paperBounds.y2) {
            y = paperBounds.y2 - box.h
        }
        handler.move(x, y);
    }

    // A function that calls into WASM code to calculate a new crease based
    // on the current state of the paper.
    function callCurrentAxiom() {
        // Retrieve all of the relevant points and convert them into WASM objects
        const pointCoords = draw.children()
            .filter(elem => elem.hasClass('point'))
            .map(elem => new wasm.Point(elem.cx(), elem.cy()));

        const segmentEndpointCoords = draw.children()
            .filter(elem => elem.hasClass('segment'))
            .map(element => [ 
                new wasm.Point(...element.array().slice(0, 2)),
                new wasm.Point(...element.array().slice(2, 4))
            ]);

        const coords = pointCoords.concat(segmentEndpointCoords);

        // Run current axiom - points then lines (in that order)
        const results = currentAxiom.function(
            paperStruct,
            ...coords
        );

        // WASM code will return `null` if no valid creases are found
        if (results != null && results.line.a != null && results.line.b != null && results.line.c != null) {
            // Rebuild the crease and update the cut polygons
            crease.remove();
            crease = drawLineFromCoeffs(results.line.a, results.line.b, results.line.c);
            crease.insertAfter(negative);
            positive.plot(results.positive.map(pt => [pt.x, pt.y]));
            negative.plot(results.negative.map(pt => [pt.x, pt.y]));
        } else {
            // Otherwise, a valid crease wasn't found, so hide the crease and cut polygons
            crease.remove();
            positive.plot([]);
            negative.plot([]);
        }
    }

    const pointDragCallback = function(e) {
        checkPaperBoundaries(e);
        callCurrentAxiom();
    };

    // Iterates through all of the SVG elements that represent interactive
    // line segments and updates their internal attributes to match their
    // source and destination endpoints.
    function updateSegments() {
        draw.children()
            .filter(elem => elem.hasClass('segment'))
            .forEach(elem => {
                // See: https://stackoverflow.com/questions/22636291/svg-line-in-y-mxc-format
                const src = elem.remember('src');
                const dst = elem.remember('dst');
                const theta = radiansToDegrees(Math.atan2(dst.cy() - src.cy(), dst.cx() - src.cx()));
                elem.plot(...lineExtrema);
                elem.attr('transform', `translate(${src.cx()}, ${src.cy()}) rotate(${theta})`)

                // TODO: this should work but doesn't:
                // elem
                // .transform({
                //     x: src.cx(),
                //     y: src.cy()
                // .transform({
                //     rotation: angleInDegrees 
                // });

                // Or:
                //elem.plot(src.cx(), src.cy(), dst.cx(), dst.cy());
            });
    }

    const segmentCallback = function(e) {
        updateSegments();
        pointDragCallback(e);
    }

    function clear() {
        // Delete all existing points and lines
        draw.children()
            .filter(elem => elem.hasClass('segment') || elem.hasClass('point'))
            .forEach(elem => elem.remove());

        // Remove the existing crease and cut polygons
        crease.remove();
        positive.plot([]);
        negative.plot([]);
    }

    function initCurrentAxiom() {
        clear();

        // Initialize interactive points
        currentAxiom.inputs.points.forEach(coords => {
            let circle = draw.circle(pointRadius)
                .center(coords[0], coords[1])
                .addClass('point')
                .draggable();

            circle.on('dragmove.namespace', pointDragCallback);
        });

        // Initialize interactive lines
        currentAxiom.inputs.lines.forEach(coords => {
            // Create the interactive point that represents the source endpoint of this line segment
            let src = draw.circle(pointRadius)
                .center(...coords.slice(0, 2))
                .addClass('point')
                .draggable();

            // Create the interactive point that represents the destination endpoint of this line segment
            let dst = draw.circle(pointRadius)
                .center(...coords.slice(2, 4))
                .addClass('point')
                .draggable();

            // Create the line segment itself (which is not interactive) - store references to 
            // its source and destination endpoints so that they can be retrieved later on
            const theta = radiansToDegrees(Math.atan2(dst.cy() - src.cy(), dst.cx() - src.cx()));
              
            let line = draw.line(...lineExtrema)
                .attr('transform', `translate(${src.cx()}, ${src.cy()}) rotate(${theta})`)
                .attr({
                    'stroke-width': lineWidth,
                    'stroke': '#312d33',
                })
                .addClass('segment')
                .remember('src', src)
                .remember('dst', dst);

            src.insertAfter(line);
            dst.insertAfter(line);
            src.on('dragmove.namespace', segmentCallback);
            dst.on('dragmove.namespace', segmentCallback);
        });
    }

    function switchAxiom(index) {
        // Initialize interactive objects
        currentAxiom = axiomSpecifications[index];
        initCurrentAxiom();

        // Set some descriptive info text
        const descriptionP = document.getElementById('description');
        descriptionP.innerHTML = currentAxiom.description;

        // Update crease and cut polygons 
        callCurrentAxiom();
    }

    // Add a key callback for changing between the different axioms.
    document.addEventListener('keydown', (event) => {
        // Make sure that the key is 1-7 (inclusive)
        const key = event.key;
        const isValidAxiom = /^[1-7]$/i.test(key)

        // Axioms are numbered 1-7, but array indexing is 0-based, so subtract 1
        if (isValidAxiom) {
            switchAxiom(key - 1);
        }
    });

    // Kick off the application
    switchAxiom(0);
});