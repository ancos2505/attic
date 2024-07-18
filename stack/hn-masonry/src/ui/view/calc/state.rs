use std::cell::RefCell;

use masonry::app_driver::{AppDriver, DriverCtx};
use masonry::widget::{Flex, Label, RootWidget, SizedBox};
use masonry::{Action, WidgetId};

use crate::ui::components::button::CalcAction;

#[derive(Clone)]
pub(crate) struct CalcState {
    /// The number displayed. Generally a valid float.
    pub value: String,
    pub operand: f64,
    pub operator: char,
    pub in_num: bool,
}

// ---

impl CalcState {
    fn digit(&mut self, digit: u8) {
        if !self.in_num {
            self.value.clear();
            self.in_num = true;
        }
        let ch = (b'0' + digit) as char;
        self.value.push(ch);
    }

    fn display(&mut self) {
        self.value = self.operand.to_string();
    }

    fn compute(&mut self) {
        if self.in_num {
            let operand2 = self.value.parse().unwrap_or(0.0);
            let result = match self.operator {
                '+' => Some(self.operand + operand2),
                '−' => Some(self.operand - operand2),
                '×' => Some(self.operand * operand2),
                '÷' => Some(self.operand / operand2),
                _ => None,
            };
            if let Some(result) = result {
                self.operand = result;
                self.display();
                self.in_num = false;
            }
        }
    }

    fn op(&mut self, op: char) {
        match op {
            '+' | '−' | '×' | '÷' | '=' => {
                self.compute();
                self.operand = self.value.parse().unwrap_or(0.0);
                self.operator = op;
                self.in_num = false;
            }
            '±' => {
                if self.in_num {
                    if self.value.starts_with('−') {
                        self.value = self.value[3..].to_string();
                    } else {
                        self.value = ["−", &self.value].concat();
                    }
                } else {
                    self.operand = -self.operand;
                    self.display();
                }
            }
            '.' => {
                if !self.in_num {
                    self.value = "0".to_string();
                    self.in_num = true;
                }
                if self.value.find('.').is_none() {
                    self.value.push('.');
                }
            }
            'c' => {
                self.value = "0".to_string();
                self.in_num = false;
            }
            'C' => {
                self.value = "0".to_string();
                self.operator = 'C';
                self.in_num = false;
            }
            'b' => {
                if self.in_num {
                    self.value.pop();
                    if self.value.is_empty() || self.value == "−" {
                        self.value = "0".to_string();
                        self.in_num = false;
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}

impl AppDriver for CalcState {
    fn on_action(&mut self, ctx: &mut DriverCtx<'_>, _widget_id: WidgetId, action: Action) {
        match action {
            Action::Other(payload) => {
                tracing::debug!("ANCOS :{payload:?}");
                match payload.downcast_ref::<CalcAction>().unwrap() {
                    CalcAction::Digit(digit) => self.digit(*digit),
                    CalcAction::Op(op) => self.op(*op),
                }
            }
            _ => unreachable!(),
        }

        update_calc_display(ctx, &*self.value).unwrap();
    }
}

// ---
fn update_calc_display(
    ctx: &mut DriverCtx<'_>,
    new_text: impl Into<masonry::ArcStr>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut root_widget = ctx.get_root::<RootWidget<Flex>>();

    let mut flex = root_widget.get_element();

    let mut child_widget = flex.child_mut(1).ok_or(anyhow::anyhow!("Element not found!"))?;

    let mut label = child_widget.downcast::<Label>();

    label.set_text(new_text);

    Ok(())
}
