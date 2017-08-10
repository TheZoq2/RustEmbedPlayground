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
            resources: [GPIOC, ON],
        },
    }
}

fn init(p: init::Peripherals, r: init::Resources) {
    r.ON;


    // Power up gpioc
    p.RCC.apb2enr.modify(|_, w| w.iopcen().enabled());

    // Configure pc13 as an output
    p.GPIOC.bsrr.write(|w| w.bs13().set());
    p.GPIOC
        .crh
        .modify(|_, w| w.mode13().output().cnf13().push());

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
    **r.ON = !r.ON;

    if **r.ON {
        r.GPIOC.bsrr.write(|w| w.bs13().set());
    }
    else {
        r.GPIOC.bsrr.write(|w| w.br13().reset());
    }
}
