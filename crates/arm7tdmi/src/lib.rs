//! ARM7TDMI
//!
//! # Sources
//!
//! \[1\]: <https://problemkaputt.de/gbatek.htm#armcpureference>

use int_enum::IntEnum;

mod bit;
mod psr;

pub mod arm;
pub mod thumb;

/// Cpu state.
///
/// # Source
///
/// \[1\]: <https://problemkaputt.de/gbatek.htm#armcpuflagsconditionfieldcond>
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
#[repr(u8)]
pub enum CpuState {
    /// ARM (32-bit opcodes).
    Arm = 0,
    /// THUMB (16-bit opcodes).
    Thumb = 1,
}

/// Cpu mode.
///
/// # Source
///
/// \[1\]: <https://problemkaputt.de/gbatek.htm#armcpuflagsconditionfieldcond>
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
#[repr(u8)]
pub enum CpuMode {
    /// User (non-privileged).
    User = 0b10000,
    /// FIQ.
    Fiq = 0b10001,
    /// IRQ.
    Irq = 0b10010,
    /// Supervisor (SWI).
    Supervisor = 0b10011,
    /// Abort.
    Abort = 0b10111,
    /// Undefined.
    Undefined = 0b11011,
    /// System (privileged `User` mode).
    System = 0b11111,
}
