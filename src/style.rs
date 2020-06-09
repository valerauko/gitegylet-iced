use iced::{checkbox, container};
use iced::{Background, Color};

pub struct Window;

impl container::StyleSheet for Window {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb8(0x1a, 0x1d, 0x21))),
            text_color: Some(Color::from_rgb8(0xc6, 0xc1, 0xa7)),
            ..container::Style::default()
        }
    }
}

pub enum BranchCheckbox {
    Head,
    Normal,
}

impl checkbox::StyleSheet for BranchCheckbox {
    fn active(&self, _selected: bool) -> checkbox::Style {
        let base = checkbox::Style {
            background: Background::Color(Color::from_rgba8(0x0, 0x0, 0x0, 0.0)),
            checkmark_color: Color::from_rgb8(0xc6, 0xc1, 0xa7),
            border_radius: 0,
            border_width: 0,
            border_color: Color::BLACK,
        };
        match self {
            BranchCheckbox::Head => checkbox::Style {
                checkmark_color: Color::from_rgb8(0x1a, 0x1d, 0x21),
                ..base
            },
            BranchCheckbox::Normal => base,
        }
    }

    fn hovered(&self, selected: bool) -> checkbox::Style {
        self.active(selected)
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
                background: Some(Background::Color(Color::from_rgb8(0xff, 0xc8, 0x06))),
                text_color: Some(Color::from_rgb8(0x1a, 0x1d, 0x21)),
                ..container::Style::default()
            },
            Branch::UnselectedHead => container::Style {
                background: Some(Background::Color(Color::from_rgb8(0x48, 0x3f, 0x1c))),
                ..container::Style::default()
            },
            Branch::Normal => container::Style::default(),
        }
    }
}
