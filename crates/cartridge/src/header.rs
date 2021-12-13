use std::fmt;
use std::ops::Range;

use crate::util::ascii::Ascii;

/// GBA cartridge header.
///
/// The first 192 bytes at `0x8000000` in ROM are used as cartridge header. The
/// same header is also used for Multiboot images at `0x2000000` (plus some
/// additional multiboot entries at `0x20000C0` and up).
///
/// # Header Overview
///
/// ```text
/// Address  Bytes  Expl.
/// 0x000    4      ROM Entry Point  (32bit ARM branch opcode, eg. "B rom_start")
/// 0x004    156    Nintendo Logo    (compressed bitmap, required!)
/// 0x0A0    12     Game Title       (uppercase ascii, max 12 characters)
/// 0x0AC    4      Game Code        (uppercase ascii, 4 characters)
/// 0x0B0    2      Maker Code       (uppercase ascii, 2 characters)
/// 0x0B2    1      Fixed value      (must be 96h, required!)
/// 0x0B3    1      Main unit code   (00h for current GBA models)
/// 0x0B4    1      Device type      (usually 00h) (bit7=DACS/debug related)
/// 0x0B5    7      Reserved Area    (should be zero filled)
/// 0x0BC    1      Software version (usually 00h)
/// 0x0BD    1      Complement check (header checksum, required!)
/// 0x0BE    2      Reserved Area    (should be zero filled)
/// --- Additional Multiboot Header Entries ---
/// 0x0C0    4      RAM Entry Point  (32bit ARM branch opcode, eg. "B ram_start")
/// 0x0C4    1      Boot mode        (init as 00h - BIOS overwrites this value!)
/// 0x0C5    1      Slave ID Number  (init as 00h - BIOS overwrites this value!)
/// 0x0C6    26     Not used         (seems to be unused)
/// 0x0E0    4      JOYBUS Entry Pt. (32bit ARM branch opcode, eg. "B joy_start")
/// ```
///
/// Note: With all entry points, the CPU is initially set into system mode.
///
/// # Sources
///
/// \[1\]: <https://problemkaputt.de/gbatek.htm#gbacartridgeheader>
#[derive(Clone, Copy, Debug)]
pub struct CartridgeHeader {
    /// Game title (uppercase ASCII, padded with `0x00`).
    pub game_title: Ascii<12>, // 0xA0
    /// Game code (uppercase ASCII).
    pub game_code: Ascii<4>, // 0xAC
    /// Maker code (uppercase ASCII).
    pub maker_code: Ascii<2>, // 0xB0
    /// Software version (usually `0x00`).
    pub software_version: u8, // 0xBC
    /// Header checksum.
    ///
    /// Computed over `0xA0..=0xBC`.
    pub checksum: u8, // 0xBD
}

/// ROMs must have 192 bytes at a minimum.
const HEADER_MIN_SIZE: usize = 0xC0;

const CHECKSUM_OFFSET: usize = 0xBD;
const SOFTWARE_VERSION_OFFSET: usize = 0xBC;

const CHECKSUM_RANGE: Range<usize> = 0xA0..0xBD;
const GAME_TITLE_RANGE: Range<usize> = 0xA0..0xAC;
const GAME_CODE_RANGE: Range<usize> = 0xAC..0xB0;
const MAKER_CODE_RANGE: Range<usize> = 0xB0..0xB2;

/// An error when a ROM header is incomplete.
#[derive(Clone, Copy, Debug)]
pub struct IncompleteHeaderError;

impl fmt::Display for IncompleteHeaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("incomplete ROM header")
    }
}

impl CartridgeHeader {
    /// Parse header information from the first 192 bytes located at
    /// `0x8000000` in ROM.
    pub fn parse(bytes: &[u8]) -> Result<CartridgeHeader, IncompleteHeaderError> {
        if bytes.len() < HEADER_MIN_SIZE {
            return Err(IncompleteHeaderError);
        }

        let checksum = bytes[CHECKSUM_OFFSET];
        let computed_checksum = compute_checksum(&bytes[CHECKSUM_RANGE]);
        if checksum != computed_checksum {
            log::warn!(
                "invalid header checksum: computed {:#04X}, expected {:#04X}",
                computed_checksum,
                checksum,
            );
        }

        // SAFETY: Slice is 12 bytes long.
        let game_title = unsafe { Ascii::<12>::from_bytes(&bytes[GAME_TITLE_RANGE]) };
        // SAFETY: Slice is 4 bytes long.
        let game_code = unsafe { Ascii::<4>::from_bytes(&bytes[GAME_CODE_RANGE]) };
        // SAFETY: Slice is 2 bytes long.
        let maker_code = unsafe { Ascii::<2>::from_bytes(&bytes[MAKER_CODE_RANGE]) };

        let software_version = bytes[SOFTWARE_VERSION_OFFSET];

        Ok(CartridgeHeader {
            game_title,
            game_code,
            maker_code,
            software_version,
            checksum,
        })
    }
}

/// Compute header checksum over `0xA0..=0xBC`.
///
/// # Sources
///
/// \[1\]: <https://problemkaputt.de/gbatek.htm#gbacartridgeheader>
fn compute_checksum(bytes: &[u8]) -> u8 {
    debug_assert_eq!(bytes.len(), CHECKSUM_RANGE.len());

    let mut chk = 0u8;
    for &b in bytes {
        chk = chk.wrapping_sub(b);
    }
    chk.wrapping_sub(0x19)
}

#[cfg(test)]
mod tests {
    use crate::util::test::ROM;

    use super::*;

    #[test]
    fn info() {
        let header = CartridgeHeader::parse(ROM).unwrap();

        assert_eq!(header.game_title, "GBA Tests");
        assert_eq!(header.game_code, "1337");
        assert_eq!(header.maker_code, "JS");
        assert_eq!(header.software_version, 0x00);
        assert_eq!(header.checksum, 0x69);
    }

    #[test]
    fn checksum() {
        let header = CartridgeHeader::parse(ROM).unwrap();

        assert_eq!(header.checksum, compute_checksum(&ROM[CHECKSUM_RANGE]));
    }
}
