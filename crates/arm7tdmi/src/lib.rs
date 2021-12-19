//! ARM7TDMI
//!
//! # Sources
//!
//! \[1\]: <https://problemkaputt.de/gbatek.htm#armcpureference>

use std::fmt;

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

impl fmt::Display for CpuState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CpuState::Arm => f.pad("ARM"),
            CpuState::Thumb => f.pad("THUMB"),
        }
    }
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

impl fmt::Display for CpuMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CpuMode::User => f.pad("USR"),
            CpuMode::Fiq => f.pad("FIQ"),
            CpuMode::Irq => f.pad("IRQ"),
            CpuMode::Supervisor => f.pad("SVC"),
            CpuMode::Abort => f.pad("ABT"),
            CpuMode::Undefined => f.pad("UND"),
            CpuMode::System => f.pad("SYS"),
        }
    }
}
