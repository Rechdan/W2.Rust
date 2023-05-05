use egui::RichText;

pub fn monospaced(text: String) -> RichText {
    RichText::from(text).monospace()
}
