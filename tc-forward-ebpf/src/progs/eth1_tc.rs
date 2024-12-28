use aya_ebpf::{
    helpers::bpf_redirect,
    macros::classifier, programs::TcContext};
use aya_log_ebpf::info;

use crate::ETH2;
#[classifier]
pub fn eth1_tc_forward(ctx: TcContext) -> i32 {
    match try_eth1_tc_forward(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_eth1_tc_forward(ctx: TcContext) -> Result<i32, i32> {
    info!(&ctx, "received a packet on eth1");
    let ret = unsafe { bpf_redirect(ETH2, 0) };
    info!(&ctx, "redirect return value: {}", ret);
    return unsafe { Ok(ret as i32) };
}
