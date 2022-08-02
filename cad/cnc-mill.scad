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

module spindle(min_height) {
    translate([0, 0, min_height])
    union() {
        diameter = 80;

        z_collet_bottom      =   0;
        z_collet_top         =  13;
        z_top_plus_connector = 261;

        cylinder(d = 19, h = z_collet_top - z_collet_bottom);
        translate([0, 0, z_collet_top])
        cylinder(d = diameter, h = z_top_plus_connector - z_collet_top);
    }
}

module table(size_x, size_y, thickness) {
    translate([0, 0, -thickness / 2])
    cube([size_x, size_y, thickness], center = true);
}
