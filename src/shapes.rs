pub struct Shape {
    pub row_offset: i32,
    pub col_offset: i32,
    pub width: u32,
    pub height: u32,
    pub pattern: &'static[u8],
}

const GOLDENHEAD_MAP: [u8; 17] = [
    0b00001100, 
    0b00000111,
    0b10000000,
    0b00000000,
    0b11111100,
    0b00011110,
    0b00000000,
    0b00000011,
    0b00110011,
    0b01001011,
    0b00010010,
    0b00000000,
    0b00000000,
    0b00000000,
    0b00110000,
    0b00001100,
    0b00000000,
];

pub const SPACESHIP: Shape = Shape {
    row_offset: -6,
    col_offset: -4,
    width:      10,
    height:     13,
    pattern:    &GOLDENHEAD_MAP
};

const PULSAR_MAP: [u8;22] = [
    0b00111000,
    0b11100000,
    0b00000000,
    0b00100001,
    0b01000011,
    0b00001010,
    0b00011000,
    0b01010000,
    0b10011100,
    0b01110000,
    0b00000000,
    0b00000111,
    0b00011100,
    0b10000101,
    0b00001100,
    0b00101000,
    0b01100001,
    0b01000010,
    0b00000000,
    0b00000011,
    0b10001110,
    0b00000000,
];

pub const PULSAR: Shape = Shape{
    row_offset: -6,
    col_offset: -6,
    width:      13,
    height:     13,
    pattern:    &PULSAR_MAP,
};

const GLIDER_MAP : [u8;2] = [ 
    0b00110101,
    0b10000000,
];

pub const GLIDER: Shape = Shape{
    row_offset: -1,
    col_offset: -1,
    width:      3,
    height:     3,
    pattern:    &GLIDER_MAP,
};

const NEIGHBOR_MAP : [u8;2] = [ 
    0b11110111,
    0b10000000,
];

pub const NEIGHBORS: Shape = Shape{
    row_offset: -1,
    col_offset: -1,
    width:      3,
    height:     3,
    pattern:    &NEIGHBOR_MAP,
};
