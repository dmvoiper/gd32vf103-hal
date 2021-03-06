#![no_std]
#![no_main]

extern crate panic_halt;

use gd32vf103_hal::{prelude::*, pac};

#[riscv_rt::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcu = dp.RCU.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcu.apb2);
    let mut pa1 = gpioa.pa1.into_push_pull_output(&mut gpioa.ctl0);
    pa1.set_low().unwrap();
    loop {}
}
