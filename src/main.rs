#![no_main]
#![no_std]
#![feature(asm)]
#![feature(lang_items)]

//#[macro_use]
//extern crate pg;
#[macro_use]
extern crate f3;

use f3::peripheral;

#[doc(hidden)]
#[export_name = "_init"]
pub unsafe fn init() {
    f3::delay::init();
    f3::led::init();
    f3::itm::init();
}


fn set_north_led(state: bool)
{
    let gpioe = unsafe{peripheral::gpioe_mut()};

    gpioe.odr.write(|w| w.odr9(state));
}
fn set_south_led(state: bool)
{
    let gpioe = unsafe{peripheral::gpioe_mut()};

    gpioe.odr.write(|w| w.odr11(state))
}

fn set_gpio_pd15(state: bool)
{
    let gpiod = unsafe{peripheral::gpiod_mut()};

    gpiod.odr.write(|w| w.odr15(state));
}
fn enable_gpio_portd()
{
    let rcc_register = unsafe{peripheral::rcc_mut()};

    rcc_register.ahbenr.modify(|_, w| w.iopden(true));
}

fn set_gpio_pd15_to_output()
{
    let gpiod_register = unsafe{peripheral::gpiod_mut()};
    gpiod_register.moder.modify(|_, w| w.moder15(0b01));
}

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    enable_gpio_portd();
    set_gpio_pd15_to_output();

    let half_period = 500;

    iprintln!("You are yolo");


    loop 
    {
        f3::delay::ms(half_period);
        set_north_led(true);
        f3::delay::ms(half_period);
        set_south_led(true);

        set_gpio_pd15(true);
    }
}
