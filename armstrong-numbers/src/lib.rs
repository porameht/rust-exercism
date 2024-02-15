pub fn is_armstrong_number(num: u64) -> bool {
    let num_len = num.to_string().len() as u32;
    num == num.to_string()
        .chars()
        .fold(0, |t, x| (x.to_digit(10).unwrap() as u64).pow(num_len) + t)
}