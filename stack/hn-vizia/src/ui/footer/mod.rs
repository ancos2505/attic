// use vizia::{
//     binding::{
//         Lens,
//         LensValue,
//         // Map, MapRef, Res, ResGet, StaticLens, Then, UnwrapLens,
//         Wrapper,
//     },
//     context::{Context, EmitContext, EventContext},
//     events::Event,
//     layout::Units::{Pixels, Stretch},
//     model::Model,
//     modifiers::{
//         LayoutModifiers,
//         StyleModifiers,
//         // WindowModifiers
//     },
//     views::{Button, HStack, Label, Textbox, VStack},
// };
// use vizia_derive::{
//     // Data,
//     Lens,
// };
use super::theme::{BACKGROUND_WHITE, BLUE, GREY, ORANGE, RED, WHITE};
use vizia::prelude::*;

pub enum AppEvent {
    SetTitle(String),
}

#[derive(Default, Lens)]
pub struct AppData {
    pub title: String,
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetTitle(title) => self.title = title.clone(),
        })
    }
}

pub(crate) fn render_footer_region(cx: &mut Context) {
    VStack::new(cx, |cx| {
        VStack::new(cx, |_| ())
            .background_color(ORANGE)
            .height(Pixels(3.))
            .max_width(Pixels(800.));
        VStack::new(cx, |cx| {
            render_menu(cx);
            render_search(cx);
        })
        .background_color(BACKGROUND_WHITE)
        .height(Pixels(120.))
        .max_width(Pixels(800.));
        VStack::new(cx, |_| ())
            .background_color(WHITE)
            .height(Pixels(12.))
            .max_width(Pixels(800.));
    })
    .background_color(RED)
    .height(Pixels(135.))
    .max_width(Pixels(800.));
}

fn render_menu(cx: &mut Context) {
    HStack::new(cx, |cx| {
        Button::new(cx, |cx| Label::new(cx, "Guidelines"))
            .height(Pixels(18.0))
            .border_color(BACKGROUND_WHITE)
            .background_color(BACKGROUND_WHITE);
        Label::new(cx, "|");
        Button::new(cx, |cx| Label::new(cx, "FAQ"))
            .height(Pixels(18.0))
            .border_color(BACKGROUND_WHITE)
            .background_color(BACKGROUND_WHITE);
        Label::new(cx, "|");
        Button::new(cx, |cx| Label::new(cx, "Lists"))
            .height(Pixels(18.0))
            .border_color(BACKGROUND_WHITE)
            .background_color(BACKGROUND_WHITE);
        Label::new(cx, "|");
        Button::new(cx, |cx| Label::new(cx, "API"))
            .height(Pixels(18.0))
            .border_color(BACKGROUND_WHITE)
            .background_color(BACKGROUND_WHITE);
        Label::new(cx, "|");
        Button::new(cx, |cx| Label::new(cx, "Security"))
            .height(Pixels(18.0))
            .border_color(BACKGROUND_WHITE)
            .background_color(BACKGROUND_WHITE);
        Label::new(cx, "|");
        Button::new(cx, |cx| Label::new(cx, "Legal"))
            .height(Pixels(18.0))
            .border_color(BACKGROUND_WHITE)
            .background_color(BACKGROUND_WHITE);
        Label::new(cx, "|");
        Button::new(cx, |cx| Label::new(cx, "Apply to YC"))
            .height(Pixels(18.0))
            .border_color(BACKGROUND_WHITE)
            .background_color(BACKGROUND_WHITE);
        Label::new(cx, "|");
        Button::new(cx, |cx| Label::new(cx, "Contact"))
            .height(Pixels(18.0))
            .border_color(BACKGROUND_WHITE)
            .background_color(BACKGROUND_WHITE);
    })
    .width(Pixels(800.))
    .child_space(Stretch(1.));
}
fn render_search(cx: &mut Context) {
    HStack::new(cx, |cx| {
        Label::new(cx, "Search: ").color(GREY);
        Textbox::new(cx, AppData::title)
            .on_edit(|ex, txt| ex.emit(AppEvent::SetTitle(txt)))
            .width(Stretch(1.0))
            .border_color(GREY);
    })
    .width(Pixels(800.))
    .child_space(Stretch(1.));
}
