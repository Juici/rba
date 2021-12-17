//! Program status register model.
//!
//! # Sources
//!
//! \[1\]: <https://problemkaputt.de/gbatek.htm#armcpuflagsconditionfieldcond>

use crate::bit::BitIndex;
use crate::{CpuMode, CpuState};

/// A program status register (xPSR).
#[derive(Clone, Copy, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct Psr {
    raw: u32,
}

impl Psr {
    /// Returns the current operating mode (bits 4-0).
    pub fn mode(self) -> Option<CpuMode> {
        CpuMode::try_from(self.raw.bits::<0, 5>() as u8).ok()
    }

    pub fn state(self) -> CpuState {
        if self.raw.bit::<5>() {
            CpuState::Thumb
        } else {
            CpuState::Arm
        }
    }

    pub fn fiq_disabled(self) -> bool {
        self.raw.bit::<6>()
    }

    pub fn irq_disabled(self) -> bool {
        self.raw.bit::<7>()
    }

    /// Checks the overflow flag (V).
    pub fn overflow(self) -> bool {
        self.raw.bit::<28>()
    }

    /// Checks the carry flag (C).
    pub fn carry(self) -> bool {
        self.raw.bit::<29>()
    }

    /// Checks the zero flag (Z).
    pub fn zero(self) -> bool {
        self.raw.bit::<30>()
    }

    /// Checks the sign flag (N).
    pub fn sign(self) -> bool {
        self.raw.bit::<31>()
    }
}
