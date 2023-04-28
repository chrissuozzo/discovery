#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds) = aux5::init();

    let led_count = leds.len();
    loop {
        for i in 0..led_count {
            leds[i].on().ok();
            delay.delay_ms(50_u16);
            if i == 0 {
                leds[led_count - 1].off().ok();
            } else {
                leds[i - 1].off().ok();
            }
            delay.delay_ms(50_u16);
        }
    }
}
