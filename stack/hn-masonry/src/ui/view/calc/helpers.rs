use masonry::theme::BACKGROUND_LIGHT;
use masonry::widget::{Align, Flex, Label, SizedBox};
use masonry::{Color, Widget};

use crate::ui::components::button::{CalcAction, CalcButton};
use crate::ui::view::{BACKGROUND_WHITE, BLACK, DARK_GREY, GREY, ORANGE};

pub fn op_button_with_label(op: char, label: String) -> CalcButton {
    const BLUE: Color = Color::rgb8(0x00, 0x8d, 0xdd);
    const LIGHT_BLUE: Color = Color::rgb8(0x5c, 0xc4, 0xff);

    CalcButton::new(
        SizedBox::new(Align::centered(Label::new(label).with_text_size(24.)))
            .background(BLUE)
            .expand(),
        CalcAction::Op(op),
        BLUE,
        LIGHT_BLUE,
    )
}

pub fn op_button(op: char) -> CalcButton {
    op_button_with_label(op, op.to_string())
}

pub fn digit_button(digit: u8) -> CalcButton {
    // const BG: Color = BACKGROUND_LIGHT;
    const BG_COLOR: Color = ORANGE;
    const BASE_COLOR: Color = DARK_GREY;
    const ACTIVE_COLOR: Color = GREY;
    const TEXT_COLOR: Color = BLACK;
    // const COLOR_BASE
    // const GRAY: Color = Color::rgb8(0x3a, 0x3a, 0x3a);
    // const LIGHT_GRAY: Color = Color::rgb8(0x71, 0x71, 0x71);
    CalcButton::new(
        SizedBox::new(Align::centered(
            Label::new(format!("{digit}"))
                .with_text_size(24.)
                .with_text_brush(TEXT_COLOR),
        ))
        .background(BG_COLOR)
        .expand(),
        CalcAction::Digit(digit),
        BASE_COLOR,
        ACTIVE_COLOR,
    )
}

pub fn flex_row(
    w1: impl Widget + 'static,
    w2: impl Widget + 'static,
    w3: impl Widget + 'static,
    w4: impl Widget + 'static,
) -> impl Widget {
    Flex::row()
        .with_flex_child(w1, 1.0)
        .with_spacer(1.0)
        .with_flex_child(w2, 1.0)
        .with_spacer(1.0)
        .with_flex_child(w3, 1.0)
        .with_spacer(1.0)
        .with_flex_child(w4, 1.0)
}
