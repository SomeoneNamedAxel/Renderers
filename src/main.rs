mod objects;
mod camera;
mod intersections;
mod graphics;

use objects::sphere::Sphere;
use objects::line::Line;
use objects::v3::V3;

use crate::camera::take_picture;

fn main() {

    let sphere1 = Sphere {
        center: V3 {x: 2.0, y: 0.0, z:0.0},
        radius: 0.5
    };

    let sphere2 = Sphere {
        center: V3 {x: 2.0, y: 0.0, z:2.0},
        radius: 0.5
    };

    let light_point = V3 {x: 0.0, y: 0.0, z: 10.0};

    let geometries = vec![&sphere1, &sphere2];

    take_picture(V3 {x: 0.0, y: 0.0, z:0.0},geometries, light_point, 100.0, 200, 200, "test.png");

}

