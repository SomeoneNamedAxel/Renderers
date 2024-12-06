use crate::graphics::rgb::RGB;
use super::v3::V3;
use crate::objects::line::Line;

pub trait Geometry {
    fn get_intersections_points(&self, line: &Line) -> Option<Vec<V3>>;
    fn get_color(&self) -> RGB;
    fn get_pos(&self) -> V3;
}