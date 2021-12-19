//! Program status register model.
//!
//! # Sources
//!
//! \[1\]: <https://problemkaputt.de/gbatek.htm#armcpuflagsconditionfieldcond>

use std::fmt;

use crate::bit::BitIndex;
use crate::{CpuMode, CpuState};

/// A program status register (xPSR).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct Psr {
    raw: u32,
}

impl Psr {
    /// Returns the current operating mode (bits 4-0).
    #[inline]
    pub fn mode(self) -> CpuMode {
        match CpuMode::try_from(self.raw.bits::<0, 5>() as u8) {
            Ok(mode) => mode,
            Err(err) => invalid_cpu_mode(err.value()),
        }
    }

    #[inline]
    pub fn state(self) -> CpuState {
        CpuState::from(self.raw.bit::<5>())
    }

    /// Checks if fast interrupt requests are disabled.
    #[inline]
    pub fn fiq_disabled(self) -> bool {
        self.raw.bit::<6>()
    }

    /// Checks if regular interrupt requests are disabled.
    #[inline]
    pub fn irq_disabled(self) -> bool {
        self.raw.bit::<7>()
    }

    /// Checks the overflow flag (V).
    #[inline]
    pub fn overflow(self) -> bool {
        self.raw.bit::<28>()
    }

    /// Checks the carry flag (C).
    #[inline]
    pub fn carry(self) -> bool {
        self.raw.bit::<29>()
    }

    /// Checks the zero flag (Z).
    #[inline]
    pub fn zero(self) -> bool {
        self.raw.bit::<30>()
    }

    /// Checks the sign flag (N).
    #[inline]
    pub fn sign(self) -> bool {
        self.raw.bit::<31>()
    }
}

impl fmt::Display for Psr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn interrupt_disabled(disabled: bool) -> &'static str {
            if disabled {
                "disabled"
            } else {
                "enabled"
            }
        }

        write!(
            f,
            "{{ [{raw:#010X}], mode: {mode}, state: {state}, irq: {irq}, fiq: {fiq}, (N={N}, Z={Z}, C={C}, V={V}) }}",
            raw = self.raw,
            mode = self.mode(),
            state = self.state(),
            irq = interrupt_disabled(self.irq_disabled()),
            fiq = interrupt_disabled(self.fiq_disabled()),
            N = self.sign(),
            Z = self.zero(),
            C = self.carry(),
            V = self.overflow(),
        )
    }
}

impl From<bool> for CpuState {
    #[inline]
    fn from(state: bool) -> Self {
        if state {
            CpuState::Thumb
        } else {
            CpuState::Arm
        }
    }
}

impl From<CpuState> for bool {
    #[inline]
    fn from(state: CpuState) -> Self {
        match state {
            CpuState::Arm => false,
            CpuState::Thumb => true,
        }
    }
}

#[inline(never)]
#[cold]
#[track_caller]
fn invalid_cpu_mode(mode: u8) -> ! {
    panic!("invalid cpu mode: {:05b}", mode)
}
