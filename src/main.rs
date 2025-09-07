use sdes::keygen::subkeys;
use sdes::permute;

fn main() {
    let plaintext: u16 = 0b00011000;
    let masterkey: u16 = 0b0101111001;

    let (k1, k2) = subkeys(masterkey);

    let ip: u16 = permute::perm_initial(plaintext);

    println!("master : {:010b}", masterkey);
    println!("k1: {:08b}", k1);
    println!("k2: {:08b}", k2);
    println!("ip: {:08b}", ip);
}
