use crate::math::{self, rand, Float, Ray, Vec3};
use crate::object::Hittable;
use crate::reflection::{HitAttr, HitKind};
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Raytracer {
    pub image_width: usize,
    pub image_height: usize,
    sample_num: usize,
    thread_num: usize,
}

impl Raytracer {
    pub fn new(image_width: usize, image_height: usize, sample_num: usize) -> Self {
        Self {
            image_width,
            image_height,
            sample_num,
            thread_num: {
                let num = num_cpus::get();
                println!("{} core{} in use!", num, if num > 1 { "s" } else { "" });
                num_cpus::get()
            },
        }
    }

    pub fn run(&self, objects: Vec<Box<dyn Hittable + Send + Sync>>) -> Vec<Vec3> {
        let mut thread_pool = Vec::new();
        let pb = ProgressBar::new((self.image_height * self.image_width * self.thread_num) as u64);
        pb.set_style(ProgressStyle::default_bar());
        pb.set_message("Raytracing...");
        let mutex_pb = Arc::new(Mutex::new(pb));
        let pixel_buffer = vec![Vec3::zero(); self.image_height * self.image_width];
        let mutex_pixelbuffer = Arc::new(Mutex::new(pixel_buffer));
        let arc_objects = Arc::new(objects);
        let image_width = self.image_width;
        let image_height = self.image_height;
        let sample_num = self.sample_num / self.thread_num;

        for _ in 0..self.thread_num {
            let mutex_pb = Arc::clone(&mutex_pb);
            let mutex_pixelbuffer = Arc::clone(&mutex_pixelbuffer);
            let arc_objects = Arc::clone(&arc_objects);
            thread_pool.push(thread::spawn(move || {
                let viewport_width = 2.0;
                let viewport_height = 2.0;
                let focal_length = 2.0;
                let origin = Vec3::new(0.0, -1.0, 0.0);
                let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
                let vertical = Vec3::new(0.0, 0.0, viewport_height);
                let lower_left_corner =
                    origin - horizontal * 0.5 - vertical * 0.5 + Vec3::new(0.0, focal_length, 0.0);
                for y in 0..image_height {
                    for x in 0..image_width {
                        let mut accum_color = Vec3::zero();
                        for _ in 0..sample_num {
                            let r = Ray::new(origin, {
                                let rand_x = rand() - 0.5;
                                let rand_y = rand() - 0.5;
                                let u = (x as Float + rand_x) / image_width as Float;
                                let v = 1.0 - (y as Float + rand_y) / image_height as Float;
                                lower_left_corner + horizontal * u + vertical * v - origin
                            });
                            let rb = RayBouncer::new(r, 1000, &*arc_objects);
                            let color = if let Some(h) = rb.last() {
                                h.ray.color
                            } else {
                                Vec3::sky_color()
                            };
                            accum_color += color;
                        }
                        let color = accum_color * (1.0 / (sample_num as Float));
                        {
                            let mut pixel_buffer = mutex_pixelbuffer.lock().unwrap();
                            pixel_buffer[y * image_width + x] =
                                pixel_buffer[y * image_width + x] + color;
                        }
                        mutex_pb.lock().unwrap().inc(1);
                    }
                }
            }));
        }
        for j in thread_pool {
            j.join().unwrap();
        }
        let mut pixel_buffer = mutex_pixelbuffer.lock().unwrap();
        for x in 0..image_height {
            for y in 0..image_width {
                pixel_buffer[y * image_width + x] =
                    pixel_buffer[y * image_width + x] * (1.0 / (self.thread_num as Float));
            }
        }
        let res = pixel_buffer.clone();
        res
    }
}

/// RayBouncer is an iterator that iterates bounces the ray until it hits nothing.
/// It returns the HitAttr of the last bounce every time it is called.
/// If the ray bounces nothing, it returns None.
struct RayBouncer<'a> {
    depth: usize,
    max_depth: usize,
    objects: &'a Vec<Box<dyn Hittable + Send + Sync>>,
    hitattr: Option<HitAttr>,
}

impl<'a> RayBouncer<'a> {
    /// Constructs a new RayBouncer. The ray bounces until it hits nothing or the maximum depth.
    pub fn new(
        ray: Ray,
        max_depth: usize,
        objects: &'a Vec<Box<dyn Hittable + Send + Sync>>,
    ) -> Self {
        Self {
            depth: 0,
            max_depth,
            objects,
            hitattr: Some(HitAttr {
                t: 0.0,
                ray,
                hitkind: HitKind::NormalHit,
            }),
        }
    }

    /// Returns the last hit attribute. After the last hit(e.g. Hits the sky), returns None.
    fn ray_increment(
        h: &HitAttr,
        objects: &Vec<Box<dyn Hittable + Send + Sync>>,
    ) -> Option<HitAttr> {
        // Returns None when the ray hits the sky.
        if let HitKind::LastHit = h.hitkind {
            return None;
        }
        let mut closest_hitattr: Option<HitAttr> = None;
        for object in objects.iter() {
            let mut next_hitattr = object.hit(&h);
            // Make sure to use math::EPSILON defined in this crate, not std::f32::EPSILON
            // Add a small epsilon to avoid shadow acne
            next_hitattr.ray.origin =
                next_hitattr.ray.origin + next_hitattr.ray.direction * math::EPSILON;
            match closest_hitattr {
                None => {
                    closest_hitattr = Some(next_hitattr);
                }
                Some(prev_hitattr) => {
                    // If the next hit is closer than the previous hit, replace the previous hit with the next hit.
                    if next_hitattr.t < prev_hitattr.t {
                        closest_hitattr = Some(next_hitattr);
                    }
                }
            }
        }
        closest_hitattr
    }
}

impl<'a> Iterator for RayBouncer<'a> {
    type Item = HitAttr;
    fn next(&mut self) -> Option<Self::Item> {
        if self.depth < self.max_depth {
            self.depth += 1;
            if let Some(hitattr) = self.hitattr {
                if let Some(next_hitattr) = RayBouncer::ray_increment(&hitattr, self.objects) {
                    self.hitattr = Some(next_hitattr);
                    Some(next_hitattr)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}
