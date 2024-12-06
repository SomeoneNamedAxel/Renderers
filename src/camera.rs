use std::cmp::min;
use crate::objects::geometry::Geometry;
use crate::objects::line::Line;
use crate::objects::v3::V3;

#[derive(Clone, Copy)]
pub struct Camera {
    pub pos: V3,
    pub width: u32,
    pub height: u32
}

impl Camera {
    pub fn take_picture(self, geometries: &Vec<Box<dyn Geometry>>, light_sources: &V3, img_path: &str) {
        let mut ray = Line {
            pos: V3 { x: 0.0, y: 0.0, z: 0.0 },
            dir: V3 { x: 1.0, y: 0.0, z: 0.6 }
        };

        let mut imgbuf = image::ImageBuffer::new(self.width, self.height);

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            ray.dir.y = (((x as i32) - (self.width / 2) as i32) as f64) / min(self.width, self.height) as f64;
            ray.dir.z = -(((y as i32) - (self.height / 2) as i32) as f64) / min(self.width, self.height) as f64;

            *pixel = image::Rgb([0, 0, 0]);

            

            for geometry in geometries.iter() {
                let intersection_result = geometry.get_intersections_points(&ray).unwrap_or(Vec::new());

                if !intersection_result.is_empty() {
                    let intersection_point = *intersection_result.iter().next().unwrap();
                    let ray_to_light = Line {
                        pos: intersection_point,
                        dir: (*light_sources),
                    };

                    let epsilon = 1e-5; // Small offset to avoid self-intersection
                    let ray_to_light = Line {
                        pos: V3 {
                            x: intersection_point.x + epsilon * ray_to_light.dir.x,
                            y: intersection_point.y + epsilon * ray_to_light.dir.y,
                            z: intersection_point.z + epsilon * ray_to_light.dir.z,
                        },
                        dir: (*light_sources - intersection_point).normalize(),
                    };


                    let ray_to_light_intersections : Vec<V3>= geometry.get_intersections_points(&ray_to_light).unwrap_or(Vec::new());


                    
                    if !ray_to_light_intersections.is_empty() {
                        *pixel = image::Rgb([10, 10, 10]); // Shadow color
                    } else {

                        *pixel = image::Rgb([geometry.get_color().r, geometry.get_color().g, geometry.get_color().b]); // Lit color
                    }
                }
            }
        }
        imgbuf.save(img_path).unwrap();
    }
}
pub fn launch_ray(){

}