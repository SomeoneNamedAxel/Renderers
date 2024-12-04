use crate::objects::{line::Line, sphere::Sphere};

pub fn line_and_sphere(line: &Line, sphere: &Sphere) -> bool
{
    let l = [
        line.pos.x - sphere.center.x,
        line.pos.y - sphere.center.y,
        line.pos.z - sphere.center.z,
    ];

    let a = line.dir.x * line.dir.x + line.dir.y * line.dir.y + line.dir.z * line.dir.z; // d · d
    let b = 2.0 * (line.dir.x * l[0] + line.dir.y * l[1] + line.dir.z * l[2]); // 2 * (d · L)
    let c = (l[0] * l[0] + l[1] * l[1] + l[2] * l[2]) - sphere.radius * sphere.radius; // L · L - R^2

    let discriminant = b * b - 4.0 * a * c;

    discriminant >= 0.0
}