use super::v3::V3;
use crate::graphics::rgb::RGB;
use crate::objects::geometry::Geometry;
use crate::objects::line::Line;

pub struct Cube {
    pub pos: V3,
    pub size: f64,
    pub color: RGB
}

impl Geometry for Cube {
    fn get_intersections_points(&self, line: &Line) -> Option<Vec<V3>> {
        let half_size = self.size / 2.0;

        // Define the min and max bounds of the cube
        let min = V3 {
            x: self.pos.x - half_size,
            y: self.pos.y - half_size,
            z: self.pos.z - half_size,
        };
        let max = V3 {
            x: self.pos.x + half_size,
            y: self.pos.y + half_size,
            z: self.pos.z + half_size,
        };

        let mut t_min = f64::NEG_INFINITY;
        let mut t_max = f64::INFINITY;

        // Check for intersection with each pair of planes (X, Y, Z)
        for i in 0..3 {
            let (line_origin, line_direction, min_bound, max_bound) = match i {
                0 => (line.pos.x, line.dir.x, min.x, max.x), // X-axis
                1 => (line.pos.y, line.dir.y, min.y, max.y), // Y-axis
                2 => (line.pos.z, line.dir.z, min.z, max.z), // Z-axis
                _ => unreachable!(),
            };

            if line_direction.abs() < 1e-6 {
                // Line is parallel to the planes
                if line_origin < min_bound || line_origin > max_bound {
                    return None; // Outside the slab
                }
            } else {
                // Compute intersection t values for the slab
                let t1 = (min_bound - line_origin) / line_direction;
                let t2 = (max_bound - line_origin) / line_direction;

                // Ensure t1 is the near intersection and t2 is the far intersection
                let (t1, t2) = if t1 > t2 { (t2, t1) } else { (t1, t2) };

                // Update t_min and t_max to find the intersection range
                t_min = t_min.max(t1);
                t_max = t_max.min(t2);

                // If t_min exceeds t_max, there's no intersection
                if t_min > t_max {
                    return None;
                }
            }
        }

        // If we have valid t_min and t_max, calculate intersection points
        let mut points = Vec::new();

        if t_min > 0.0 {
            points.push(V3 {
                x: line.pos.x + t_min * line.dir.x,
                y: line.pos.y + t_min * line.dir.y,
                z: line.pos.z + t_min * line.dir.z,
            });
        }

        if t_max > 0.0 {
            points.push(V3 {
                x: line.pos.x + t_max * line.dir.x,
                y: line.pos.y + t_max * line.dir.y,
                z: line.pos.z + t_max * line.dir.z,
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
        self.pos
    }
}
