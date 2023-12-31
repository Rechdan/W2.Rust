use eframe::{App, Frame};
use egui::{Button, CentralPanel, Context, SidePanel, TopBottomPanel, Ui, Window};
use rfd::FileDialog;
use std::path::PathBuf;

use crate::editors::{server_list::ServerListEditor, server_name::ServerNameEditor, EditorRender};

#[derive(Default)]
pub struct MainWindow {
    client_folder: Option<PathBuf>,

    server_list: Option<(bool, Box<ServerListEditor>)>,
    server_name: Option<(bool, Box<ServerNameEditor>)>,
}

impl App for MainWindow {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        match self.client_folder.clone() {
            Some(folder) => self.main_view(ctx, folder),
            None => self.folder_picker(ctx),
        }
    }
}

impl MainWindow {
    fn pick_new_folder(&mut self) {
        match FileDialog::new().pick_folder() {
            Some(new_folder) => {
                self.client_folder = Some(new_folder);
            }
            None => {}
        };
    }

    fn folder_picker(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("W2.Rust Editors");

            if ui.button("Selecionar pasta do cliente").clicked() {
                self.pick_new_folder();
            }
        });
    }

    fn main_view(&mut self, ctx: &Context, folder: PathBuf) {
        TopBottomPanel::top("top_panel")
            .resizable(false)
            .exact_height(30.0)
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.heading("W2.Rust Editors");
                });
            });

        SidePanel::left("left_panel")
            .resizable(false)
            .exact_width(150.0)
            .show(ctx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    Self::manage_editor_btn(&mut self.server_list, ui, folder.clone());
                    Self::manage_editor_btn(&mut self.server_name, ui, folder.clone());
                });
            });

        Self::manage_window(&mut self.server_list, ctx);
        Self::manage_window(&mut self.server_name, ctx);
    }

    fn manage_window<T: EditorRender>(editor: &mut Option<(bool, Box<T>)>, ctx: &Context) {
        match editor {
            Some((open, editor)) => {
                Window::new(T::name())
                    .resizable(false)
                    .open(open)
                    .show(ctx, |ui| {
                        editor.render(ui);
                    });
            }
            None => {}
        };
    }

    fn manage_editor_btn<T: EditorRender>(
        editor: &mut Option<(bool, Box<T>)>,
        ui: &mut Ui,
        folder: PathBuf,
    ) {
        if ui
            .add(Button::new(T::name()).selected(editor.as_ref().is_some_and(|(open, _)| *open)))
            .clicked()
        {
            match editor {
                Some((open, _)) => *open = !*open,
                None => match T::new(folder) {
                    Some(new_editor) => *editor = Some((true, new_editor)),
                    None => {}
                },
            };
        }
    }
}
