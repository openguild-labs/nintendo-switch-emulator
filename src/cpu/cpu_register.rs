use crate::cpu::constant;

pub struct CpuRegister {
    /// ## General Purpose Registers (GPRs)
    /// Link to documentation: https://wiki.oatmealdome.me/ARMv8#:~:text=variety%20of%20registers.-,GPR,-The%20GPRs%20have
    gprs: [u64; constant::GPR_COUNT],
    /// ##  Floating-Point Registers (FPRs)
    /// Link to documentation: https://wiki.oatmealdome.me/ARMv8#:~:text=than%20X.-,FPR,-The%20architecture%20has
    fprs: [u128; constant::FPR_COUNT],
    /// ## Speical Registers (SRs)
    wzr: u32,
    xzr: u64,
    wsp: u32,
    sp: u64,
    pc: u64,
    elr: u64,
    spsr: u64,
}

impl CpuRegister {
    pub fn new() -> Self {
        CpuRegister {
            gprs: [0u64; constant::GPR_COUNT],
            fprs: [0u128; constant::FPR_COUNT],
            wzr: 0u32,
            xzr: 0u64,
            wsp: 0u32,
            sp: 0u64,
            pc: 0u64,
            elr: 0u64,
            spsr: 0u64,
        }
    }
}
