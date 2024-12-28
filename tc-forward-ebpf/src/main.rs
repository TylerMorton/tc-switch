#![no_std]
#![no_main]

pub mod progs;

use aya_ebpf::{
    helpers::bpf_redirect,
    macros::classifier, programs::TcContext};
use aya_log_ebpf::info;
pub const ETH1: u32 = 6;
pub const ETH2: u32 = 7;



#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
