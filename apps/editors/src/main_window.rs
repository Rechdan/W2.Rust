use eframe::{App, Frame};
use egui::{CentralPanel, Context};

use crate::editors::Editors;

pub struct MainWindow {
    editors: Editors,
}

impl Default for MainWindow {
    fn default() -> Self {
        Self {
            editors: Default::default(),
        }
    }
}

impl App for MainWindow {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("W2.Rust Editors");

                self.editors.render(ui);
            });
        });
    }
}
