use crate::slide::Slide;
use eframe::egui::Ui;

pub struct ExampleSlide {}

impl Slide for ExampleSlide {
    fn name(&self) -> String {
        "Example Slide".to_string()
    }
    fn draw(&self, ui: Ui) {}
    fn new() -> Box<ExampleSlide> {
        Box::new(ExampleSlide {})
    }
}
