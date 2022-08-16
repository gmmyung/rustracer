use crate::math::{Float, Ray, Vec3};
use crate::object::{self, Hit, HitAttr};
use crate::object::Hittable;
use indicatif::{ProgressBar, ProgressStyle};
use rand::{self, Rng};

pub struct Raytracer {
    pub image_width: usize,
    pub image_height: usize,
    objects: Vec<Box<dyn Hittable>>,
    pixel_buffer: Vec<Vec3>,
    multiple_sampling: usize,
}

impl Raytracer {
    pub fn new() -> Self {
        Self {
            image_width: 512,
            image_height: 512,
            objects: vec![
                Box::new(object::Sphere::new(Vec3::new(0.0, 3.0, 0.0), 0.5, Vec3::new(0.7,0.7,0.7))),
                Box::new(object::Sphere::new(Vec3::new(1.0, 3.0, 0.0), 0.5, Vec3::new(0.5,0.7,0.7))),
                Box::new(object::Sphere::new(Vec3::new(-1.0, 2.0, 0.0), 0.5, Vec3::new(0.7,0.3,0.7))),
                Box::new(object::Sphere::new(Vec3::new(0.2, 1.0, -0.3), 0.2, Vec3::new(0.7,0.3,0.7))),
                Box::new(object::Floor::new(-0.5, Vec3::new(0.5, 0.5, 0.5), true)),
            ],
            pixel_buffer: vec![Vec3::new(0.0, 0.0, 0.0); 512 * 512],
            multiple_sampling: 100,
        }
    }

    fn set_pixel(&mut self, x: usize, y: usize, sample_num: usize, color: Vec3) {
        let index = y * self.image_width + x;
        self.pixel_buffer[index] = (self.pixel_buffer[index] * (sample_num as Float) + color) * (1.0 / (sample_num as Float + 1.0));
    }

    pub fn run(&mut self) -> &Vec<Vec3> {
        let mut rng = rand::thread_rng();
        let pb = ProgressBar::new((self.image_height * self.multiple_sampling) as u64);
        pb.set_style(ProgressStyle::default_bar());
        pb.set_message("Raytracing...");

        let viewport_width = 2.0;
        let viewport_height = 2.0;
        let focal_length = 2.0;
        let origin = Vec3::new(0.0, -1.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, 0.0, viewport_height);
        let lower_left_corner =
            origin - horizontal * 0.5 - vertical * 0.5 + Vec3::new(0.0, focal_length, 0.0);
        for i in 0..self.multiple_sampling {
            for y in 0..self.image_height {
                for x in 0..self.image_width {
                    let r = Ray::new(origin, {
                        let rand_x = rng.gen_range(-0.5..0.5);
                        let rand_y = rng.gen_range(-0.5..0.5);
                        let u = (x as Float + rand_x) / self.image_width as Float;
                        let v = 1.0 - (y as Float + rand_y) / self.image_height as Float;
                        lower_left_corner + horizontal * u + vertical * v - origin
                    });
                    let rb = RayBouncer::new(r, 1000, &self.objects);
                    let color = if let Some(h) = rb.last(){
                        h.ray.color
                    } else {
                        panic!("No hit");
                    };
                    self.set_pixel(x, y, i, color);
                }
                pb.inc(1);
            }
        }
        &self.pixel_buffer
    }
}



pub struct RayBouncer<'a> {
    depth: usize,
    max_depth: usize,
    objects: &'a Vec<Box<dyn Hittable>>,
    hit: Hit,
}

impl<'a> RayBouncer<'a> {
    pub fn new(ray: Ray, max_depth:usize, objects: &'a Vec<Box<dyn Hittable>>) -> Self {
        Self { 
            depth:0, 
            max_depth, 
            objects, 
            hit: Hit {
                ray,
                t: 0.0,
                hitattr: HitAttr::FirstHit,
                prev_hit_index: None,
            }
        } 
    }

    pub fn ray_increment(&mut self){
        let mut closest_hit = Hit { 
            t: Float::INFINITY,
                    ray: Ray {
                origin: Vec3::new(0.0,0.0,0.0),
                direction: Vec3::new(0.0,0.0,0.0),
                color: self.hit.ray.color,
            },
            prev_hit_index: None,
            hitattr: HitAttr::LastHit
        };
        match self.hit.hitattr {
            HitAttr::LastHit => {
                self.hit = panic!("ray_increment input should not be last hit");
            }
            HitAttr::FirstHit | HitAttr::MidHit => {
                for (index, object) in self.objects.iter().enumerate() {
                    if let Some(exclude_index) = self.hit.prev_hit_index {
                        if index == exclude_index {
                            continue
                        }
                    }
                    let hit = object.hit(&self.hit);
                    match hit.hitattr{
                        HitAttr::MidHit | HitAttr::LastHit => {
                            if hit.t < closest_hit.t {
                                closest_hit = hit;
                                closest_hit.prev_hit_index = Some(index);
                            }
                        }
                        HitAttr::FirstHit => {
                            panic!("Hit return should not be first hit");
                        }
                    }
                }
                self.hit = closest_hit;
            }
        }
        self.depth += 1;

    }
}

impl<'a> Iterator for RayBouncer<'a> {
    type Item = Hit;
    fn next(&mut self) -> Option<Self::Item> {
        if self.depth >= self.max_depth {
            return None
        }
        self.depth += 1;
        self.ray_increment();
        if let HitAttr::LastHit = self.hit.hitattr {
            None
        } else {
            Some(self.hit)
        }
    }
}
