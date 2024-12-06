mod objects;
mod camera;
mod intersections;
mod graphics;

use image::io;
use objects::sphere::Sphere;
use objects::v3::V3;
use std::io::stdin;
use crate::camera::Camera;
use crate::graphics::rgb::RGB;
use std::time::Instant;


fn main() {

    let sphere1 = Sphere {
        center: V3 {x: 2.0, y: 0.0, z:0.0},
        radius: 0.5,
        color: RGB {r: 255, g: 255, b: 255},
    };

    let sphere2 = Sphere {
        center: V3 {x: 2.0, y: 1.0, z:0.0},
        radius: 0.2,
        color: RGB {r: 255, g: 0, b: 0},
    };

    let mut light_pos = V3 {x: 2.0, y:1.0, z:0.0};

    let mut light_point = light_pos;

    let cam = Camera {
        pos: V3 {x: 0.0, y: 0.0, z:0.0},
        height: 1080,
        width: 1920
    };

    

    let geometries = vec![&sphere1, &sphere2];

    loop {
        println!("light y pos : ");

        let mut input = String::new();
    
        stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    
        
        let input = input.trim();
        let nbr_input : f64 = input.parse::<f64>().unwrap();
        light_pos.y = nbr_input;
        light_point.y = nbr_input;

        let now = Instant::now();

        cam.take_picture(&geometries, &light_point, "test.png");

        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
    }

    

}

