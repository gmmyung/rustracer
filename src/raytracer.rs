use crate::math::{ray, vec3, Float};
use crate::object;
use indicatif::{ProgressBar, ProgressStyle};

pub struct Raytracer {
    pub image_width: usize,
    pub image_height: usize,
    objects: Vec<object::sphere>,
    pixel_buffer: Vec<vec3>,
}

impl Raytracer {
    pub fn new() -> Self {
        Self {
            image_width: 256,
            image_height: 256,
            objects: vec![object::sphere::new(vec3::new(0.0, 0.0, 0.0), 0.5)],
            pixel_buffer: vec![vec3::new(0.0, 0.0, 0.0); 256 * 256],
        }
    }

    fn set_pixel(&mut self, x: usize, y: usize, color: vec3) {
        let index = y * self.image_width + x;
        self.pixel_buffer[index] = color;
    }

    pub fn run(&mut self) -> &Vec<vec3> {
        let pb = ProgressBar::new(self.image_height as u64);
        pb.set_style(
            ProgressStyle::default_bar()
        );
        pb.set_message("Raytracing...");
        for y in 0..self.image_height {
            for x in 0..self.image_width {
                let r = ray::new(vec3::new((x as f32) / 256.0, (y as f32) / 256.0, 0.0), vec3::new(0.0, 0.0, 1.0));
                self.set_pixel(x, y, self.ray_color(r));
            }
            pb.inc(1);
        }
        &self.pixel_buffer
    }

    pub fn ray_color(&self, r: ray) -> vec3 {
        let hit_by_object = self
            .objects
            .iter()
            .any(|s| if s.hit_sphere(&r) { true } else { false });
        if hit_by_object {
            vec3::new(1.0, 0.0, 0.0)
        } else {
            vec3::new(0.0, 0.0, 0.0)
        }
    }
}
