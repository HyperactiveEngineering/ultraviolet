use {
    crate::state::{Action, State, reducer},
    defmt::unwrap,
    embassy_sync::{
        blocking_mutex::raw::ThreadModeRawMutex,
        channel::Channel,
        mutex::Mutex,
        watch::{Receiver, Watch},
    },
};

const MAX_SUBSCRIPTIONS: usize = 2;

static STATE: Mutex<ThreadModeRawMutex, State> = Mutex::new(State::new());
static ACTION: Channel<ThreadModeRawMutex, Action, 8> = Channel::new();
static SUBSCRIPTION: Watch<ThreadModeRawMutex, State, MAX_SUBSCRIPTIONS> = Watch::new();

impl Action {
    pub async fn dispatch(self) {
        ACTION.send(self).await;
    }
}

pub struct Subscription<'a> {
    receiver: Receiver<'a, ThreadModeRawMutex, State, MAX_SUBSCRIPTIONS>,
}

impl<'a> State {
    pub fn subscribe() -> Subscription<'a> {
        Subscription {
            receiver: unwrap!(SUBSCRIPTION.receiver()),
        }
    }
}

impl<'a> Subscription<'a> {
    pub async fn latest(&mut self) -> State {
        self.receiver.changed().await
    }
}

#[embassy_executor::task]
pub async fn store_task() {
    let sender = SUBSCRIPTION.sender();

    loop {
        let action = ACTION.receive().await;
        let mut state = STATE.lock().await;
        reducer(&mut state, action);
        sender.send(state.clone());
    }
}

macro_rules! action {
    (
        $(
            $name_upper:ident $name_lower:ident $( { $( $input:ident: $type:ty ),* $(,)? } )?
        ),* $(,)?
    ) => {
        pub enum Action {
            $(
                $name_upper $( { $( $input: $type ),* } )?,
            )*
        }

        pub fn reducer(state: &mut State, action: Action) {
            match action {
                $(
                    Action::$name_upper $( { $($input),* } )? => state.$name_lower( $( $( $input ),* )? ),
                )*
            }
        }
    };
}

pub(crate) use action;
