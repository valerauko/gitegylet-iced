use iced::{checkbox, container};
use iced::{Background, Color};

pub struct Container;

impl container::StyleSheet for Container {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb8(0x12, 0x12, 0x12))),
            text_color: Some(Color::from_rgba8(0xff, 0xff, 0xff, 0.6)),
            ..container::Style::default()
        }
    }
}

pub struct Branch;

impl checkbox::StyleSheet for Branch {
    fn active(&self, selected: bool) -> checkbox::Style {
        self.hovered(selected)
    }

    fn hovered(&self, _selected: bool) -> checkbox::Style {
        checkbox::Style {
            background: Background::Color(Color::from_rgb8(0x1e, 0x1e, 0x1e)),
            checkmark_color: Color::from_rgba8(0xff, 0xff, 0xff, 0.4),
            border_radius: 0,
            border_width: 0,
            border_color: Color::BLACK,
        }
    }
}
