#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{run_native, NativeOptions};
use egui::{vec2, ViewportBuilder};
use main_window::MainWindow;

pub mod consts;
pub mod editors;
pub mod enc_dec;
pub mod get_file;
pub mod main_window;
pub mod structs;

fn main() {
    let options = NativeOptions {
        centered: true,
        follow_system_theme: true,
        viewport: ViewportBuilder::default()
            .with_active(true)
            .with_min_inner_size(vec2(900.0, 600.0)),
        ..NativeOptions::default()
    };

    match run_native(
        "W2.Rust Editors",
        options,
        Box::new(|_cc| Box::<MainWindow>::default()),
    ) {
        Ok(_ok) => {}
        Err(error) => {
            println!("main.run_native.error: {}", error);
        }
    };
}
