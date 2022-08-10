// The CNC spindle motor
//
// https://www.zhonghuajiangspindle.com/1.5kw-cnc-air-cooled-spindle-motor-80mm.html
module spindle() {
    union() {
        // These are dimensions that are specified in the drawing.
        diameter                     =  80;
        diameter_collet              =  19;
        diameter_neck_base           =  29.5;
        diameter_shoulder            =  54;
        height_collet                =  13;
        height_collet_neck_neck_base =  36;
        height_neck_base             =   3;
        height_shoulder              =  18;
        height_body_bottom           =   8;
        height_body_main             = 164;
        height_body_top              =  26;
        height_total                 = 261;

        // These dimensions are derived from the previous ones.
        height_neck = height_collet_neck_neck_base
            - height_collet
            - height_neck_base;
        height_connector = height_total
            - height_body_top - height_body_main - height_body_bottom
            - height_shoulder
            - height_collet_neck_neck_base;

        // These ones are just guesses, as the drawing doesn't say.
        diameter_neck      = 15;
        diameter_connector = 20;

        // Colors
        black  = [0.0, 0.0, 0.0, 1.0];
        silver = [0.8, 0.8, 0.8, 1.0];

        elements = [
            [     height_collet,    diameter_collet,  black], // collet
            [       height_neck,      diameter_neck, silver], // neck
            [  height_neck_base, diameter_neck_base, silver], // neck base
            [   height_shoulder,  diameter_shoulder,  black], // shoulder
            [height_body_bottom,           diameter,  black], // body: bottom
            [  height_body_main,           diameter, silver], // body: main
            [   height_body_top,           diameter,  black], // body: top
            [  height_connector, diameter_connector, silver], // connector
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
