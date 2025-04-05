use {
    crate::store::{Action, Store},
    defmt::Format,
    embassy_nrf::gpio::{AnyPin, Input, Level, Pull},
    embassy_time::Timer,
};

#[derive(Debug, Format)]
pub enum Button {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

#[derive(Debug, Format)]
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
            0 => Ok(Self::Zero),
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            4 => Ok(Self::Four),
            5 => Ok(Self::Five),
            6 => Ok(Self::Six),
            7 => Ok(Self::Seven),
            _ => Err(()),
        }
    }
}

impl From<Button> for usize {
    fn from(val: Button) -> Self {
        match val {
            Button::Zero => 0,
            Button::One => 1,
            Button::Two => 2,
            Button::Three => 3,
            Button::Four => 4,
            Button::Five => 5,
            Button::Six => 6,
            Button::Seven => 7,
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
                Store::dispatch(Action::ButtonPressed(Button::$var, ButtonState::Down)).await;
                Timer::after_millis(10).await;
                input.wait_for_low().await;
                Store::dispatch(Action::ButtonPressed(Button::$var, ButtonState::Up)).await;
                Timer::after_millis(10).await;
            }
        }
    };
}

impl_button_task!(button_task_0, Zero);
impl_button_task!(button_task_1, One);
impl_button_task!(button_task_2, Two);
impl_button_task!(button_task_3, Three);
impl_button_task!(button_task_4, Four);
impl_button_task!(button_task_5, Five);
impl_button_task!(button_task_6, Six);
impl_button_task!(button_task_7, Seven);
