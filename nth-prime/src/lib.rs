pub fn nth(n: u32) -> u32 {
    if n == 0 { return 2; }

    let mut count = 1;
    let mut num = 1;

    while count <= n {
        num += 2;
        if is_prime(num) {
            count += 1;
        }
    }

    num
}

fn is_prime(num: u32) -> bool {
    for i in 2..=(num as f64).sqrt() as u32 {
        if num % i == 0 {
            return false;
        }
    }
    true
}