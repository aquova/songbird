extern crate sdl2;

use sdl2::pixels::Color;

pub fn get_tile_data_1(ram: &[u8]) -> &[u8] {
    &ram[0x8000..0x9000]
}

pub fn get_tile_data_2(ram: &[u8]) -> &[u8] {
    &ram[0x8800..0x9800]
}

pub fn get_tile_view_1(ram: &[u8]) -> &[u8] {
    &ram[0x9800..0x9C00]
}

pub fn get_tile_view_2(ram: &[u8]) -> &[u8] {
    &ram[0x9C00..0xA000]
}

pub fn get_sprite_attributes(ram: &[u8]) -> &[u8] {
    &ram[0xFE00..0xFEA0]
}

pub fn draw_screen(canvas: sdl2::video::Window) {

}

fn parse_sprite_attributes(ram: &[u8], offset: usize) -> (u8, u8, u8, u8) {
    let attrs = get_sprite_attributes(ram);
    let x = attrs[offset];
    let y = attrs[offset + 1];
    let tile_num = attrs[offset + 2];
    let flags = attrs[offset + 3];

    (x, y, tile_num, flags)
}
