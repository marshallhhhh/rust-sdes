use crate::permute;
use crate::sboxes;
use crate::keygen::subkeys;

pub fn encrypt(input: u16, key: u16) -> u16 {
    let (k1, k2) = subkeys(key);

    // Initial permutation
    let initial_perm: u16 = permute::perm_initial(input);

    // feistel steps
    let round1 = feistel(initial_perm, k1);
    let swapped = ((round1 & 0b1111) << 4) | (round1 >> 4);
    let round2 = feistel(swapped, k2);

    return permute::perm_inverse(round2);
}

pub fn decrypt(input: u16, key: u16) -> u16 {
    let (k1, k2) = subkeys(key);

    // Initial permutation
    let initial_perm: u16 = permute::perm_initial(input);

    // feistel steps
    let round1 = feistel(initial_perm, k2);
    let swapped = ((round1 & 0b1111) << 4) | (round1 >> 4);
    let round2 = feistel(swapped, k1);

    return permute::perm_inverse(round2);
}

fn feistel(input: u16, subkey: u16) -> u16 {
    let left = input >> 4;
    let right = input & 0b00001111;
    
    let f = mixing_function(right, subkey);
    let new_left = left ^ f;

    return (new_left << 4) | right;
}

fn mixing_function(input: u16, k: u16) -> u16 {
    let expanded = permute::perm_expansion4(input);

    let xor = expanded ^ k;

    let left = permute::permute(xor, &[0, 1, 2, 3], 8, 4);
    let right = permute::permute(xor, &[4, 5, 6, 7], 8, 4);

    let s0 = sboxes::sbox_0(left);
    let s1 = sboxes::sbox_1(right);

    let sboxes_joined = (s0 << 2) | s1;

    return permute::perm_4(sboxes_joined);
}