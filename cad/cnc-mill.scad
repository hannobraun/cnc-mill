$fn = 60;

spindle();
table(size_x = 300, size_y = 150, thickness = 10);

module spindle() {
    cylinder(d = 80, h = 261);
}

module table(size_x, size_y, thickness) {
    translate([0, 0, -thickness / 2])
    cube([size_x, size_y, thickness], center = true);
}
