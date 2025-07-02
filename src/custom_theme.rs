use iced::{Background, Border, Color, color, Shadow };
use iced::widget::button;

pub struct ChessColor;
impl ChessColor {
    pub fn highlight_bt_active()    -> Color { color!(0xFF, 0xFF, 0x0) }
    pub fn highlight_bt_hovered()   -> Color { color!(0xcc, 0xc0, 0xb4, 0.6) }

    pub fn dark_bt_active()         -> Color { color!(0xb6, 0x87, 0x6b) }
    pub fn dark_bt_hovered()        -> Color { color!(0xb6, 0x87, 0x6b, 0.6) }

    pub fn bright_bt_active()       -> Color { color!(0xf4, 0xdf, 0xc1) }
    pub fn bright_bt_hovered()      -> Color { color!(0xf4, 0xdf, 0xc1, 0.6) }
}

pub struct ChessStyle;
impl ChessStyle {
    pub fn hightlighted_button_wrapper(status: iced::widget::button::Status) -> button::Style {
        button::Style {
            background: Some(Background::Color(
                match status {
                    button::Status::Active => ChessColor::highlight_bt_active(),
                    button::Status::Hovered => ChessColor::highlight_bt_hovered(),
                    _ => ChessColor::highlight_bt_active()
                }
            )),
            text_color: Color::default(),
            border: Border::default().rounded(0),
            shadow: Shadow::default()
        }
    }

    pub fn dark_button_wrapper(status: iced::widget::button::Status) -> button::Style {
        button::Style {
            background: Some(Background::Color(
                match status {
                    button::Status::Active   => ChessColor::dark_bt_active(),
                    button::Status::Hovered  => ChessColor::dark_bt_hovered(),
                    _ => ChessColor::dark_bt_active()
                }
            )),
            text_color: Color::default(),
            border: Border::default().rounded(0),
            shadow: Shadow::default()
        }
    }

    pub fn bright_button_wrapper(status: iced::widget::button::Status) -> button::Style {
        button::Style {
            background: Some(Background::Color(
                match status {
                    button::Status::Active => ChessColor::bright_bt_active(),
                    button::Status::Hovered => ChessColor::bright_bt_hovered(),
                    _ => ChessColor::bright_bt_active()
                }
            )),
            text_color: Color::default(),
            border: Border::default().rounded(0),
            shadow: Shadow::default()
        }
    }
}