use {
    crate::store::{Route, Store},
    embassy_nrf::{
        bind_interrupts,
        interrupt::{self, InterruptExt, Priority},
        peripherals::{self, P0_11, P0_12},
        twim::{self, Frequency},
    },
    embedded_graphics::{
        mono_font::{MonoTextStyleBuilder, ascii::FONT_6X10},
        pixelcolor::BinaryColor,
        prelude::*,
        primitives::{Circle, Line, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle},
    },
    embedded_text::{
        TextBox,
        alignment::HorizontalAlignment,
        style::{HeightMode, TextBoxStyleBuilder},
    },
    ssd1306::{
        I2CDisplayInterface, Ssd1306Async, mode::DisplayConfigAsync, prelude::DisplayRotation,
        size::DisplaySize128x64,
    },
};

bind_interrupts!(struct Irqs {
    TWISPI0 => twim::InterruptHandler<peripherals::TWISPI0>;
});

#[embassy_executor::task]
pub async fn render_task(twispi0: peripherals::TWISPI0, p0_12: P0_12, p0_11: P0_11) {
    interrupt::TWISPI0.set_priority(Priority::P3);
    let mut config = twim::Config::default();
    config.frequency = Frequency::K400;
    let i2c = twim::Twim::new(twispi0, Irqs, p0_12, p0_11, config);

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306Async::new(interface, DisplaySize128x64, DisplayRotation::Rotate270)
        .into_buffered_graphics_mode();

    display.init().await.unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();
    let textbox_style = TextBoxStyleBuilder::new()
        .height_mode(HeightMode::FitToText)
        .alignment(HorizontalAlignment::Justified)
        .paragraph_spacing(6)
        .build();

    TextBox::with_textbox_style(
        "Hello world!",
        Rectangle::new(Point::zero(), Size::new(64, 0)),
        text_style,
        textbox_style,
    )
    .draw(&mut display)
    .unwrap();

    display.flush().await.unwrap();

    let battery_border_style = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::On)
        .stroke_width(1)
        .fill_color(BinaryColor::Off)
        .build();
    let battery_fill_style = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::Off)
        .stroke_width(1)
        .fill_color(BinaryColor::On)
        .build();
    let stroke_white_1px = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
    let stroke_black_1px = PrimitiveStyle::with_stroke(BinaryColor::Off, 1);

    loop {
        let state = Store::select().await;

        display.clear_buffer();

        let v_min = 3.2;
        let v_max = 4.2;
        let v_battery = state.battery_voltage;
        let v_percentage = ((v_battery - v_min) / (v_max - v_min)) * 100.0;

        Rectangle::new(Point::new(64 - 16, 0), Size::new(16, 8))
            .into_styled(battery_border_style)
            .draw(&mut display)
            .unwrap();

        let battery_fill_width = (v_percentage / 100.0) * 14.0;

        Rectangle::new(
            Point::new(64 - 16 + 1 - (battery_fill_width as i32 - 14), 1),
            Size::new(battery_fill_width as u32, 6),
        )
        .into_styled(battery_fill_style)
        .draw(&mut display)
        .unwrap();

        Line::new(Point::new(64 - 16 - 1, 2), Point::new(64 - 16 - 1, 5))
            .into_styled(stroke_white_1px)
            .draw(&mut display)
            .unwrap();

        match state.route {
            Route::ButtonDebugger(buttons) => {
                for (index, button) in buttons.iter().rev().enumerate() {
                    Circle::new(Point::new(24, 16 * index as i32), 16)
                        .into_styled(if *button {
                            stroke_black_1px
                        } else {
                            stroke_white_1px
                        })
                        .draw(&mut display)
                        .unwrap();
                }
            }
        }

        display.flush().await.unwrap();
    }
}
