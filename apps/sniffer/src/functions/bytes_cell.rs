use egui::{Color32, NumExt, Response, RichText, Sense, Stroke, TextStyle, Ui, WidgetText};

pub fn byte_cell(ui: &mut Ui, selected: bool, value: String) -> Response {
    let value = WidgetText::from(RichText::new(value).monospace());

    let button_padding = ui.spacing().button_padding;
    let total_extra = button_padding + button_padding;

    let wrap_width = ui.available_width() - total_extra.x;
    let text = value.into_galley(ui, None, wrap_width, TextStyle::Button);

    let mut desired_size = ui.available_size();

    desired_size.y = desired_size.y.at_least(ui.spacing().interact_size.y);

    let (rect, response) = ui.allocate_at_least(desired_size, Sense::click());

    if ui.is_rect_visible(response.rect) {
        let mut text_pos = ui
            .layout()
            .align_size_within_rect(text.size(), rect.shrink2(button_padding));

        text_pos.set_center(rect.center());

        let text_pos = text_pos.min;

        let mut visuals = ui.style().interact_selectable(&response, selected);

        visuals.bg_stroke = Stroke {
            color: Color32::DARK_GRAY,
            ..visuals.bg_stroke
        };

        if selected || response.hovered() || response.highlighted() || response.has_focus() {
            let rect = rect.expand(visuals.expansion);

            ui.painter().rect(
                rect,
                visuals.rounding,
                visuals.weak_bg_fill,
                visuals.bg_stroke,
            );
        }

        ui.painter().galley(text_pos, text, visuals.text_color());

        // text.paint_with_visuals(ui.painter(), text_pos, &visuals);
    }

    response
}
