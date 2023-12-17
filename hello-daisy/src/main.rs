#![no_main]
#![no_std]

#[macro_use]
mod utilities;

use cortex_m_rt::entry;

use daisy::hal::prelude::*;
use daisy_bsp as daisy;


use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use log::{error, info};

use sh1106::{prelude::*, Builder};

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

    let mut scl = pins.SEED_PIN_11.into_alternate().set_open_drain();
    let mut sda = pins.SEED_PIN_12.into_alternate().set_open_drain();
    // scl.into_push_pull_output().set_high();
    // let up1 = sda.into_push_pull_output_in_state(hal::gpio::PinState::High);

    /* let mut i2c = dp
        .I2C1
        .i2c((scl, sda), 100.kHz(), ccdr.peripheral.I2C1, &ccdr.clocks);

    info!("Entering main loop");

    // - main loop ------------------------------------------------------------
    let buf = [0x80, 0xae];
    // i2c.master_re_start(0x3c, buf.len(), hal::i2c::Stop::Software);
    // i2c.master_write(0x3c, buf.len(), hal::i2c::Stop::Automatic);
    let message = match i2c.write(0x3C, &buf) {
        Ok(n) => "write ok",
        Err(e) => "error",
    };
    info!("{}", message); */
    loop {
        
    }
}
