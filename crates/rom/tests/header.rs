use rom::RomHeader;

static ROM: &[u8] = include_bytes!("../../../test-roms/jsmolka/ppu/stripes.gba");

#[test]
fn header_info() {
    let header = RomHeader::parse(ROM).unwrap();

    assert_eq!(header.game_title, "GBA Tests");
    assert_eq!(header.game_code, "1337");
    assert_eq!(header.maker_code, "JS");
    assert_eq!(header.software_version, 0x00);
    assert_eq!(header.checksum, 0x69);
}
