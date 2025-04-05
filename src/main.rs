#![no_std]
#![no_main]

extern crate alloc;
extern crate defmt_rtt;
extern crate panic_probe;

mod store;
mod task_battery;
mod task_bluetooth;
mod task_button;
mod task_screen;

use {
    core::mem,
    defmt::unwrap,
    embassy_executor::Spawner,
    embassy_nrf::{gpio::Pin, interrupt::Priority},
    embedded_alloc::LlffHeap as Heap,
    nrf_softdevice::{Softdevice, raw},
    store::reducer_task,
    task_battery::battery_task,
    task_bluetooth::softdevice_task,
    task_button::{
        button_task_0, button_task_1, button_task_2, button_task_3, button_task_4, button_task_5,
        button_task_6, button_task_7,
    },
    task_screen::render_task,
};

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(&raw mut HEAP_MEM as usize, HEAP_SIZE) }
    }

    let mut config = embassy_nrf::config::Config::default();
    config.gpiote_interrupt_priority = Priority::P2;
    config.time_interrupt_priority = Priority::P2;
    let p = embassy_nrf::init(config);

    unwrap!(spawner.spawn(reducer_task()));
    unwrap!(spawner.spawn(button_task_7(p.P0_25.degrade())));
    unwrap!(spawner.spawn(button_task_6(p.P1_08.degrade())));
    unwrap!(spawner.spawn(button_task_5(p.P0_07.degrade())));
    unwrap!(spawner.spawn(button_task_4(p.P0_26.degrade())));
    unwrap!(spawner.spawn(button_task_3(p.P0_27.degrade())));
    unwrap!(spawner.spawn(button_task_2(p.P0_06.degrade())));
    unwrap!(spawner.spawn(button_task_1(p.P0_08.degrade())));
    unwrap!(spawner.spawn(button_task_0(p.P1_09.degrade())));
    unwrap!(spawner.spawn(render_task(p.TWISPI0, p.P0_12, p.P0_11)));
    unwrap!(spawner.spawn(battery_task(p.SAADC, p.P0_29)));

    let config = nrf_softdevice::Config {
        clock: Some(raw::nrf_clock_lf_cfg_t {
            source: raw::NRF_CLOCK_LF_SRC_RC as u8,
            rc_ctiv: 16,
            rc_temp_ctiv: 2,
            accuracy: raw::NRF_CLOCK_LF_ACCURACY_500_PPM as u8,
        }),
        conn_gap: Some(raw::ble_gap_conn_cfg_t {
            conn_count: 6,
            event_length: 24,
        }),
        conn_gatt: Some(raw::ble_gatt_conn_cfg_t { att_mtu: 256 }),
        gatts_attr_tab_size: Some(raw::ble_gatts_cfg_attr_tab_size_t {
            attr_tab_size: raw::BLE_GATTS_ATTR_TAB_SIZE_DEFAULT,
        }),
        gap_role_count: Some(raw::ble_gap_cfg_role_count_t {
            adv_set_count: 1,
            periph_role_count: 3,
            central_role_count: 3,
            central_sec_count: 0,
            _bitfield_1: raw::ble_gap_cfg_role_count_t::new_bitfield_1(0),
        }),
        gap_device_name: Some(raw::ble_gap_cfg_device_name_t {
            p_value: b"HelloRust" as *const u8 as _,
            current_len: 9,
            max_len: 9,
            write_perm: unsafe { mem::zeroed() },
            _bitfield_1: raw::ble_gap_cfg_device_name_t::new_bitfield_1(
                raw::BLE_GATTS_VLOC_STACK as u8,
            ),
        }),
        ..Default::default()
    };

    let sd = Softdevice::enable(&config);
    unwrap!(spawner.spawn(softdevice_task(sd)));
}
