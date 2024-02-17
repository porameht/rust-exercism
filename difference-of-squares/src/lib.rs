pub fn square_of_sum(n: u32) -> u32 {
   (1..=n).sum::<u32>().pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..=n).map(|s| s.pow(2)).sum()
}

pub fn difference(n: u32) -> u32 {
    let result = square_of_sum(n) - sum_of_squares(n);
    println!("Difference between square of sum and sum of squares for first {} numbers is {}", n, &result);
    result
}
