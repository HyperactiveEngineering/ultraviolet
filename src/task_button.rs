use {
    crate::{
        route::Route,
        state::{Action, State},
    },
    defmt::Format,
    embassy_nrf::gpio::{AnyPin, Input, Level, Pull},
    embassy_time::Timer,
};

#[derive(Debug, Format, PartialEq, Eq)]
pub enum Button {
    Select,
    Back,
    Two,
    Three,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Format, PartialEq, Eq)]
pub enum ButtonState {
    Up,
    Down,
}

impl From<ButtonState> for bool {
    fn from(val: ButtonState) -> Self {
        match val {
            ButtonState::Up => false,
            ButtonState::Down => true,
        }
    }
}

impl From<Level> for ButtonState {
    fn from(value: Level) -> Self {
        match value {
            Level::High => Self::Down,
            Level::Low => Self::Up,
        }
    }
}

impl TryFrom<usize> for Button {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select),
            1 => Ok(Self::Back),
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            4 => Ok(Self::Up),
            5 => Ok(Self::Down),
            6 => Ok(Self::Left),
            7 => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

impl From<Button> for usize {
    fn from(val: Button) -> Self {
        match val {
            Button::Select => 0,
            Button::Back => 1,
            Button::Two => 2,
            Button::Three => 3,
            Button::Up => 4,
            Button::Down => 5,
            Button::Left => 6,
            Button::Right => 7,
        }
    }
}

macro_rules! impl_button_task {
    ($n:ident, $var:ident) => {
        #[embassy_executor::task]
        pub async fn $n(pin: AnyPin) {
            let mut input = Input::new(pin, Pull::Down);

            /*
                Ideally a schmitt trigger or a set-reset flipflop would be used
                in place of a 10ms delay after reading. But for now the delay
                is a convenient solution to the problem of switch bounce.
            */
            loop {
                input.wait_for_high().await;
                Action::ButtonPressed {
                    button: Button::$var,
                    button_state: ButtonState::Down,
                }
                .dispatch()
                .await;
                Timer::after_millis(10).await;
                input.wait_for_low().await;
                Action::ButtonPressed {
                    button: Button::$var,
                    button_state: ButtonState::Up,
                }
                .dispatch()
                .await;
                Timer::after_millis(10).await;
            }
        }
    };
}

impl_button_task!(button_task_0, Select);
impl_button_task!(button_task_1, Back);
impl_button_task!(button_task_2, Two);
impl_button_task!(button_task_3, Three);
impl_button_task!(button_task_4, Up);
impl_button_task!(button_task_5, Down);
impl_button_task!(button_task_6, Left);
impl_button_task!(button_task_7, Right);

impl State {
    pub fn handle_button_pressed(&mut self, button: Button, button_state: ButtonState) {
        Route::handle_button_pressed(self, button, button_state);
    }
}
