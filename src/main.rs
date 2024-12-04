mod objects;
mod camera;
mod intersections;

use objects::sphere::Sphere;
use objects::line::Line;
use objects::v3::V3;

//use crate::camera::generateImg;

use std::io::Cursor;
use image::ImageReader;

fn main() {

    let sphere1 = Sphere {
        center: V3 {x: 2.0, y: 0.0, z:0.0},
        radius: 1.0
    };

    let sphere2 = Sphere {
        center: V3 {x: 2.0, y: 0.0, z:2.0},
        radius: 0.5
    };

    let mut droite = Line {
        pos: V3 {x: 0.0, y: 0.0, z:0.0},
        dir: V3 {x: 1.0, y: 0.0, z:0.6}
    };

    let geometries = vec![&sphere1, &sphere2];


    let weird_depth = 100.0;
    let cam_width = 200;
    let cam_height = 200;

    let mut imgbuf = image::ImageBuffer::new(cam_width, cam_height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {

        droite.dir.z = (((x as i32)-(cam_width/2) as i32) as f64)/weird_depth;
        droite.dir.y = (((y as i32)-(cam_height/2) as i32) as f64)/weird_depth;

        let white : u8 = 254;
        *pixel = image::Rgb([0,0,0]);

        for geometry in &geometries {
            if intersections::line_and_sphere(&droite, &geometry) {
                *pixel = image::Rgb([white,white,white]);
            }
        }
    }
    imgbuf.save("test.png").unwrap();
}

