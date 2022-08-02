$fn = 60;

// TASK: What is a good value for the minimum spindle height? I think to answer
//       this question, I think I need to answer the following ones first:
//       1. Do I want to mill to be able to machine its own table?
//       2. If the answer is no:
//          a) What is the thinnest piece of material that I might conceivably
//             machine?
//          a) what is the shortest tool I might conceivably use?
spindle(min_height = 10);
table(size_x = 300, size_y = 150, thickness = 10);

// The CNC spindle motor
//
// https://www.zhonghuajiangspindle.com/1.5kw-cnc-air-cooled-spindle-motor-80mm.html
module spindle(min_height) {
    translate([0, 0, min_height])
    union() {
        diameter = 80;

        elements = [
            [      13,       19], // collet
            [261 - 13, diameter], // placeholder for rest of spindle
        ];

        element(i = 0, elements = elements);
    }

    module element(i, elements) {
        if (i < len(elements)) {
            height   = elements[i][0];
            diameter = elements[i][1];

            cylinder(d = diameter, h = height);

            translate([0, 0, height])
            element(i = i + 1, elements = elements);
        }
    }
}

module table(size_x, size_y, thickness) {
    translate([0, 0, -thickness / 2])
    cube([size_x, size_y, thickness], center = true);
}
