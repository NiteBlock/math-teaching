use eframe::egui::Ui;

mod example;

pub trait Slide {
    fn name(&self) -> String;
    fn draw(&self, ui: Ui);
    fn new() -> Box<Self>
    where
        Self: Sized;
}
