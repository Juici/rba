use crate::header::{CartridgeHeader, HeaderError};
use crate::Cartridge;

/// An error loading a cartridge.
#[derive(Clone, Copy, Debug, thiserror::Error)]
pub enum LoadError {
    /// An error parsing the cartridge header.
    #[error(transparent)]
    HeaderError(#[from] HeaderError),
}

impl Cartridge {
    pub fn load_from_bytes<B: Into<Box<[u8]>>>(rom: B) -> Result<Cartridge, LoadError> {
        let rom = rom.into();
        let header = CartridgeHeader::parse(&rom)?;

        Ok(Cartridge { header, rom })
    }
}
