/// generates a permutation of the input based on the provided permutation table
/// 
/// # Examples
/// ```
/// let perm_4: [usize: 4] = [1, 3, 2, 0];
/// p4 = permute(0b1010, p4);
/// ```
pub fn permute(input: u16, table: &[usize], input_bits: usize, output_bits: usize) -> u16 {
    let mut output: u16 = 0;
    if input == 0 { return 0; }

    for (i, &pos) in table.iter().enumerate() {
        let mask: u16 = 1 << (input_bits - 1 - pos);
        if (mask & input) != 0 {
            output |= 1 << output_bits - 1 - i;
        }
    }

    return output;
}

pub fn perm_4(input: u16) -> u16 {
    let p4_table: [usize; 4] = [1,3,2,0];
    return permute(input, &p4_table, 4, 4);
}

pub fn perm_expansion4(input: u16) -> u16 {
    let expansion_table: [usize; 8] = [3, 0, 1, 2, 1, 2, 3, 0];
    return permute(input, &expansion_table, 4, 8);
}

pub fn perm_initial(input: u16) -> u16 {
    let table: [usize; 8] = [1, 5, 2, 0, 3, 7, 4, 6];
    return permute(input, &table, 8, 8);
}

pub fn perm_inverse(input: u16) -> u16 {
    let table: [usize; 8] = [3, 0, 2, 4, 6, 1, 7, 5];
    return permute(input, &table, 8, 8);
}
