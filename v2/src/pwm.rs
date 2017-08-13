
const PWM_CHANNELS: usize = 3;

pub struct Pwm {
    /// In microseconds
    pub pulse_interval: u16,
    /// The duty cycles of each PWM channel. In microseconds.
    /// Sorted by lowest cycle first
    duty_cycles: [(usize, u16); PWM_CHANNELS],
}

impl Pwm {
    pub fn new(pulse_interval: u16) -> Pwm {
        let mut duty_cycles = [(0, 0); PWM_CHANNELS];
        for (i, mut channel_value) in duty_cycles.iter_mut().enumerate() {
            channel_value.0 += i;
        }

        Pwm {
            pulse_interval,
            duty_cycles
        }
    }
    pub fn get_duty_cycles(&self) -> [(usize, u16); PWM_CHANNELS] {
        self.duty_cycles
    }

    pub fn set_channel(&mut self, target_channel: usize, duty_cycle: u16) {
        let mut new_dc = [(0,0); PWM_CHANNELS];

        // Insert all the unchanged channels into the new array
        let mut current_insert_index = 0;
        for elem in self.duty_cycles.iter() {
            if elem.0 == target_channel {
                continue
            }
            else {
                new_dc[current_insert_index] = *elem;
                current_insert_index += 1;
            }
        }

        // Insert the old channel at the back
        new_dc[PWM_CHANNELS-1] = (target_channel, duty_cycle);

        // Make it sorted again
        for i in (1..PWM_CHANNELS).rev() {
            if new_dc[i].1 < new_dc[i-1].1 {
                let temp = new_dc[i];
                new_dc[i] = new_dc[i-1];
                new_dc[i-1] = temp;
            }
        }

        self.duty_cycles = new_dc;
    }
}

pub struct ActivePwm {
    current_index: usize,
    setpoints: [(usize, u16); PWM_CHANNELS],
    end_wait: u16
}

impl ActivePwm {
    pub fn new(pwm: &Pwm) -> ActivePwm {
        let mut setpoints = [(0,0); PWM_CHANNELS];
        let mut last_time = 0;

        for (i, &(channel, value)) in pwm.get_duty_cycles().iter().enumerate() {
            setpoints[i] = (channel, value-last_time);
            last_time = value
        }

        ActivePwm {
            setpoints,
            end_wait: pwm.pulse_interval - last_time,
            current_index: 0
        }
    }

    pub fn get_current_sleep(&self) -> u16 {
        self.setpoints[self.current_index].1
    }

    pub fn on_timer_tick(&mut self) -> TimerTickResult {
        let command = if self.current_index == PWM_CHANNELS {
            self.current_index = 0;

            TimerTickCommand::Done
        }
        else {
            let mut channels_to_turn_off = [0; PWM_CHANNELS];
            channels_to_turn_off[0] = self.setpoints[self.current_index].0;
            let mut off_amount = 1;

            self.current_index += 1;

            while self.current_index < PWM_CHANNELS && self.setpoints[self.current_index].1 == 0 {
                channels_to_turn_off[off_amount] = self.setpoints[self.current_index].0;
                off_amount += 1;
                self.current_index += 1;
            }

            TimerTickCommand::TurnOff(off_amount, channels_to_turn_off)
        };

        let next_step = if self.current_index == PWM_CHANNELS {
            self.end_wait
        } else
        {
            self.setpoints[self.current_index].1
        };

        TimerTickResult {
            next_step,
            command
        }
    }
}

pub enum TimerTickCommand {
    /// All channels should go high
    Done,
    /// The following `n` channels should be turned off
    TurnOff(usize, [usize; PWM_CHANNELS])
}

pub struct TimerTickResult {
    /// The amount of time to wait until triggering the next timer interrupt
    pub next_step: u16,

    pub command: TimerTickCommand
}

