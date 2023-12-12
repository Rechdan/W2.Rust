use eframe::App;
use egui::CentralPanel;
use rfd::FileDialog;

pub struct MainWindow {}

impl Default for MainWindow {
    fn default() -> Self {
        Self {}
    }
}

impl App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("W2.Rust Editors");

                if ui.button("Selecionar").clicked() {
                    match FileDialog::new().pick_folder() {
                        Some(folder) => {
                            println!("Folder: {}", folder.display());
                        }
                        None => {}
                    };
                }
            });
        });
    }
}
