use crate::intersections;
use crate::objects::line::Line;
use crate::objects::sphere::Sphere;
use crate::objects::v3::V3;

#[derive(Clone, Copy)]
pub struct Camera {
    pub pos: V3,
    pub width: u32,
    pub height: u32,
    pub weird_depth: f64
}

impl Camera {
    pub fn take_picture(self, geometries: &Vec<&Sphere>, light_sources: &V3, img_path: &str) {
        let mut ray = Line {
            pos: V3 { x: 0.0, y: 0.0, z: 0.0 },
            dir: V3 { x: 1.0, y: 0.0, z: 0.6 }
        };

        let mut imgbuf = image::ImageBuffer::new(self.width, self.height);

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            ray.dir.z = (((x as i32) - (self.width / 2) as i32) as f64) / self.weird_depth;
            ray.dir.y = (((y as i32) - (self.height / 2) as i32) as f64) / self.weird_depth;

            let white: u8 = 254;
            *pixel = image::Rgb([0, 0, 0]);

            for geometry in geometries {
                let intersection_result = intersections::line_and_sphere_intersection(&ray, &geometry).unwrap_or(Vec::new());

                if !intersection_result.is_empty() {
                    //println!("{},{},{}", intersection_result.iter().next().unwrap().x, intersection_result.iter().next().unwrap().y, intersection_result.iter().next().unwrap().z);
                    let ray_to_light = Line {
                        pos: *intersection_result.iter().next().unwrap(),
                        dir: *light_sources
                    };
                    //println!("ray_to_light pos : {},{},{}", intersection_result.iter().next().unwrap().x,intersection_result.iter().next().unwrap().y,intersection_result.iter().next().unwrap().z);
                    //println!("ray_to_light dir : {},{},{}", light_sources.x,light_sources.y,light_sources.z);



                    if intersections::line_and_sphere_intersection(&ray_to_light, &geometry).is_some() {
                        *pixel = image::Rgb([geometry.color.r, geometry.color.g, geometry.color.b]);
                    } else {
                        *pixel = image::Rgb([0, 0, 0]);
                    }
                }
            }
        }
        imgbuf.save(img_path).unwrap();
    }
}
pub fn launch_ray(){

}