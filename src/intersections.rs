use crate::objects::{line::Line, sphere::Sphere};
use crate::objects::v3::V3;

pub fn line_and_sphere_intersection(line: &Line, sphere: &Sphere) -> Option<Vec<V3>> {
    let l = [
        line.pos.x - sphere.center.x,
        line.pos.y - sphere.center.y,
        line.pos.z - sphere.center.z,
    ];

    let a = line.dir.x * line.dir.x + line.dir.y * line.dir.y + line.dir.z * line.dir.z; // d · d
    let b = 2.0 * (line.dir.x * l[0] + line.dir.y * l[1] + line.dir.z * l[2]); // 2 * (d · L)
    let c = (l[0] * l[0] + l[1] * l[1] + l[2] * l[2]) - sphere.radius * sphere.radius; // L · L - R^2

    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return None;
    }

    let sqrt_discriminant = discriminant.sqrt();
    let t1 = (-b - sqrt_discriminant) / (2.0 * a);
    let t2 = (-b + sqrt_discriminant) / (2.0 * a);

    let mut points = Vec::new();

    let point1 = V3 {
        x: line.pos.x + t1 * line.dir.x,
        y: line.pos.y + t1 * line.dir.y,
        z: line.pos.z + t1 * line.dir.z,
    };
    points.push(point1);
    //println!("point 1: {},{},{}", points[0].x,points[0].y,points[0].z);

    if discriminant > 0.0 {
        let point2 = V3 {
            x: line.pos.x + t2 * line.dir.x,
            y: line.pos.y + t2 * line.dir.y,
            z: line.pos.z + t2 * line.dir.z,
        };
        points.push(point2);
        //println!("point 2: {},{},{}", points[1].x,points[1].y,points[1].z);
    }

    Some(points)
}