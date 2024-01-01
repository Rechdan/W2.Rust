use eframe::{App, Frame};
use egui::{Align, CentralPanel, Context, Layout, TopBottomPanel};
use rfd::FileDialog;

use crate::editors::Editors;

#[derive(Default)]
pub struct MainWindow {
    editors: Option<Box<Editors>>,
}

impl App for MainWindow {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        match self.editors.is_some() {
            true => self.editors_render(ctx),
            false => self.folder_picker(ctx),
        }
    }
}

impl MainWindow {
    fn pick_new_folder(&mut self) {
        match FileDialog::new().pick_folder() {
            Some(new_folder) => {
                self.editors = Some(Box::new(Editors::new(new_folder)));
            }
            None => {}
        };
    }

    fn top_panel(&mut self, ctx: &Context) {
        TopBottomPanel::top("top_panel")
            .resizable(false)
            .exact_height(30.0)
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.heading("W2.Rust Editors");

                    match &self.editors {
                        Some(editors) => {
                            if ui
                                .link(editors.get_client_folder().display().to_string())
                                .clicked()
                            {
                                self.pick_new_folder();
                            }
                        }

                        None => {}
                    }

                    ui.allocate_ui_with_layout(
                        ui.available_size(),
                        Layout::right_to_left(Align::Center),
                        |ui| {
                            ui.hyperlink_to(
                                format!("Versão: {}", env!("CARGO_PKG_VERSION")),
                                "https://github.com/Rechdan/W2.Rust/releases/latest",
                            );
                        },
                    );
                });
            });
    }

    fn folder_picker(&mut self, ctx: &Context) {
        self.top_panel(ctx);

        CentralPanel::default().show(ctx, |ui| {
            if ui.button("Selecionar pasta do cliente").clicked() {
                self.pick_new_folder();
            }
        });
    }

    fn editors_render(&mut self, ctx: &Context) {
        self.top_panel(ctx);
        self.editors.as_mut().unwrap().render(ctx);
    }
}
