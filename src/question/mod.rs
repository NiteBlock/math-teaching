use eframe::egui::Ui;

pub trait Question {
    fn name(&self) -> String;
    fn incorrect_answers(&self) -> [&str; 3];
    fn answer(&self) -> &str;
    fn draw(&self, ui: Ui);
    fn new() -> Box<Self>
    where
        Self: Sized;
}
