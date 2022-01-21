mod app;
mod error;
mod question;
mod slide;
mod topic;
mod utils;

use app::MathTeachingApp;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    let app = MathTeachingApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
