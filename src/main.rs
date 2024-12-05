mod objects;
mod camera;
mod intersections;
mod graphics;

use objects::sphere::Sphere;
use objects::line::Line;
use objects::v3::V3;

use crate::camera::{Camera};
use crate::graphics::rgb::RGB;

fn main() {

    let sphere1 = Sphere {
        center: V3 {x: 2.0, y: 0.0, z:0.0},
        radius: 0.5,
        color: RGB {r: 255, g: 255, b: 255},
    };

    let sphere2 = Sphere {
        center: V3 {x: 2.0, y: 0.0, z:2.0},
        radius: 0.5,
        color: RGB {r: 255, g: 0, b: 0},
    };

    let temoin_point_1 = Sphere {
        center: V3 {x:  1.7034243813445806, y:-0.3781602126584969, z:0.13797737488891104},
        radius: 0.01,
        color: RGB {r: 0, g: 0, b: 255},
    };
    let temoin_point_2 = Sphere {
        center: V3 {x: 2.0, y:1.0, z:0.0},
        radius: 0.01,
        color: RGB {r: 0, g: 255, b: 0},
    };

    let cam = Camera {
        pos: V3 {x: 0.0, y: 0.0, z:0.0},
        height: 1080,
        width: 1920,
        weird_depth: 1000.0
    };

    let light_point = V3 {x: 2.0, y:0.0, z:1.0};

    let geometries = vec![&sphere1, &sphere2, &temoin_point_1, &temoin_point_2];

    cam.take_picture(&geometries, &light_point, "test.png");

}

