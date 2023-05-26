#![no_std]
#![no_main]
#![feature(panic_info_message, strict_provenance)]

use core::arch::global_asm;

use drivers_pci::Pci;
use drivers_tts::Tts;

global_asm!(include_str!("asm.S"));

#[no_mangle]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
extern "C" fn _start_rust() {
    unsafe {
        let pci = Pci::new(0x30000000);
        let mut tts = Tts::from(pci.device(0).unwrap());

        tts.flush();
        tts.write_string("Hello, world!");
        tts.speech();
        tts.flush();
    }
}

#[no_mangle]
extern "C" fn _start_trap() {
    loop {}
}
