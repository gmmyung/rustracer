use crate::math::{Float, Ray, Vec3};
use crate::object::{Sphere, Floor, Hittable};
use crate::reflection::{Diffuse, Mirror, HitAttr, HitKind, DiffusedLightSource};
use indicatif::{ProgressBar, ProgressStyle};
use rand::{self, Rng};

pub struct Raytracer {
    pub image_width: usize,
    pub image_height: usize,
    objects: Vec<Box<dyn Hittable>>,
    pixel_buffer: Vec<Vec3>,
    sample_num: usize,
    max_depth: usize,
}

impl Raytracer {
    pub fn new(image_width: usize, image_height: usize, sample_num: usize, max_depth: usize) -> Self {
        Self {
            image_width,
            image_height,
            objects: 
            // {
            //     let mut objvec: Vec<Box<dyn Hittable>> = Vec::new();
            //     for _ in 0..50 {
            //         let mut rng = rand::thread_rng();
            //         let center = Vec3::new(rng.gen_range(-2.0..2.0), rng.gen_range(2.0..5.0), rng.gen_range(-1.5..0.5));
            //         let radius = rng.gen_range(0.1..0.4);
            //         let color = Vec3::new(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0));
            //         let material = rng.gen_range(0..5);
            //         match material {
            //             0|1 => {objvec.push(Box::new(Sphere::new(center, radius, Diffuse::new(color))));},
            //             2|3 => {objvec.push(Box::new(Sphere::new(center, radius, Mirror::new(color))));},
            //             _ => {objvec.push(Box::new(Sphere::new(center, radius, DiffusedLightSource::new(color * 2.0))));},
            //         }
            //     }
            //     objvec.push(Box::new(Floor::new(
            //         -2.0,
            //         true,
            //         Diffuse::new(Vec3::new(0.5, 0.5, 0.5))
            //     )));
            //     objvec
            // },
            vec![
                Box::new(Sphere::new(
                    Vec3::new(0.0, 3.0, 0.2),
                    0.5,
                    Mirror::new(Vec3::new(0.7, 0.7, 0.7)),
                )),
                Box::new(Sphere::new(
                    Vec3::new(1.0, 3.0, 0.05),
                    0.5,
                    Diffuse::new(Vec3::new(0.5, 0.7, 0.7)),
                )),
                // Box::new(Sphere::new(
                //     Vec3::new(-1.0, 2.0, 0.1),
                //     0.5,
                //     Glass::new(Vec3::new(0.7, 0.3, 0.7), 1.05),
                // )),
                Box::new(Sphere::new(
                    Vec3::new(-1.0, 5.0, 0.1),
                    0.3,
                    Diffuse::new(Vec3::new(0.7, 0.3, 0.7)),
                )),
                Box::new(Sphere::new(
                    Vec3::new(0.2, 1.0, -0.25),
                    0.2,
                    DiffusedLightSource::new(Vec3::new(5.0, 5.0, 5.0)),
                )),
                Box::new(Floor::new(
                    -0.5,
                    true,
                    Diffuse::new(Vec3::new(0.5, 0.5, 0.5)),
                )),
            ],
            pixel_buffer: vec![Vec3::zero(); image_height * image_width],
            sample_num,
            max_depth
        }
    }

    fn set_pixel(&mut self, x: usize, y: usize, sample_num: usize, color: Vec3) {
        let index = y * self.image_width + x;
        self.pixel_buffer[index] = (self.pixel_buffer[index] * (sample_num as Float) + color)
            * (1.0 / (sample_num as Float + 1.0));
    }

    pub fn run(&mut self) -> &Vec<Vec3> {
        let mut rng = rand::thread_rng();
        let pb = ProgressBar::new((self.image_height * self.sample_num) as u64);
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
        for i in 0..self.sample_num {
            for y in 0..self.image_height {
                for x in 0..self.image_width {
                    let r = Ray::new(origin, {
                        let rand_x = rng.gen_range(-0.5..0.5);
                        let rand_y = rng.gen_range(-0.5..0.5);
                        let u = (x as Float + rand_x) / self.image_width as Float;
                        let v = 1.0 - (y as Float + rand_y) / self.image_height as Float;
                        lower_left_corner + horizontal * u + vertical * v - origin
                    });
                    let rb = RayBouncer::new(r, self.max_depth, &self.objects);
                    let color = if let Some(h) = rb.last() {
                        h.ray.color
                    } else {
                        Vec3::sky_color()
                    };
                    self.set_pixel(x, y, i, color.sqrt());
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
    hitattr: Option<HitAttr>,
}

impl<'a> RayBouncer<'a> {
    pub fn new(
        ray: Ray,
        max_depth: usize,
        objects: &'a Vec<Box<dyn Hittable>>,
    ) -> Self {
        Self {
            depth: 0,
            max_depth,
            objects,
            hitattr: Some(HitAttr {
                t: 0.0,
                ray,
                prev_hit_index: None,
                hitkind: HitKind::NormalHit
            }),
        }
    }

    pub fn ray_increment(h: &HitAttr, objects: &Vec<Box<dyn Hittable>>) -> Option<HitAttr> {
        if let HitKind::LastHit = h.hitkind {
            return None;
        }
        let mut closest_hitattr: Option<HitAttr> = None;
        for (i, object) in objects.iter().enumerate() {
            // if let Some(pi) = h.prev_hit_index {
            //     if i == pi {
            //         continue;
            //     }
            // }
            let next_hitattr = object.hit(&h, i);
            if next_hitattr.prev_hit_index.is_none() {
                continue;
            }
            match closest_hitattr {
                None => {
                    closest_hitattr = Some(next_hitattr);
                }
                Some(prev_hitattr) => {
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
