use masonry::{
    widget::{
        // Align,
        Flex,
        Label,
        SizedBox,
    },
    Color, Widget,
};
use vello::wgpu::naga::LocalVariable;

// Hackernews Theme
const BLACK: Color = Color::rgb8(0x00, 0x00, 0x00);
const ORANGE: Color = Color::rgb8(0xff, 0x66, 0x00);
const DARK_GREY: Color = Color::rgb8(0x22, 0x22, 0x22);
const GREY: Color = Color::rgb8(0x99, 0x99, 0x99);
const BACKGROUND_WHITE: Color = Color::rgb8(0xf6, 0xf6, 0xef);

const WHITE: Color = Color::rgb8(0xff, 0xff, 0xff);

struct Theme {
    base_100: Color,
    default: Color,
    primary: Color,
    secondary: Color,
    neutral: Color,
}

// use super::{calc::build_calc, BACKGROUND_WHITE, BLACK, DARK_GREY, GREY};

// Windows 10 Theme
// const BLUE: Color = Color::rgb8(0x00, 0x8d, 0xdd);
// const LIGHT_BLUE: Color = Color::rgb8(0x5c, 0xc4, 0xff);
// const GRAY: Color = Color::rgb8(0x3a, 0x3a, 0x3a);
// const LIGHT_GRAY: Color = Color::rgb8(0x71, 0x71, 0x71);
const CURRENT_THEME: Theme = Theme {
    base_100: BACKGROUND_WHITE,
    default: GREY,
    primary: ORANGE,
    secondary: DARK_GREY,
    neutral: BLACK,
};

pub(crate) fn render_root_view() -> impl Widget {
    let theme = CURRENT_THEME;
    let root_widget = {
        let main_panel = {
            let label = Label::new("label".to_string())
                .with_text_size(32.0)
                .with_text_brush(theme.neutral);
            let panel = Flex::column()
                .with_flex_child(
                    SizedBox::new(Label::new("header".to_string()).with_text_brush(theme.neutral))
                        .background(theme.primary)
                        .expand_width(),
                    1.0,
                )
                .with_flex_child(
                    SizedBox::new(Label::new("content".to_string()).with_text_brush(theme.neutral))
                        .background(theme.base_100)
                        .expand_width()
                        .expand_height(),
                    1.0,
                )
                .with_spacer(2.)
                .with_flex_child(
                    SizedBox::new(Label::new("footer".to_string()).with_text_brush(theme.neutral))
                        .background(theme.base_100)
                        .height(100.),
                    1.0,
                );

            SizedBox::new(panel)
                // .border(theme.secondary, 2.)
                .background(theme.primary)
                // .width(800.)
                .expand_height()
        };

        let root_flex = Flex::column()
            .with_flex_child(
                SizedBox::new(Label::new("".to_string()).with_text_size(6.)),
                1.0,
            )
            .with_flex_child(main_panel, 1.0);

        SizedBox::new(root_flex).background(WHITE).expand()
    };
    root_widget
}
