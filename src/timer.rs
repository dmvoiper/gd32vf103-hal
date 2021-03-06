//! Timers
use crate::time::Hertz;
use embedded_hal::timer::{CountDown};
use embedded_hal::blocking::delay::DelayMs;
use nb::*;

/// Hardware timers
pub struct Timer6 {
    timer: TIMER6,
    clock_scaler: u16,
    clock_frequency: Hertz,
}

use crate::rcu;
use crate::pac::TIMER6;

impl Timer6 {
    pub fn new(timer: TIMER6, clock: rcu::Clocks, apb1: &mut rcu::APB1) -> Self 
    {
        riscv::interrupt::free(|_| {
            apb1.en().modify(|_,w| w.timer6en().set_bit());
            apb1.rst().write(|w| w.timer6rst().set_bit());
            apb1.rst().write(|w| w.timer6rst().clear_bit());
        });
        Timer6 {
            timer: timer,
            clock_scaler: 1000,
            clock_frequency: clock.ck_apb1(),
        }
    }
}

impl<T: Into<u32>> DelayMs<T> for Timer6 {
    fn delay_ms(&mut self, ms: T) {
        let count = (ms.into() * self.clock_frequency.0) / (self.clock_scaler as u32 * 1000);
        if count > u16::max_value() as u32 {
            panic!("can not delay that long");
        }
        self.start(count as u16);
        block!(self.wait()).ok();
    }
}


impl CountDown for Timer6 {
    type Time = u16;
    fn start<T>(&mut self, count: T) where T: Into<Self::Time> {
        unsafe{
            let c = count.into();
            riscv::interrupt::free(|_| {
                self.timer.psc.write(|w|{w.psc().bits(self.clock_scaler)});
                self.timer.intf.write(|w|{w.upif().clear_bit()});
                self.timer.swevg.write(|w|{w.upg().set_bit()});
                self.timer.intf.write(|w|{w.upif().clear_bit()});
                self.timer.car.modify(|_,w|{w.carl().bits(c)});
                self.timer.ctl0.modify(|_,w|{w.cen().set_bit()});
            });
        }
    }

    //TODO this signature changes in a future version, so we don'ot need the void crate.
    fn wait(&mut self) -> nb::Result<(), void::Void> {
        let flag = self.timer.intf.read().upif().bit_is_set();
        if flag {return Ok(())
        } else {
            return Err(nb::Error::WouldBlock)
        }
    }
}

// impl Periodic for Timer<TIMER2> {}
