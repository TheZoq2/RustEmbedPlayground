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
    f3::itm::init();
    //init_h_bridge();
}

const GPIO_OUTPUT: u8 = 0b01;
//Needs to happen in an interrupt free enviroment, possibly?
pub unsafe fn init_h_bridge()
{
    //Turn on gpiod
    let rcc_register = peripheral::rcc_mut();
    rcc_register.ahbenr.modify(|_, w| w.iopden(true));

    //set pd11,13,15 enabled
    let gpiod_register = peripheral::gpiod_mut();
    gpiod_register.moder.modify(|_, w|{
                w.moder15(GPIO_OUTPUT)
                    .moder13(GPIO_OUTPUT)
                    .moder11(GPIO_OUTPUT)
    });
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

enum HBridgeState
{
    Enabled,
    Disabled
}
/**
  Enables or disables the h bridge
 */
fn set_h_bridge_state(state: HBridgeState)
{
    unsafe{init_h_bridge();}
    let gpiod = unsafe{peripheral::gpiod_mut()};

    let output = match state
    {
        HBridgeState::Enabled => true,
        HBridgeState::Disabled => false
    };

    gpiod.odr.modify(|_, w| w.odr11(output));
}

enum HBridgeDirection
{
    Forward,
    Backward,
    Break,
}

fn set_h_bridge_direction(direction: HBridgeDirection)
{
    let (pin1, pin2) = match direction
    {
        HBridgeDirection::Forward => (true, false),
        HBridgeDirection::Backward => (false, true),
        HBridgeDirection::Break => (false, false)
    };

    let gpiod = unsafe{peripheral::gpiod_mut()};

    gpiod.odr.modify(|_, w|{
        w.odr13(pin1).odr15(pin2)
    });
}

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    let half_period = 1000;

    iprintln!("You are yolo");

    set_h_bridge_state(HBridgeState::Enabled);
    loop
    {
        f3::delay::ms(half_period);
        set_north_led(true);
        set_h_bridge_direction(HBridgeDirection::Backward);
        f3::delay::ms(half_period);
        //set_south_led(true);
        set_h_bridge_direction(HBridgeDirection::Forward);
    }
}
