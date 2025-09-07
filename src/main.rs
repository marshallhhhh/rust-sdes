use sdes::keygen::subkeys;
use sdes::permute;
use sdes::sboxes;

fn main() {
    let plaintext: u16 = 0b00011000;
    let masterkey: u16 = 0b0101111001;

    let (k1, k2) = subkeys(masterkey);

    let ip: u16 = permute::perm_initial(plaintext);

    let s0: u16 = sboxes::sbox_0(0b0100);
    let s1: u16 = sboxes::sbox_1(0b0110);
    println!("master : {:010b}", masterkey);
    println!("k1: {:08b}", k1);
    println!("k2: {:08b}", k2);
    println!("ip: {:08b}", ip);

    println!("s1: {:02b}", s1);     
}
