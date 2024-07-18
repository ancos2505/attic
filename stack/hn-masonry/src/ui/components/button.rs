use accesskit::{DefaultActionVerb, Role};
use masonry::{
    widget::{SizedBox, WidgetRef},
    AccessCtx, AccessEvent, Action, BoxConstraints, Color, EventCtx, LayoutCtx, LifeCycle,
    LifeCycleCtx, PaintCtx, Point, PointerEvent, Size, StatusChange, TextEvent, Widget, WidgetPod,
};
use smallvec::smallvec;
use smallvec::SmallVec;
use std::sync::Arc;
use tracing::{trace, trace_span, Span};
use vello::Scene;

pub(crate) struct CalcButton {
    inner: WidgetPod<SizedBox>,
    action: CalcAction,
    base_color: Color,
    active_color: Color,
}

#[derive(Clone, Copy)]
pub(crate) enum CalcAction {
    Digit(u8),
    Op(char),
}

impl CalcButton {
    pub(crate) fn new(
        inner: SizedBox,
        action: CalcAction,
        base_color: Color,
        active_color: Color,
    ) -> Self {
        Self {
            inner: WidgetPod::new(inner),
            action,
            base_color,
            active_color,
        }
    }
}

impl Widget for CalcButton {
    fn on_pointer_event(&mut self, ctx: &mut EventCtx, event: &PointerEvent) {
        match event {
            PointerEvent::PointerDown(_, _) => {
                if !ctx.is_disabled() {
                    ctx.get_mut(&mut self.inner)
                        .set_background(self.active_color);
                    ctx.set_active(true);
                    ctx.request_paint();
                    trace!("CalcButton {:?} pressed", ctx.widget_id());
                }
            }
            PointerEvent::PointerUp(_, _) => {
                if ctx.is_active() && !ctx.is_disabled() {
                    ctx.submit_action(Action::Other(Arc::new(self.action)));
                    ctx.request_paint();
                    trace!("CalcButton {:?} released", ctx.widget_id());
                }
                ctx.get_mut(&mut self.inner).set_background(self.base_color);
                ctx.set_active(false);
            }
            _ => (),
        }
        self.inner.on_pointer_event(ctx, event);
    }

    fn on_text_event(&mut self, ctx: &mut EventCtx, event: &TextEvent) {
        self.inner.on_text_event(ctx, event);
    }

    fn on_access_event(&mut self, ctx: &mut EventCtx, event: &AccessEvent) {
        if event.target == ctx.widget_id() {
            match event.action {
                accesskit::Action::Default => {
                    ctx.submit_action(Action::Other(Arc::new(self.action)));
                    ctx.request_paint();
                }
                _ => {}
            }
        }
        ctx.skip_child(&mut self.inner);
    }

    fn on_status_change(&mut self, ctx: &mut LifeCycleCtx, event: &StatusChange) {
        match event {
            StatusChange::HotChanged(true) => {
                ctx.get_mut(&mut self.inner).set_border(Color::WHITE, 3.0);
                ctx.request_paint();
            }
            StatusChange::HotChanged(false) => {
                ctx.get_mut(&mut self.inner)
                    .set_border(Color::TRANSPARENT, 3.0);
                ctx.request_paint();
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle) {
        self.inner.lifecycle(ctx, event);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints) -> Size {
        let size = self.inner.layout(ctx, bc);
        ctx.place_child(&mut self.inner, Point::ORIGIN);

        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, scene: &mut Scene) {
        self.inner.paint(ctx, scene);
    }

    fn accessibility_role(&self) -> Role {
        Role::Button
    }

    fn accessibility(&mut self, ctx: &mut AccessCtx) {
        let _name = match self.action {
            CalcAction::Digit(digit) => digit.to_string(),
            CalcAction::Op(op) => op.to_string(),
        };
        // We may want to add a name if it doesn't interfere with the child label
        // ctx.current_node().set_name(name);
        ctx.current_node()
            .set_default_action_verb(DefaultActionVerb::Click);

        self.inner.accessibility(ctx);
    }

    fn children(&self) -> SmallVec<[WidgetRef<'_, dyn Widget>; 16]> {
        smallvec![self.inner.as_dyn()]
    }

    fn make_trace_span(&self) -> Span {
        trace_span!("CalcButton")
    }
}
