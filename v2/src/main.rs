#![deny(unsafe_code)]
#![feature(proc_macro)]
#![no_std]
#![feature(lang_items)]


extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate stm32f30x;

use cortex_m::peripheral::SystClkSource;
use rtfm::{app, Threshold};

app! {
    device: stm32f30x,

    resources: {
        static ON: bool = false;
        static ON2: bool = false;
    },

    tasks: {
        SYS_TICK: {
            path: sys_tick,
            resources: [GPIOE, ON],
        },
        TIM7: {
            path: sys_tim7,
            resources: [GPIOE, ON2, TIM7]
        }
    }
}

fn init(p: init::Peripherals, r: init::Resources) {
    // Power up gpioc
    p.RCC.ahbenr.modify(|_, w| w.iopeen().enabled());

    // Enable gpoie15
    p.GPIOE.moder.modify(|_, w|
                    w.moder8().output()
                        .moder9().output()
                        .moder10().output()
                        .moder11().output()
                        .moder12().output()
                        .moder13().output()
                        .moder14().output()
                        .moder15().output()
                    );

    // configure the system timer to generate one interrupt every second
    p.SYST.set_clock_source(SystClkSource::Core);
    p.SYST.set_reload(8_000_000); // 1s
    p.SYST.enable_interrupt();
    p.SYST.enable_counter();


    //Power up timer 7
    p.RCC.apb1enr.write(|w| w.tim7en().enabled());

    p.TIM7.dier.write(|w| w.uie().set_bit());

    // Make the counter count at 1kHz
    p.TIM7.psc.write(|w| w.psc().bits(7999));
    //Reload every half second
    p.TIM7.arr.write(|w| w.arr().bits(500));

    // Enable interrupts and clock for timer7
    p.TIM7.cr1.modify(|_, w| w.cen().enabled());
}

fn idle() -> ! {
    loop {
        rtfm::wfi();
    }
}


fn sys_tick(_t: &mut Threshold, r: SYS_TICK::Resources) {
    **r.ON = !**r.ON;

    if **r.ON {
        r.GPIOE.odr.modify(|_, w| {
            w.odr12().set_bit()
                .odr13().set_bit()
                .odr14().set_bit()
                .odr15().set_bit()
        })
    }
    else {
        r.GPIOE.odr.modify(|_, w| {
            w.odr12().clear_bit()
                .odr13().clear_bit()
                .odr14().clear_bit()
                .odr15().clear_bit()
        })
    }
}

fn sys_tim7(_t: &mut Threshold, r: TIM7::Resources)
{
    r.TIM7.sr.write(|w| w.uif().clear_bit());

    **r.ON2 = !**r.ON2;

    if **r.ON2 {
        r.GPIOE.odr.modify(|_, w| {
            w.odr8().set_bit()
                .odr9().set_bit()
                .odr10().set_bit()
                .odr11().set_bit()
        })
    }
    else {
        r.GPIOE.odr.modify(|_, w| {
            w.odr8().clear_bit()
                .odr9().clear_bit()
                .odr10().clear_bit()
                .odr11().clear_bit()
        })
    }
}
