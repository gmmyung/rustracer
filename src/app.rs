use crate::{
    raytracer, 
    object::{Sphere, Floor, Hittable}, 
    math::Vec3, 
    reflection::{Diffuse, Mirror, Glass, DiffusedLightSource},
};
use eframe::egui;

const IMAGE_HEIGHT:usize = 512;
const IMAGE_WIDTH:usize = 512;
const SAMPLE_NUM: usize = 1024;

pub struct MyApp {
    image: egui::ColorImage,
}

impl MyApp {
    pub fn new() -> Self {
        MyApp {
            image: load_image_raytracer()
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Reload").clicked() {
                self.image = load_image_raytracer()
            }
            let texture = ui.ctx().load_texture("test image", self.image.clone());
            ui.image(
                &texture,
                egui::Vec2 {
                    x: 512.0,
                    y: 512.0,
                },
            );
        });
    }
}

fn load_image_raytracer() -> egui::ColorImage {
    
    let objects: Vec<Box<dyn Hittable + Sync + Send>> = vec![
        Box::new(Sphere::new(
            Vec3::new(0.0, 3.0, 0.2),
            0.5,
            Diffuse::new(Vec3::new(0.7, 0.7, 0.7)),
        )),
        Box::new(Sphere::new(
            Vec3::new(1.0, 3.0, 0.05),
            0.5,
            Mirror::new(Vec3::new(0.5, 0.7, 0.7)),
        )),
        Box::new(Sphere::new(
            Vec3::new(-0.6, 1.0, 0.1),
            0.3,
            Glass::new(Vec3::new(0.9, 0.9, 0.9), 2.0),
        )),
        Box::new(Sphere::new(
            Vec3::new(-0.2, 0.5, -0.3),
            0.2,
            Glass::new(Vec3::new(0.9, 0.9, 0.7), 1.3),
        )),
        Box::new(Sphere::new(
            Vec3::new(-0.2, 0.5, -0.3),
            0.17,
            Glass::new(Vec3::new(1.0, 1.0, 1.0), 1.0 / 1.3),
        )),
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
    ];
    let raytracer = raytracer::Raytracer::new(IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLE_NUM);
    let pixels = raytracer.run(objects);
    let mut image_buffer = Vec::new();
    for i in 0..pixels.len() {
        image_buffer.push((pixels[i].x.sqrt() * 256.0 ) as u8);
        image_buffer.push((pixels[i].y.sqrt() * 256.0 ) as u8);
        image_buffer.push((pixels[i].z.sqrt() * 256.0 ) as u8);
        image_buffer.push(255);
    }
    // image buffer to file
    let path = "img/hello_world.png";
    let mut file = std::fs::File::create(path).unwrap();
    let mut encoder = png::Encoder::new(&mut file, IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&image_buffer).unwrap();
    println!("image saved to {path}");


    egui::ColorImage::from_rgba_unmultiplied(
        [raytracer.image_width as _, raytracer.image_height as _],
        image_buffer.as_slice(),
    )
}