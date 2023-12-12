#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

pub mod connection;
pub mod consts;
pub mod functions;
pub mod last_id;
pub mod main_window;
pub mod packet;
pub mod state;
pub mod statics;

use egui::{vec2, ViewportBuilder};
use main_window::MainWindow;

fn main() {
    let options = eframe::NativeOptions {
        centered: true,
        follow_system_theme: true,
        viewport: ViewportBuilder::default()
            .with_active(true)
            .with_min_inner_size(vec2(1240.0, 700.0)),
        ..Default::default()
    };

    let _app = eframe::run_native(
        "W2.Rust Sniffer",
        options,
        Box::new(|_cc| Box::<MainWindow>::new(MainWindow::new())),
    );
}
