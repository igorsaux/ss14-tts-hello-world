#![no_std]
#![no_main]

use pci::PciBus;
use tts::Tts;

use riscv_rt::entry;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main() -> ! {
    unsafe {
        let pci = PciBus::default();
        let mut tts = Tts::from(pci.device(0).unwrap());

        tts.flush();
        tts.write_string("Hello, world!");
        tts.speech();
        tts.flush();

        loop {}
    }
}
