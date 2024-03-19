use iced::{Background, Border, Color, color, Shadow, Vector};

use iced::widget::button;
use iced::theme::Theme;

#[derive(Default)]
pub enum BColor {
    #[default]
    Bright,
    Dark,
    HighLight,
}

#[derive(Clone)]
pub enum BTColor {
    White,
    Black,
}

pub struct Bbutton {
    pub background : BColor,
    pub text_color: BTColor,
}


impl button::StyleSheet for Bbutton {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {

        let background: Option<Background> = match self.background {
            BColor::Bright      => { Some(Background::Color(color!(0xf4, 0xdf, 0xc1))) },
            BColor::Dark        => { Some(Background::Color(color!(0xb6, 0x87, 0x6b))) },
            BColor::HighLight   => { Some(Background::Color(color!(0xFF, 0xFF, 0x0))) },
        };

        let text_color: Color = match self.text_color {
            BTColor::Black  => { color!(0x0, 0x0, 0x0) },
            BTColor::White  => { color!(0xFF, 0xFF, 0xFF) },
        };

        button::Appearance {
            shadow_offset: Vector::default(),
            background ,
            text_color,
            border: Border::with_radius(0),
            shadow: Shadow::default(),
        }

    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        let background: Option<Background> = match self.background {
            BColor::Bright      => { Some(Background::Color(color!(0xf4, 0xdf, 0xc1, 0.6))) },
            BColor::Dark        => { Some(Background::Color(color!(0xb6, 0x87, 0x6b, 0.6))) },
            BColor::HighLight   => { Some(Background::Color(color!(0xcc, 0xc0, 0xb4, 0.6))) },
        };

        let text_color: Color = match self.text_color {
            BTColor::Black  => { color!(0x0, 0x0, 0x0) },
            BTColor::White  => { color!(0xFF, 0xFF, 0xFF) },
        };

        button::Appearance {
            shadow_offset: Vector::default(),
            background ,
            text_color,
            border: Border::with_radius(0),
            shadow: Shadow::default(),
        }
    }

    // fn pressed(&self, style: &Self::Style) -> button::Appearance {
    //     if let Button::Custom(custom) = style {
    //         return custom.pressed(self);
    //     }

    //     button::Appearance {
    //         shadow_offset: Vector::default(),
    //         ..self.active(style)
    //     }
    // }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        self.active(style)
    }
}
