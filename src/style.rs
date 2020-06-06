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

pub struct Commit;

impl checkbox::StyleSheet for Commit {
    fn active(&self, is_checked: bool) -> checkbox::Style {
        self.hovered(is_checked)
    }

    fn hovered(&self, _: bool) -> checkbox::Style {
        checkbox::Style {
            background: Background::Color(Color::from_rgb8(0x1e, 0x1e, 0x1e)),
            ..checkbox::Style::default()
        }
    }
}
