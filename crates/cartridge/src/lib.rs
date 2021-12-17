mod header;
mod load;
mod util;

pub use crate::header::{CartridgeHeader, HeaderError};

/// GBA cartridge.
#[derive(Clone, Debug)]
pub struct Cartridge {
    pub header: CartridgeHeader,
    rom: Box<[u8]>,
}
