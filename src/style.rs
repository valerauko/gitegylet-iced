use iced::{checkbox, container};
use iced::{Background, Color};

pub struct Window;

impl container::StyleSheet for Window {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb8(0x12, 0x12, 0x12))),
            text_color: Some(Color::from_rgba8(0xff, 0xff, 0xff, 0.6)),
            ..container::Style::default()
        }
    }
}

pub struct BranchCheckbox;

impl checkbox::StyleSheet for BranchCheckbox {
    fn active(&self, selected: bool) -> checkbox::Style {
        self.hovered(selected)
    }

    fn hovered(&self, _selected: bool) -> checkbox::Style {
        checkbox::Style {
            background: Background::Color(Color::from_rgba8(0x1e, 0x1e, 0x1e, 0.0)),
            checkmark_color: Color::from_rgba8(0xff, 0xff, 0xff, 0.4),
            border_radius: 0,
            border_width: 0,
            border_color: Color::BLACK,
        }
    }
}

pub enum Branch {
    Head,
    UnselectedHead,
    Normal,
}

impl container::StyleSheet for Branch {
    fn style(&self) -> container::Style {
        match self {
            Branch::Head => container::Style {
                background: Some(Background::Color(Color::from_rgb8(0x11, 0x64, 0xa3))),
                ..container::Style::default()
            },
            Branch::UnselectedHead => container::Style {
                background: Some(Background::Color(Color::from_rgb8(0x27, 0x24, 0x2c))),
                ..container::Style::default()
            },
            Branch::Normal => container::Style::default(),
        }
    }
}
