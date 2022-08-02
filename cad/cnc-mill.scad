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
        // These are dimensions that are specified in the drawing.
        diameter                      =  80;
        diameter_collet               =  19;
        diameter_neck_base            =  29.5;
        diameter_shoulder             =  54;
        height_collet                 =  13;
        height_collect_neck_neck_base =  36;
        height_neck_base              =   3;
        height_shoulder               =  18;
        height_total                  = 261;

        // These dimensions are derived from the previous ones.
        height_neck = height_collect_neck_neck_base
            - height_collet
            - height_neck_base;

        // This one's just a guess, as the drawing doesn't say.
        diameter_neck = 15;

        // Colors
        black  = [0.0, 0.0, 0.0, 1.0];
        silver = [0.8, 0.8, 0.8, 1.0];

        elements = [
            [   height_collet,    diameter_collet,  black], // collet
            [     height_neck,      diameter_neck, silver], // neck
            [height_neck_base, diameter_neck_base, silver], // neck base
            [ height_shoulder,  diameter_shoulder,  black], // shoulder

            // placeholder for rest of spindle
            [height_total - height_collet - height_neck - height_neck_base - height_shoulder, diameter, silver],
        ];

        element(i = 0, elements = elements);
    }

    module element(i, elements) {
        if (i < len(elements)) {
            height   = elements[i][0];
            diameter = elements[i][1];
            color    = elements[i][2];

            color(color)
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
