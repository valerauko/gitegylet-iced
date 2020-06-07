use iced::container;
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

impl container::StyleSheet for Commit {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb8(0x1e, 0x1e, 0x1e))),
            text_color: Some(Color::from_rgba8(0xff, 0xff, 0xff, 0.6)),
            ..container::Style::default()
        }
    }
}
