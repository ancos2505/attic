mod repo;
mod ui;

use ui::footer::{render_footer_region, AppData};
use vizia::model::Model;
use vizia::{
    context::Context,
    layout::Units::{Pixels, Stretch},
    modifiers::{LayoutModifiers, WindowModifiers},
    views::VStack,
    Application,
};

use crate::ui::{content::render_content_region, header::render_header_region};

type AppResult<T> = anyhow::Result<T>;

fn main() -> AppResult<()> {
    Application::new(|cx| {
        AppData::default().build(cx);
        VStack::new(cx, |cx| {
            render_main(cx);
        })
        .child_left(Stretch(1.0))
        .child_right(Stretch(1.0));
    })
    .title("hn-vizia")
    .inner_size((970, 700))
    .run()
    .map_err(|err| anyhow::format_err!("{err:?}"))?;
    Ok(())
}

fn render_main(cx: &mut Context) {
    VStack::new(cx, |cx| {
        // Spacer
        VStack::new(cx, |_| ()).height(Pixels(10.));

        render_header_region(cx);

        render_content_region(cx);

        render_footer_region(cx);

        /////////////////////////////////////////////////////////////////
    })
    .width(Pixels(800.));
}
