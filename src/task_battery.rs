use {
    crate::state::{Action, State},
    embassy_nrf::{
        bind_interrupts,
        peripherals::{P0_29, SAADC},
        saadc::{self, ChannelConfig, Config, Oversample, Saadc},
    },
    embassy_time::Timer,
};

bind_interrupts!(struct Irqs {
    SAADC => saadc::InterruptHandler;
});

// input range = reference voltage / gain = 0v6 / (1/6) = 3v6
const SAADC_INPUT_RANGE: f32 = 3.6;
const MAX_12_BIT_VALUE: f32 = 4095.0;

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
        Action::BatteryVoltage { voltage }.dispatch().await;

        Timer::after_millis(1000).await;
    }
}

impl State {
    pub fn handle_battery_voltage(&mut self, voltage: f32) {
        self.battery_voltage = voltage;
    }

    pub fn battery_voltage(&self) -> f32 {
        self.battery_voltage
    }

    pub fn battery_percentage(&self) -> f32 {
        let v_min = 3.2;
        let v_max = 4.2;
        let v_battery = self.battery_voltage;
        ((v_battery - v_min) / (v_max - v_min)) * 100.0
    }
}
