use core::arch::asm;
use rng::Rng;

unsafe fn rdmsr(msr: u32) -> u64 {
    let low: u32;
    let high: u32;

    asm!(
        "rdmsr",
        in("ecx") msr,
        out("eax") low,
        out("edx") high,
    );
    ((high as u64) << 32) | (low as u64)
}


unsafe fn wrmsr(msr: u32, val: u64) {
    let low:  u32 = val as u32;
    let high: u32 = (val >> 32) as u32;

    asm!(
        "wrmsr",
        in("ecx") msr,
        in("eax") low,
        in("edx") high,
    );
}


