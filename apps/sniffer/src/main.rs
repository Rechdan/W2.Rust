#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

pub mod connection;
pub mod consts;
pub mod functions;
pub mod last_id;
pub mod main_window;
pub mod packet;
pub mod state;
pub mod statics;

use egui::vec2;
use main_window::MainWindow;

fn main() {
    let app_size = Some(vec2(1240.0, 700.0));

    let options = eframe::NativeOptions {
        initial_window_size: app_size,
        min_window_size: app_size,
        ..Default::default()
    };

    let _app = eframe::run_native(
        "W2.Rust Sniffer",
        options,
        Box::new(|_cc| Box::<MainWindow>::default()),
    );
}
