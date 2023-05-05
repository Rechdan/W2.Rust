use eframe::App;
use egui::CentralPanel;

pub struct MainWindow {}

impl Default for MainWindow {
    fn default() -> Self {
        Self {}
    }
}

impl App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.heading("Future W2.Rust Manager");
            });
        });
    }
}
