use egui::{Button, ComboBox, Context, Frame, SidePanel, Ui, Window};
use std::{fs, path::PathBuf};

use crate::consts::LANGS_FOLDER;

use self::{server_list::ServerListEditor, server_name::ServerNameEditor, strdef::StrDefEditor};

pub mod server_list;
pub mod server_name;
pub mod strdef;

pub trait EditorRender {
    fn name() -> &'static str;
    fn new(folder: PathBuf) -> Option<Box<Self>>;
    fn render(&mut self, ui: &mut Ui);
}

type LangFolders = Vec<(String, PathBuf)>;

pub struct Editors {
    client_folder: PathBuf,

    lang_folders: LangFolders,
    selected_lang_folder: PathBuf,

    server_list: Option<(bool, Box<ServerListEditor>)>,
    server_name: Option<(bool, Box<ServerNameEditor>)>,
    strdef: Option<(bool, Box<StrDefEditor>)>,
}

impl Editors {
    pub fn new(client_folder: PathBuf) -> Self {
        let lang_folder = client_folder.join(LANGS_FOLDER);

        let lang_folders: LangFolders = fs::read_dir(lang_folder)
            .unwrap()
            .take_while(|f| {
                f.as_ref()
                    .is_ok_and(|f| f.file_type().is_ok_and(|f| f.is_dir()))
            })
            .map(|f| f.unwrap().path())
            .map(|f| (f.file_name().unwrap().to_str().unwrap().to_string(), f))
            .collect();

        Self {
            client_folder,
            selected_lang_folder: lang_folders.get(0).unwrap().1.clone(),
            lang_folders,
            server_list: Default::default(),
            server_name: Default::default(),
            strdef: Default::default(),
        }
    }

    pub fn get_client_folder(&self) -> PathBuf {
        self.client_folder.clone()
    }

    fn clear_editors(&mut self) {
        Self::clear_editor(&mut self.server_list);
        Self::clear_editor(&mut self.server_name);
        Self::clear_editor(&mut self.strdef);
    }

    fn clear_editor<T: EditorRender>(editor: &mut Option<(bool, Box<T>)>) {
        *editor = None;
    }

    pub fn render(&mut self, ctx: &Context) {
        self.lef_panel(ctx);

        Self::manage_window(&mut self.server_list, ctx);
        Self::manage_window(&mut self.server_name, ctx);
        Self::manage_window(&mut self.strdef, ctx);
    }

    fn lef_panel(&mut self, ctx: &Context) {
        SidePanel::left("left_panel")
            .resizable(false)
            .exact_width(150.0)
            .show(ctx, |ui| {
                ui.group(|ui| {
                    ui.label("Configurações rápidas");
                    ui.separator();
                    self.manage_lang_folder(ui);
                });

                ui.group(|ui| {
                    ui.label("Editores");
                    ui.vertical_centered_justified(|ui| {
                        Self::manage_editor_btn(
                            &mut self.server_list,
                            ui,
                            self.client_folder.clone(),
                        );

                        Self::manage_editor_btn(
                            &mut self.server_name,
                            ui,
                            self.client_folder.clone(),
                        );

                        Self::manage_editor_btn(
                            &mut self.strdef,
                            ui,
                            self.selected_lang_folder.clone(),
                        );
                    });
                });
            });
    }

    fn manage_lang_folder(&mut self, ui: &mut Ui) {
        ui.label("Selecione o idioma");

        let selected_index = &mut self
            .lang_folders
            .iter()
            .position(|(_, p)| *p == *self.selected_lang_folder)
            .unwrap();

        if ComboBox::from_id_source("lang_folder_combobox")
            .width(ui.available_width())
            .show_index(ui, selected_index, self.lang_folders.len(), |i| {
                self.lang_folders[i].0.to_string()
            })
            .changed()
        {
            self.selected_lang_folder = self.lang_folders[*selected_index].1.clone();
            self.clear_editors();
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

    fn manage_window<T: EditorRender>(editor: &mut Option<(bool, Box<T>)>, ctx: &Context) {
        match editor {
            Some((open, editor)) => {
                Window::new(T::name())
                    .resizable(false)
                    .auto_sized()
                    .open(open)
                    .show(ctx, |ui| {
                        Frame::none().show(ui, |ui| {
                            editor.render(ui);
                        });
                    });
            }
            None => {}
        };
    }
}
