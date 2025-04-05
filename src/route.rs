use crate::{
    state::State,
    task_button::{Button, ButtonState},
};

pub trait HandleButtonPressed {
    fn handle_button_pressed(&self, state: &mut State, button: Button, button_state: ButtonState);
}

#[derive(Clone, PartialEq, Eq)]
pub enum Home {
    ButtonDebugger,
    Statistics,
}

impl HandleButtonPressed for Home {
    fn handle_button_pressed(&self, state: &mut State, button: Button, button_state: ButtonState) {
        match self {
            Home::ButtonDebugger => {
                if button == Button::Select && button_state == ButtonState::Down {
                    state.route = Route::ButtonDebugger(ButtonDebugger(Default::default()))
                }
                if button == Button::Down && button_state == ButtonState::Down {
                    state.route = Route::Home(Home::Statistics)
                }
            }
            Home::Statistics => {
                if button == Button::Select && button_state == ButtonState::Down {
                    state.route = Route::Statistics(Statistics)
                }
                if button == Button::Up && button_state == ButtonState::Down {
                    state.route = Route::Home(Home::ButtonDebugger)
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct Statistics;

impl HandleButtonPressed for Statistics {
    fn handle_button_pressed(&self, state: &mut State, button: Button, button_state: ButtonState) {
        if button == Button::Back && button_state == ButtonState::Down {
            state.route = Route::Home(Home::Statistics)
        }
    }
}

#[derive(Clone)]
pub struct ButtonDebugger(pub [bool; 8]);

impl HandleButtonPressed for ButtonDebugger {
    fn handle_button_pressed(&self, state: &mut State, button: Button, button_state: ButtonState) {
        let index: usize = button.into();
        let mut buttons = self.0.clone();

        buttons[index] = button_state.into();
        state.route = Route::ButtonDebugger(ButtonDebugger(buttons));
    }
}

#[derive(Clone)]
pub enum Route {
    Home(Home),
    Statistics(Statistics),
    ButtonDebugger(ButtonDebugger),
}

impl Route {
    pub fn handle_button_pressed(state: &mut State, button: Button, button_state: ButtonState) {
        match state.route.clone() {
            Route::Home(home) => home.handle_button_pressed(state, button, button_state),
            Route::Statistics(statistics) => {
                statistics.handle_button_pressed(state, button, button_state)
            }
            Route::ButtonDebugger(button_debugger) => {
                button_debugger.handle_button_pressed(state, button, button_state)
            }
        }
    }
}
