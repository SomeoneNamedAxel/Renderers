use crate::intersections;
use crate::objects::line::Line;
use crate::objects::sphere::Sphere;
use crate::objects::v3::V3;

pub fn take_picture(cam_pos: V3, geometries : Vec<&Sphere>, light_sources: V3, weird_depth: f64, cam_width: u32, cam_height: u32, img_path: &str) {

    let mut ray = Line {
        pos: V3 {x: 0.0, y: 0.0, z:0.0},
        dir: V3 {x: 1.0, y: 0.0, z:0.6}
    };

    let mut imgbuf = image::ImageBuffer::new(cam_width, cam_height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {

        ray.dir.z = (((x as i32)-(cam_width/2) as i32) as f64)/weird_depth;
        ray.dir.y = (((y as i32)-(cam_height/2) as i32) as f64)/weird_depth;

        let white : u8 = 254;
        *pixel = image::Rgb([0,0,0]);

        for geometry in &geometries {
            if intersections::line_and_sphere_intersection(&ray, &geometry).is_some() {
                *pixel = image::Rgb([white,white,white]);
            }
        }
    }
    imgbuf.save(img_path).unwrap();
}

pub fn launch_ray(){

}