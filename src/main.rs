use eframe;
use rustracer::app;
use rustracer::raytracer;


fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Hello world", 
        options, 
        Box::new(|_cc| Box::new(app::MyApp::new())),
    );
}