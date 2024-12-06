mod objects;
mod camera;
mod graphics;

use crate::camera::Camera;
use crate::graphics::rgb::RGB;
use crate::objects::geometry::Geometry;
use objects::cube::Cube;
use objects::sphere::Sphere;
use objects::v3::V3;
use std::io::stdin;


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

    let cube1 = Cube {
        pos: V3 {x: 6.0, y: -1.0, z:0.0},
        size: 0.5,
        color: RGB {r: 0, g: 175, b: 0},
    };

    let mut light_pos = V3 {x: 2.0, y:3.0, z:4.0};

    let mut light_point = light_pos;

    let cam = Camera {
        pos: V3 {x: 0.0, y: 0.0, z:0.0},
        height: 1080,
        width: 1920
    };


    let mut geometries: Vec<Box<dyn Geometry>> = Vec::new();

    geometries.push(Box::new(sphere1));
    geometries.push(Box::new(sphere2));
    geometries.push(Box::new(cube1));

    loop {
        cam.take_picture(&geometries, &light_point, "test.png");

        println!("light y pos : ");

        let mut input = String::new();
    
        stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    
        
        let input = input.trim();
        let nbr_input : f64 = input.parse::<f64>().unwrap();
        light_pos.y = nbr_input;
        light_point.y = nbr_input;
    }
}

