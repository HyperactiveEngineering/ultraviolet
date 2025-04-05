# Ultraviolet

## Prerequisites 

- nRF52840 programmable via a JTAG/SWD
  - Seeed Studio
    - [Seeed XIAO nRF52840 Sense](https://wiki.seeedstudio.com/XIAO_BLE/)
    - [Seeed XIAO Expansion Board](https://wiki.seeedstudio.com/Seeeduino-XIAO-Expansion-Board/)
  - Adafruit
    - [Adafruit Feather nRF52840 Express](https://www.adafruit.com/product/4062)
- [`probe-rs`](https://probe.rs/docs/getting-started/installation/)

```shell
probe-rs erase --chip nrf52840_xxAA
probe-rs download --verify --binary-format hex --chip nRF52840_xxAA .cargo/s140_nrf52_7.3.0_softdevice.hex
```
