use egui::Ui;
use std::path::PathBuf;

pub mod server_list;
pub mod server_name;

pub trait EditorRender {
    fn name() -> &'static str;
    fn new(folder: PathBuf) -> Option<Box<Self>>;
    fn render(&mut self, ui: &mut Ui);
}

pub struct Editor {}
