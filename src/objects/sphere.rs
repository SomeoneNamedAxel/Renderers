use super::v3::V3;
use crate::graphics::rgb::RGB;
use crate::objects::geometry::Geometry;
use crate::objects::line::Line;

pub struct Sphere {
    pub center: V3,
    pub radius: f64,
    pub color: RGB
}

impl Geometry for Sphere {
    fn get_intersections_points(&self, line: &Line) -> Option<Vec<V3>> {
        let l = [
            line.pos.x - self.center.x,
            line.pos.y - self.center.y,
            line.pos.z - self.center.z,
        ];

        let a = line.dir.x * line.dir.x + line.dir.y * line.dir.y + line.dir.z * line.dir.z; // d · d
        let b = 2.0 * (line.dir.x * l[0] + line.dir.y * l[1] + line.dir.z * l[2]); // 2 * (d · L)
        let c = (l[0] * l[0] + l[1] * l[1] + l[2] * l[2]) - self.radius * self.radius; // L · L - R^2

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None; // No intersection
        }

        let sqrt_discriminant = discriminant.sqrt();
        let t1 = (-b - sqrt_discriminant) / (2.0 * a);
        let t2 = (-b + sqrt_discriminant) / (2.0 * a);

        let mut points = Vec::new();

        if t1 > 0.0 {
            points.push(V3 {
                x: line.pos.x + t1 * line.dir.x,
                y: line.pos.y + t1 * line.dir.y,
                z: line.pos.z + t1 * line.dir.z,
            });
        }

        if t2 > 0.0 {
            points.push(V3 {
                x: line.pos.x + t2 * line.dir.x,
                y: line.pos.y + t2 * line.dir.y,
                z: line.pos.z + t2 * line.dir.z,
            });
        }

        if points.is_empty() {
            None
        } else {
            Some(points)
        }
    }

    fn get_color(&self) -> RGB {
        self.color
    }

    fn get_pos(&self) -> V3 {
        self.center
    }
}