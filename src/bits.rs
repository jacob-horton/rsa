// Get the bits of a number, MSB first
pub fn get_bits(n: u32) -> Vec<bool> {
    let mut num = n;
    let mut bits = Vec::new();
    while num != 0 {
        // Same as: `let bit = num % 1;`
        let bit = num & 1;
        num /= 2;

        bits.push(bit == 1);
    }

    // Went from LSB to MSB, we want MSB first
    bits.reverse();
    return bits;
}
