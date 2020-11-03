#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
use defmt;
use defmt_rtt as _; // global logger
use stm32f1::stm32f103::{interrupt, Interrupt, NVIC};

#[entry]
fn main() -> ! {
    defmt::info!("Hello, world!");

    unsafe { NVIC::unmask(Interrupt::EXTI0) };

    loop {
        NVIC::pend(Interrupt::EXTI0);
        asm::bkpt()
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    defmt::info!("Panic!");
    exit()
}

fn exit() -> ! {
    loop {
        asm::bkpt() // halt = exit probe-run
    }
}

#[interrupt]
fn EXTI0() {
    defmt::info!("IRQ!");
}
