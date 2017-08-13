#![deny(unsafe_code)]
#![feature(proc_macro)]
#![no_std]
#![feature(lang_items)]


extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate stm32f30x;

use cortex_m::peripheral::SystClkSource;
use rtfm::{app, Threshold};

mod pwm;

app! {
    device: stm32f30x,

    resources: {
        static STATE: u8 = 0;
        static PWM: Option<pwm::Pwm> = None;
        static ACTIVE_PWM: Option<pwm::ActivePwm> = None;
    },

    tasks: {
        SYS_TICK: {
            path: sys_tick,
            resources: [PWM, GPIOE, STATE],
        },
        TIM7: {
            path: sys_tim7,
            resources: [GPIOE, TIM7, PWM, ACTIVE_PWM]
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
    p.RCC.apb1enr.modify(|_, w| w.tim7en().enabled());

    p.TIM7.dier.modify(|_, w| w.uie().set_bit());

    // Make the counter count at 1 mHz
    p.TIM7.psc.modify(|_, w| w.psc().bits(7));
    //Reload every half second
    //p.TIM7.arr.modify(|_, w| w.arr().bits(5_000_000 as u16));

    // Enable interrupts and clock for timer7
    p.TIM7.cr1.modify(|_, w| w.cen().enabled());

    **r.PWM = {
        let mut pwm = pwm::Pwm::new(2500);

        pwm.set_channel(2, 20);
        pwm.set_channel(1, 2000);
        pwm.set_channel(0, 700);

        Some(pwm)
    };

    // Start the pwm in one ms
    p.TIM7.arr.modify(|_, w| w.arr().bits(1000));
}

fn idle() -> ! {
    loop {
        rtfm::wfi();
    }
}


fn sys_tick(_t: &mut Threshold, r: SYS_TICK::Resources) {
    **r.STATE += 1;

    if **r.STATE % 2 == 0 {
        r.GPIOE.odr.modify(|_, w| {
            w.odr12().set_bit()
                .odr13().set_bit()
                .odr14().set_bit()
                .odr15().set_bit()
        });

        match **r.PWM {
            Some(ref mut pwm) => {
                pwm.set_channel(1, 1000)
            },
            None => {}
        }
    } else {
        r.GPIOE.odr.modify(|_, w| {
            w.odr12().clear_bit()
                .odr13().clear_bit()
                .odr14().clear_bit()
                .odr15().clear_bit()
        });
        match **r.PWM {
            Some(ref mut pwm) => {
                pwm.set_channel(1, 2000)
            },
            None => {}
        }
    }

    if **r.STATE % 3 == 1 {
        match **r.PWM {
            Some(ref mut pwm) => {
                pwm.set_channel(0, 1200)
            },
            None => {}
        }
    }
    else {
        match **r.PWM {
            Some(ref mut pwm) => {
                pwm.set_channel(0, 1800)
            },
            None => {}
        }
    }
}

fn sys_tim7(_t: &mut Threshold, r: TIM7::Resources) {
    // Clear the interrupt
    r.TIM7.sr.modify(|_, w| w.uif().clear_bit());

    let done = match **r.ACTIVE_PWM {
        Some(ref mut active_pwm) => {
            let tick_result = active_pwm.on_timer_tick();

            // Set the timer to pause on the next milestone
            r.TIM7.arr.modify(|_, w| w.arr().bits(tick_result.next_step));
            // Reset the counter
            r.TIM7.cnt.reset();

            // Perform the actual pwm. Return true if it is done
            match tick_result.command {
                pwm::TimerTickCommand::Done => {
                    true
                },
                pwm::TimerTickCommand::TurnOff(amount, channels) => {
                    for i in 0..amount {
                        match channels[i] {
                            0 => r.GPIOE.odr.modify(|_, w| w.odr8().clear_bit()),
                            1 => r.GPIOE.odr.modify(|_, w| w.odr9().clear_bit()),
                            2 => r.GPIOE.odr.modify(|_, w| w.odr10().clear_bit()),
                            _ => {}
                        }
                    }

                    false
                }
            }
        },
        None => {true}
    };

    // If this PWM cycle is done, we recreate it from the new commands
    if done == true {
        match **r.PWM {
            Some(ref pwm) => { 
                let active_pwm = pwm::ActivePwm::new(&pwm);

                // Set the timer to pause on the next milestone
                r.TIM7.arr.modify(|_, w| w.arr().bits(active_pwm.get_current_sleep()));
                // Reset the counter
                r.TIM7.cnt.reset();

                **r.ACTIVE_PWM = Some(active_pwm);

                // Activate all the outputs
                r.GPIOE.odr.modify(|_, w| {
                    w.odr8().set_bit()
                        .odr9().set_bit()
                        .odr10().set_bit()
                })
            }
            None => {
                **r.ACTIVE_PWM = None
            }
        }
    }
}
