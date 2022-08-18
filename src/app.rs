
use crate::raytracer;
use eframe::egui;

const IMAGE_HEIGHT:usize = 512;
const IMAGE_WIDTH:usize = 512;
const SAMPLE_NUM: usize = 256;

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
                    x: IMAGE_WIDTH as f32,
                    y: IMAGE_HEIGHT as f32,
                },
            );
        });
    }
}

fn load_image_raytracer() -> egui::ColorImage {
    let mut raytracer = raytracer::Raytracer::new(IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLE_NUM);
    let pixels = raytracer.run();
    let mut image_buffer = Vec::new();
    for i in 0..pixels.len() {
        image_buffer.push((pixels[i].x * 256.0 ) as u8);
        image_buffer.push((pixels[i].y * 256.0 ) as u8);
        image_buffer.push((pixels[i].z * 256.0 ) as u8);
        image_buffer.push(255);
    }
    egui::ColorImage::from_rgba_unmultiplied(
        [raytracer.image_width as _, raytracer.image_height as _],
        image_buffer.as_slice(),
    )
}