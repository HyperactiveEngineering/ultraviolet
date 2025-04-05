use {
    crate::task_button::{Button, ButtonState},
    defmt::Format,
    embassy_sync::{
        blocking_mutex::raw::ThreadModeRawMutex,
        channel::Channel,
        mutex::Mutex,
        watch::{Receiver, Watch},
    },
};

pub enum Action {
    BatteryVoltage(f32),
    ButtonPressed(Button, ButtonState),
}

static DISPATCH: Channel<ThreadModeRawMutex, Action, 8> = Channel::new();
static SELECT_LATEST: Watch<ThreadModeRawMutex, Store, 1> = Watch::new();

#[derive(Debug, Format, Clone, PartialEq, Eq)]
pub enum DefaultOptions {
    ButtonDebugger,
    Statistics,
}

#[derive(Debug, Format, Clone)]
pub enum Route {
    Default { selected: DefaultOptions },
    ButtonDebugger([bool; 8]),
    Statistics,
}

#[derive(Debug, Format, Clone)]
pub struct Store {
    pub battery_voltage: f32,
    pub route: Route,
}

static STATE: Mutex<ThreadModeRawMutex, Store> = Mutex::new(Store {
    battery_voltage: 0.0,
    route: Route::Default {
        selected: DefaultOptions::Statistics,
    },
});

pub struct Subscription<'a> {
    receiver: Receiver<'a, ThreadModeRawMutex, Store, 1>,
}

impl<'a> Subscription<'a> {
    fn new() -> Self {
        Subscription {
            receiver: SELECT_LATEST.receiver().unwrap(),
        }
    }

    pub async fn select_latest(&mut self) -> Store {
        self.receiver.changed().await
    }
}

impl Store {
    pub async fn dispatch(action: Action) {
        DISPATCH.send(action).await
    }

    pub fn subscribe<'a>() -> Subscription<'a> {
        Subscription::new()
    }
}

impl Store {
    fn next(&mut self, action: Action) {
        match action {
            Action::BatteryVoltage(voltage) => {
                self.battery_voltage = voltage;
            }
            Action::ButtonPressed(button, state) => match &mut self.route {
                Route::Default { selected } => match selected {
                    DefaultOptions::ButtonDebugger => {
                        if button == Button::Select && state == ButtonState::Down {
                            self.route = Route::ButtonDebugger(Default::default())
                        }
                        if button == Button::Down && state == ButtonState::Down {
                            self.route = Route::Default {
                                selected: DefaultOptions::Statistics,
                            }
                        }
                    }
                    DefaultOptions::Statistics => {
                        if button == Button::Select && state == ButtonState::Down {
                            self.route = Route::Statistics
                        }
                        if button == Button::Up && state == ButtonState::Down {
                            self.route = Route::Default {
                                selected: DefaultOptions::ButtonDebugger,
                            }
                        }
                    }
                },
                Route::Statistics => {
                    if button == Button::Back && state == ButtonState::Down {
                        self.route = Route::Default {
                            selected: DefaultOptions::Statistics,
                        }
                    }
                }
                Route::ButtonDebugger(buttons) => {
                    let index: usize = button.into();
                    let value = buttons[index];
                    let new_value: bool = state.into();

                    if value != new_value {
                        buttons[index] = new_value
                    } else {
                        buttons[index] = false
                    }
                }
            },
        }
    }
}

#[embassy_executor::task]
pub async fn reducer_task() {
    let sender = SELECT_LATEST.sender();

    loop {
        let action = DISPATCH.receive().await;
        let mut state = STATE.lock().await;
        state.next(action);
        sender.send(state.clone());
    }
}
