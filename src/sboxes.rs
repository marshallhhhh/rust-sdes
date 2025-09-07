use crate::permute::permute;

pub fn sbox_0(input: u16) -> u16 {

    let row: u16 = permute(input, &[0,3],4, 2);
    let col: u16 = permute(input, &[1, 2], 4, 2);

    let s0: [[u16; 4]; 4] = [
        [0b01, 0b00, 0b11, 0b10],
        [0b11, 0b10, 0b01, 0b00],
        [0b00, 0b10, 0b01, 0b11],
        [0b11, 0b01, 0b11, 0b01]
    ];

    return s0[row as usize][col as usize];
}

pub fn sbox_1(input: u16) -> u16 {

    let row: u16 = permute(input, &[0,3],4, 2);
    let col: u16 = permute(input, &[1, 2], 4, 2);

    let s1: [[u16; 4]; 4] = [
        [0b00, 0b01, 0b10, 0b11],
        [0b10, 0b00, 0b01, 0b11],
        [0b11, 0b00, 0b01, 0b00],
        [0b10, 0b01, 0b00, 0b11]
    ];

    return s1[row as usize][col as usize];
}