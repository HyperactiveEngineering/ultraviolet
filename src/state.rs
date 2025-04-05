use crate::{
    route::{Home, Route},
    task_button::{Button, ButtonState},
    task_store::action,
};

#[derive(Clone)]
pub struct State {
    pub battery_voltage: f32,
    pub route: Route,
}

impl State {
    pub const fn new() -> Self {
        Self {
            battery_voltage: 0.0,
            route: Route::Home(Home::Statistics),
        }
    }
}

action!(
    BatteryVoltage handle_battery_voltage { voltage: f32 },
    ButtonPressed handle_button_pressed { button: Button, button_state: ButtonState },
);
