#![no_main]
#![no_std]

mod pins;
mod led;
mod clocks;
mod daisy;

use cortex_m_rt::entry;
use panic_semihosting as _;
use stm32h7xx_hal as hal;
use rtt_target::rtt_init_print;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    // Get device peripherals and the board abstraction.
    let dp = hal::pac::Peripherals::take().unwrap();
    let board = daisy::Board::take().unwrap();

    // Configure board's peripherals.
    let ccdr = board_freeze_clocks!(board, dp);
    let pins = board_split_gpios!(board, ccdr, dp);
    let mut led_user = board_split_leds!(pins).USER;

    // Blink every second.
    let one_second = ccdr.clocks.sys_ck().to_Hz();
    loop {
        led_user.toggle();
        cortex_m::asm::delay(one_second);
    }
}
