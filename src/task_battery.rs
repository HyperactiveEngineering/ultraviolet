use {
    crate::store::{Action, Store},
    embassy_nrf::{
        bind_interrupts,
        peripherals::{P0_29, SAADC},
        saadc::{self, ChannelConfig, Config, Oversample, Saadc},
    },
    embassy_time::Timer,
};

// input range = reference voltage / gain = 0v6 / (1/6) = 3v6
const SAADC_INPUT_RANGE: f32 = 3.6;
const MAX_12_BIT_VALUE: f32 = 4095.0;

bind_interrupts!(struct Irqs {
    SAADC => saadc::InterruptHandler;
});

#[embassy_executor::task]
pub async fn battery_task(p_saadc: SAADC, p0_29: P0_29) {
    let channel_config = ChannelConfig::single_ended(p0_29);
    let mut config = Config::default();
    config.oversample = Oversample::OVER16X;
    let mut saadc = Saadc::new(p_saadc, Irqs, config, [channel_config]);

    loop {
        let mut buf = [0i16; 1];
        saadc.sample(&mut buf).await;

        let voltage = (buf[0] as f32 / MAX_12_BIT_VALUE) * SAADC_INPUT_RANGE * 2.0;
        Store::dispatch(Action::BatteryVoltage(voltage)).await;

        Timer::after_millis(1000).await;
    }
}
