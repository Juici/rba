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

/// Opcode suffixes for conditionally executed code based on the `N`, `Z`, `C`,
/// `V` flags in [CPSR].
///
/// In [ARM] mode, `Cond` can be used with all opcodes. In [THUMB] mode, `Cond`
/// can only be used with branch opcodes.
///
/// [CPSR]: crate::psr::Psr
/// [ARM]: CpuState::Arm
/// [THUMB]: CpuState::Thumb
///
/// # Sources
///
/// \[1\]: <https://problemkaputt.de/gbatek.htm#armcpuflagsconditionfieldcond>
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
#[repr(u8)]
pub enum Cond {
    /// Equal (zero, `Z=1`).
    EQ = 0x0,
    /// Not equal (nonzero, `Z=0`).
    NE = 0x1,
    /// Unsigned higher or same (carry set, `C=1`).
    HS = 0x2,
    /// Unsigned lower (carry cleared, `C=0`).
    LO = 0x3,
    /// Signed negative (minus, `N=1`).
    MI = 0x4,
    /// Signed positive or zero (plus, `N=1`).
    PL = 0x5,
    /// Signed overflow (overflow set, `V=1`).
    VS = 0x6,
    /// Signed no overflow (overflow cleared, `V=0`).
    VC = 0x7,
    /// Unsigned higher (`C=1` and `Z=0`).
    HI = 0x8,
    /// Unsigned lower or same (`C=0` or `Z=1`).
    LS = 0x9,
    /// Signed greater than or equal (`N=V`).
    GE = 0xA,
    /// Signed less than (`N!=V`).
    LT = 0xB,
    /// Signed greater than (`Z=0` and `N=V`).
    GT = 0xC,
    /// Signed less than or equal (`Z=1` or `N!=V`).
    LE = 0xD,
    /// Always.
    AL = 0xE,
    /// Invalid.
    Invalid = 0xF,
}
