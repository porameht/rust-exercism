// Declare a public function named `sum_of_multiples` that takes two parameters: `limit` of type `u32` (an unsigned 32-bit integer), and `factors` which is a reference to a slice of `u32`. The function returns a `u32`.
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // Create a range from 1 (inclusive) to `limit` (exclusive). This range represents all the numbers up to `limit` that we're going to check.
    (1..limit)
        // Start a filter operation on the range. For each number `n` in the range, it will check a condition. The `factors.iter()` part starts an iterator over the `factors` slice.
        .filter(|n| factors.iter()
        // Check if any factor `d` in the `factors` slice satisfies the condition that `d` is not zero and `n` is divisible by `d` (i.e., `n` modulo `d` equals zero). If any factor satisfies this condition, the `any` function will return `true` and the number `n` will pass the filter.
        .any(|d| *d != 0 && n % d == 0))
        // Sum up all the numbers that passed the filter. This is the final result of the function.
        .sum()
}