use vizia::{
    context::Context,
    layout::Units::{Pixels, Stretch},
    modifiers::{LayoutModifiers, StyleModifiers, TextModifiers},
    style::FontWeightKeyword,
    views::{Button, HStack, Label, VStack},
};

use super::theme::{ORANGE, WHITE};

pub(crate) fn render_header_region(cx: &mut Context) {
    HStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            render_logo(cx);
            render_h_spacer(cx);
            render_h1(cx);
        })
        .background_color(ORANGE)
        .width(Pixels(180.));

        render_menu(cx);

        HStack::new(cx, |cx| {
            Button::new(cx, |cx| Label::new(cx, "login"))
                .height(Pixels(18.0))
                .border_color(ORANGE)
                .background_color(ORANGE);
        })
        .child_left(Stretch(1.))
        .child_top(Stretch(1.))
        .child_bottom(Stretch(1.));
    })
    .background_color(ORANGE)
    .height(Pixels(26.))
    .max_width(Pixels(800.));
}

fn render_menu(cx: &mut Context) {
    HStack::new(cx, |cx| {
        Button::new(cx, |cx| Label::new(cx, "new"))
            .height(Pixels(18.0))
            .border_color(ORANGE)
            .background_color(ORANGE);
        Label::new(cx, "|");
        Button::new(cx, |cx| Label::new(cx, "past"))
            .height(Pixels(18.0))
            .border_color(ORANGE)
            .background_color(ORANGE);
        Label::new(cx, "|");
        Button::new(cx, |cx| Label::new(cx, "comments"))
            .height(Pixels(18.0))
            .border_color(ORANGE)
            .background_color(ORANGE);
        Label::new(cx, "|");
        Button::new(cx, |cx| Label::new(cx, "ask"))
            .height(Pixels(18.0))
            .border_color(ORANGE)
            .background_color(ORANGE);
        Label::new(cx, "|");
        Button::new(cx, |cx| Label::new(cx, "show"))
            .height(Pixels(18.0))
            .border_color(ORANGE)
            .background_color(ORANGE);
        Label::new(cx, "|");
        Button::new(cx, |cx| Label::new(cx, "jobs"))
            .height(Pixels(18.0))
            .border_color(ORANGE)
            .background_color(ORANGE);
        Label::new(cx, "|");
        Button::new(cx, |cx| Label::new(cx, "submit"))
            .height(Pixels(18.0))
            .border_color(ORANGE)
            .background_color(ORANGE);
    })
    .width(Pixels(450.))
    .child_space(Stretch(1.));
}

fn render_h_spacer(cx: &mut Context) {
    HStack::new(cx, |_| ()).width(Pixels(5.));
}
fn render_h1(cx: &mut Context) {
    HStack::new(cx, |cx| {
        Label::new(cx, "Hacker News").font_weight(FontWeightKeyword::Bold);
    })
    .child_top(Stretch(1.))
    .child_bottom(Stretch(1.));
}

fn render_logo(cx: &mut Context) {
    VStack::new(cx, |cx| {
        VStack::new(cx, |cx| {
            Label::new(cx, " Y ")
                .background_color(ORANGE)
                .border_width(Pixels(1.))
                .border_color(WHITE)
                .font_size(18.)
                .color(WHITE)
                .width(Pixels(22.))
                .height(Pixels(22.))
                .child_space(Stretch(1.));
        })
        .border_width(Pixels(2.))
        .border_color(ORANGE);
    })
    .width(Pixels(26.));
}
