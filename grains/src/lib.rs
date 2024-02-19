// Defines the `pub fn square` function that takes a parameter `s` of type `u32` and returns a value of type `u64`
pub fn square(s: u32) -> u64 {
    // Check if the value of `s` is within the range 1..=64
    match s {
        // If within the range 1..=64, calculate and return the value 2^(s-1)
        1..=64 => 2u64.pow(s - 1),

        _ => panic!("Square must be between 1 and 64"),
    }
}

// Defines the `pub fn total` function that takes no parameters and returns a value of type `u64`
pub fn total() -> u64 {
    // Calculate the sum of the values obtained by calling the `square` function for each value in the range 1..=64
    (1..=64).map(square).sum()
}
