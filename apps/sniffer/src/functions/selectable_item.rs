use egui::{vec2, Response, RichText, Sense, TextStyle, Ui, Vec2, WidgetText};

use crate::consts::SPACE;

// item

#[derive(Clone)]
pub struct SelectableItem {
    items: Box<Vec<RichText>>,
}

impl Default for SelectableItem {
    fn default() -> Self {
        Self {
            items: Box::new(Vec::new()),
        }
    }
}

impl SelectableItem {
    pub fn append(&self, value: String, rt: impl FnOnce(RichText) -> RichText) -> Self {
        let mut items = self.items.clone();
        items.push(rt(RichText::from(value)));
        Self {
            items,
            ..self.clone()
        }
    }
}

// settings

#[derive(Clone)]
pub struct SelectableItemSettings {
    fill_width: bool,
    padding: Vec2,
}

impl Default for SelectableItemSettings {
    fn default() -> Self {
        Self {
            fill_width: true,
            padding: vec2(SPACE, SPACE),
        }
    }
}

impl SelectableItemSettings {
    pub fn set_fill_width(&self, fill_width: bool) -> Self {
        Self {
            fill_width,
            ..self.clone()
        }
    }

    pub fn set_padding(&self, padding: Vec2) -> Self {
        Self {
            padding,
            ..self.clone()
        }
    }
}

// function

pub fn selectable_item(
    ui: &mut Ui,
    selected: bool,
    settings: impl FnOnce(SelectableItemSettings) -> SelectableItemSettings,
    frame_content: impl FnOnce(SelectableItem) -> SelectableItem,
) -> Response {
    let settings = settings(SelectableItemSettings::default());

    let button_padding = settings.padding;
    let total_extra = button_padding + button_padding;

    let mut total_width = ui.available_width();
    let content_width = total_width - total_extra.x;

    let selectable_item = frame_content(SelectableItem::default());

    let content = selectable_item
        .items
        .iter()
        .map(|i| {
            WidgetText::RichText(i.clone()).into_galley(ui, None, content_width, TextStyle::Button)
        })
        .collect::<Vec<_>>();

    if !settings.fill_width {
        let mut max_width = 0f32;

        content
            .iter()
            .for_each(|c| max_width = max_width.max(c.size().x));

        total_width = max_width + total_extra.x;
    }

    let content_len = content.len();

    let mut content_height = 0f32;
    content.iter().for_each(|i| content_height += i.size().y);
    if content_len > 0 {
        content_height += ((content_len as f32) - 1.0) * SPACE;
    }

    let total_height = total_extra.y + content_height;

    let (rect, response) =
        ui.allocate_at_least(vec2(total_width, total_height), Sense::click_and_drag());

    if ui.is_rect_visible(response.rect) {
        let visuals = ui.style().interact_selectable(&response, selected);

        if selected || response.hovered() || response.highlighted() || response.has_focus() {
            let rect = rect.expand(visuals.expansion);

            ui.painter().rect(
                rect,
                visuals.rounding,
                visuals.weak_bg_fill,
                visuals.bg_stroke,
            );
        }

        let mut text_pos = ui
            .layout()
            .align_size_within_rect(rect.size(), rect.shrink2(button_padding))
            .min;

        text_pos += button_padding;

        content.iter().for_each(|item| {
            // item.clone().paint_with_visuals(ui.painter(), text_pos, &visuals);

            ui.painter()
                .galley(text_pos, item.clone(), visuals.text_color());

            text_pos.y += item.clone().size().y + SPACE;
        });
    }

    response
}
