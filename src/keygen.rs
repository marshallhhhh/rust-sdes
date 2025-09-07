use crate::permute::permute; 

pub fn subkeys(key: u16) -> (u16, u16) {
    let k1_table: [usize; 8] =  [0, 6, 8, 3, 7, 2, 9, 5];
    let k2_table: [usize; 8] =  [7, 2, 5, 4, 9, 1, 8, 0];

    let k1: u16 = permute(key,&k1_table,10, 8);
    let k2: u16 = permute(key,&k2_table,10, 8);
    
    return (k1, k2);
}