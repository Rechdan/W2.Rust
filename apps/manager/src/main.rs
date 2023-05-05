use eframe::{run_native, NativeOptions};
use egui::vec2;
use main_window::MainWindow;

pub mod main_window;

fn main() {
    let app_size = Some(vec2(1200.0, 700.0));

    let options = NativeOptions {
        initial_window_size: app_size,
        min_window_size: app_size,
        ..NativeOptions::default()
    };

    let _app = run_native(
        "W2.Rust GameServer Manager",
        options,
        Box::new(|_cc| Box::<MainWindow>::default()),
    );
}
