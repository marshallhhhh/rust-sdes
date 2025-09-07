use sdes::sdes::{encrypt, decrypt};
fn main() {
    let plaintext: u16 = 0b00011000;
    let masterkey: u16 = 0b0101111001;

    let ciphertext = encrypt(plaintext, masterkey);
    let decrypted = decrypt(ciphertext, masterkey);
    println!("plaintext: {:08b}", plaintext);
    println!("ciphertext: {:08b}", ciphertext);
    println!("decrypted: {:08b}", decrypted);
}

