pub mod permute;
pub mod keygen;
pub mod sboxes;
pub mod sdes;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn encrypt(plaintext: u16, key: u16) -> u16 {
    sdes::encrypt(plaintext, key)
}

#[wasm_bindgen]
pub fn decrypt(ciphertext: u16, key: u16) -> u16 {
    sdes::decrypt(ciphertext, key)
}