use egui::{Frame, Margin, Ui};

use crate::consts::SPACE;

// settings

#[derive(Clone)]
pub struct BodyContainerSettings {
    fill_height: bool,
}

impl Default for BodyContainerSettings {
    fn default() -> Self {
        Self { fill_height: true }
    }
}

impl BodyContainerSettings {
    pub fn set_fill_height(&self, fill_height: bool) -> Self {
        Self {
            fill_height,
            ..self.clone()
        }
    }
}

// function

pub fn bordered_container(
    ui: &mut Ui,
    settings: impl FnOnce(BodyContainerSettings) -> BodyContainerSettings,
    add_contents: impl FnOnce(&mut Ui) -> (),
) {
    let settings = settings(BodyContainerSettings::default());

    Frame::default()
        .fill(ui.style().visuals.code_bg_color)
        .rounding(ui.style().visuals.window_rounding)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());

            if settings.fill_height {
                ui.set_min_height(ui.available_height());
            }

            Frame::default()
                .fill(ui.style().visuals.window_fill)
                .rounding(ui.style().visuals.window_rounding)
                .outer_margin(Margin::same(1.0))
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());

                    if settings.fill_height {
                        ui.set_min_height(ui.available_height());
                    }

                    Frame::default()
                        .outer_margin(Margin::same(SPACE))
                        .show(ui, |ui| {
                            add_contents(ui);
                        });
                });
        });
}
