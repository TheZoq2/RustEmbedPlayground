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
    },

    tasks: {
        SYS_TICK: {
            path: sys_tick,
            resources: [GPIOE, ON],
        },
    }
}

fn init(p: init::Peripherals, r: init::Resources) {
    // Power up gpioc
    p.RCC.ahbenr.modify(|_, w| w.iopeen().enabled());

    // Enable gpoie15
    p.GPIOE.moder.modify(|_, w| w.moder15().output());

    // configure the system timer to generate one interrupt every second
    p.SYST.set_clock_source(SystClkSource::Core);
    p.SYST.set_reload(8_000_000); // 1s
    p.SYST.enable_interrupt();
    p.SYST.enable_counter();
}

fn idle() -> ! {
    loop {
        rtfm::wfi();
    }
}


fn sys_tick(_t: &mut Threshold, r: SYS_TICK::Resources)
{
    **r.ON = !**r.ON;

    if **r.ON {
        //r.GPIOC.bsrr.write(|w| w.bs13().set());
        r.GPIOE.odr.modify(|_, w| w.odr15().set_bit())
    }
    else {
        r.GPIOE.odr.modify(|_, w| w.odr15().clear_bit())
    }
}
