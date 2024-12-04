mod objects;
mod camera;
mod images;

use objects::sphere::Sphere;
use objects::line::Line;

use crate::camera::generateImg;

use std::io::Cursor;
use image::ImageReader;

fn main() {

    let p0 = [0.0, 0.0, 0.0]; // Point de départ de la droite
    let mut d = [1.0, 0.0, 0.6];  // Direction de la droite
    let c = [2.0, 0.0, 0.0];  // Centre de la sphère
    let r = 1.0;              // Rayon de la sphère

    let weird_depth = 100.0;
    let cam_width = 200;
    let cam_height = 200;

    let mut imgbuf = image::ImageBuffer::new(cam_width, cam_height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {

        d[2] = (((x as i32)-(cam_width/2) as i32) as f64)/weird_depth;
        d[1] = (((y as i32)-(cam_height/2) as i32) as f64)/weird_depth;

        let white : u8 = 254;

        if intersection_droite_sphere(p0, d, c, r) {
            *pixel = image::Rgb([white,white,white]);
        } else {
            *pixel = image::Rgb([0,0,0]);
        }
    }
    imgbuf.save("test.png").unwrap();
}

fn intersection_droite_sphere(
    p0: [f64; 3], // Point de départ de la droite
    d: [f64; 3],  // Direction de la droite
    c: [f64; 3],  // Centre de la sphère
    r: f64        // Rayon de la sphère
) -> bool {
    let l = [
        p0[0] - c[0],
        p0[1] - c[1],
        p0[2] - c[2],
    ];

    let a = d[0] * d[0] + d[1] * d[1] + d[2] * d[2]; // d · d
    let b = 2.0 * (d[0] * l[0] + d[1] * l[1] + d[2] * l[2]); // 2 * (d · L)
    let c = (l[0] * l[0] + l[1] * l[1] + l[2] * l[2]) - r * r; // L · L - R^2

    let discriminant = b * b - 4.0 * a * c;

    discriminant >= 0.0
}