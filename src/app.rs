use eframe::{egui, epi};

#[derive(Default)]
pub struct MathTeachingApp {}

impl epi::App for MathTeachingApp {
    fn name(&self) -> &str {
        "Math Teaching App"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
        });
    }
}
