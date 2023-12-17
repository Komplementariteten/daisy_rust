#![no_main]
#![no_std]

#[macro_use]
mod utilities;

use core::result;

use cortex_m_rt::entry;

use daisy_bsp as daisy;

use daisy::hal;
use hal::prelude::*;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use log::info;

// use sh1106::Builder;
// use sh1106::mode::GraphicsMode;

#[entry]
fn main() -> ! {
    // - board setup ----------------------------------------------------------
    utilities::logger::init();
    let board = daisy::Board::take().unwrap();
    let dp = daisy::pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let pwrcfg = dp.PWR.constrain().freeze();

    /* let ccdr = board.freeze_clocks(dp.PWR.constrain(),
    dp.RCC.constrain(),
    &dp.SYSCFG); */

    let ccdr = rcc.sys_ck(100.MHz()).freeze(pwrcfg, &dp.SYSCFG);

    let pins = board.split_gpios(
        dp.GPIOA.split(ccdr.peripheral.GPIOA),
        dp.GPIOB.split(ccdr.peripheral.GPIOB),
        dp.GPIOC.split(ccdr.peripheral.GPIOC),
        dp.GPIOD.split(ccdr.peripheral.GPIOD),
        dp.GPIOE.split(ccdr.peripheral.GPIOE),
        dp.GPIOF.split(ccdr.peripheral.GPIOF),
        dp.GPIOG.split(ccdr.peripheral.GPIOG),
    );

    let scl = pins.SEED_PIN_11.into_push_pull_output();
    let sda = pins.SEED_PIN_12.into_alternate_open_drain();
    let usart1 = pins.SEED_PIN_14.;

    let mut i2c = dp
        .I2C1
        .i2c((scl, sda), 100.kHz(), ccdr.peripheral.I2C4, &ccdr.clocks);

    info!("Entering main loop");

    // - main loop ------------------------------------------------------------
    let mut buf = [0x60];
    loop {
        buf[0] = 0x11;
        i2c.master_write(0x76, 1, hal::i2c::Stop::Software);
        let result = i2c.write(0x76, &buf);
        i2c.master_stop();
        info!("i2c result: {}", result.is_err());
    }
}
