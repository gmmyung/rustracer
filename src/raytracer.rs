use crate::math::{Float, Ray, Vec3};
use crate::object::{self, HitAttr};
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
                Box::new(object::Floor::new(-0.5, Vec3::new(0.7, 0.7, 0.7), true)),
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
                    let color = rb.last().unwrap();
                    self.set_pixel(x, y, i, color);
                }
                pb.inc(1);
            }
        }
        &self.pixel_buffer
    }
}



pub struct RayBouncer<'a> {
    ray: Ray,
    depth: usize,
    max_depth: usize,
    objects: &'a Vec<Box<dyn Hittable>>,
    prev_hit_index: Option<usize>
}

impl<'a> RayBouncer<'a> {
    pub fn new(ray: Ray, max_depth:usize, objects: &'a Vec<Box<dyn Hittable>>) -> Self {
        Self { ray, depth:0, max_depth, objects,  prev_hit_index: None }
    }

    pub fn ray_increment(r: &Ray, objects: &Vec<Box<dyn Hittable>>, prev_index: Option<usize>) -> Option<(Option<usize>, HitAttr)> {
        let (index, hitattr) = objects
            .iter()
            .enumerate()
            .map(|(index, s)| {
                if let object::Hitten::Hit(hitattr) = s.hit(&r) {
                    if let Some(pi) = prev_index {
                        if pi == index {
                            (index, HitAttr {
                                t: Float::INFINITY,
                                p: Vec3::new(0.0, 0.0, 0.0),
                                normal: Vec3::new(0.0, 0.0, 0.0),
                                color: Vec3::new(0.0, 0.0, 0.0),
                            })
                        }
                        else {
                            (index, hitattr)
                        }
                    } else {
                        (index, hitattr)
                    }
                } else {
                    (index, HitAttr {
                        t: Float::INFINITY,
                        p: Vec3::new(0.0, 0.0, 0.0),
                        normal: Vec3::new(0.0, 0.0, 0.0),
                        color: Vec3::new(0.0, 0.0, 0.0),
                    })
                }
            })
            .reduce(|(index_a, hitattr_a), (index_b, hitattr_b)| {
                if hitattr_a.t < hitattr_b.t {
                    (index_a, hitattr_a)
                } else {
                    (index_b, hitattr_b)
                }
            })
            .unwrap();
        if hitattr.t < Float::INFINITY {
            Some((Some(index), hitattr))
        } else {
            None
        }
    }
}

impl<'a> Iterator for RayBouncer<'a> {
    type Item = Vec3;
    fn next(&mut self) -> Option<Self::Item> {
        if self.depth < self.max_depth {
            let hitattr = RayBouncer::ray_increment(
                &self.ray,
                self.objects, 
                self.prev_hit_index
            );
            match hitattr {
                None => {
                    self.depth = self.max_depth;
                    Some(self.ray.color)
                }
                Some((index, hitattr)) => {
                    self.prev_hit_index = index;
                    self.depth += 1;
                    self.ray = Ray{
                        origin: hitattr.p,
                        direction: hitattr.normal.random_diffusion(),
                        color: hitattr.color
                    };
                    Some(hitattr.color)
                }
            }
        } else {
            None
        }
    }
}
