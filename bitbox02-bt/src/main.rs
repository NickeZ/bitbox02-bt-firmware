#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use cortex_m::asm;

#[cfg(not(test))]
#[entry]
fn main() -> ! {
    asm::nop();
    loop {};
}
