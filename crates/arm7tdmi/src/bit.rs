use std::mem;

/// A trait for manipulating bits.
///
/// All functions operate with `0` being the least significant bit.
pub trait BitIndex: Copy {
    /// The number of bits in the type.
    const NBITS: usize;

    /// Checks if a bit is set.
    #[must_use]
    fn bit<const BIT: usize>(self) -> bool;

    /// Sets a bit to `state`.
    #[must_use]
    fn set_bit<const BIT: usize>(self, state: bool) -> Self;

    /// Returns the bits for in the range `START..END`.
    #[must_use]
    fn bits<const START: usize, const END: usize>(self) -> Self;

    /// Sets the value of the bits in the range `START..END`.
    #[must_use]
    fn set_bits<const START: usize, const END: usize>(self, value: Self) -> Self;
}

macro_rules! impl_bits {
    ($($ty:ty),*) => {
        $(
            impl BitIndex for $ty {
                const NBITS: usize = mem::size_of::<Self>() * 8;

                #[inline]
                fn bit<const BIT: usize>(self) -> bool {
                    debug_assert!(BIT < Self::NBITS);

                    self & (1 << BIT) != 0
                }

                #[inline]
                fn set_bit<const BIT: usize>(self, state: bool) -> Self {
                    debug_assert!(BIT < Self::NBITS);

                    let mask = 1 << BIT;
                    if state {
                        self | mask
                    } else {
                        self & !mask
                    }
                }

                #[inline]
                fn bits<const START: usize, const END: usize>(self) -> Self {
                    debug_assert!(START < END);
                    debug_assert!(END <= Self::NBITS);

                    let lsh = Self::NBITS - END;
                    let rsh = lsh + START;

                    (self << lsh) >> rsh
                }

                #[inline]
                fn set_bits<const START: usize, const END: usize>(self, value: Self) -> Self {
                    debug_assert!(START < END);
                    debug_assert!(END <= Self::NBITS);
                    debug_assert_eq!(value >> END, 0);

                    let mask = Self::MAX.bits::<START, END>() << START;

                    (self & !mask) | (value << START)
                }
            }
        )*
    };
}

impl_bits! { u8, u16, u32, u64, u128, usize }
