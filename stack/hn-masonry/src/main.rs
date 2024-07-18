// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]
#![allow(clippy::single_match)]

mod ui;

use masonry::dpi::LogicalSize;
use masonry::widget::RootWidget;
use crate::ui::view::calc::render_calc;
use winit::window::Window;

use crate::ui::view::{calc::state::CalcState, root::render_root_view};

pub fn main() {
    let window_size = LogicalSize::new(860., 660.);

    let window_attributes = Window::default_attributes()
        .with_title("Simple Calculator")
        .with_resizable(true)
        .with_min_inner_size(window_size);

    let calc_state = CalcState {
        value: "0".to_string(),
        operand: 0.0,
        operator: 'C',
        in_num: false,
    };

    masonry::event_loop_runner::run(
        masonry::event_loop_runner::EventLoop::with_user_event(),
        window_attributes,
        // RootWidget::new(render_root_view()),
        RootWidget::new(render_calc()),
        calc_state,
    )
    .unwrap();
}
