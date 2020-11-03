#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1::stm32f103::{interrupt, Interrupt, NVIC};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello, world!");

    unsafe { NVIC::unmask(Interrupt::EXTI0) };

    loop {
        NVIC::pend(Interrupt::EXTI0);
        asm::bkpt()
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    rprintln!("{}", info);
    exit()
}

fn exit() -> ! {
    loop {
        asm::bkpt() // halt = exit probe-run
    }
}

#[interrupt]
fn EXTI0() {
    rprintln!("EXTI0");
}
