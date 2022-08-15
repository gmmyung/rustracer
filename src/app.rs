use crate::math::{ray, vec3, Float};
use crate::raytracer;
use eframe::{
    egui,
    epaint::{TextureHandle, TextureId},
    CreationContext,
};

pub struct MyApp {
    image: egui::ColorImage,
}

impl MyApp {
    pub fn new() -> Self {
        MyApp {
            image: load_image_raytracer()
            // load_image_from_path(std::path::Path::new(
            //     "/Users/gyungmin/Developer/rustracer/IMG_4FE346550ADA-1.jpeg",
            // ))
            // .unwrap(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Reload").clicked() {
                self.image = load_image_raytracer()
            }
            let texture = ui.ctx().load_texture("test image", self.image.clone());
            ui.image(
                &texture,
                egui::Vec2 {
                    x: 500.0,
                    y: 500.0,
                },
            );
        });
    }
}

fn load_image_raytracer() -> egui::ColorImage {
    let mut raytracer = raytracer::Raytracer::new();
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

fn load_image_from_path(path: &std::path::Path) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}



fn test_image() -> egui::ColorImage {
    let image_width = 256;
    let image_height = 256;

    let mut image_buffer = Vec::with_capacity(image_width * image_height * 4);
    for y in 0..image_height {
        for x in 0..image_width {
            image_buffer.push(x as u8);
            image_buffer.push(y as u8);
            image_buffer.push(0);
            image_buffer.push(255);
        }
    }

    egui::ColorImage::from_rgba_unmultiplied(
        [image_width as _, image_height as _],
        image_buffer.as_slice(),
    )
}
