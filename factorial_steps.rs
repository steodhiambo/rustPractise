pub fn count_factorial_steps(factorial: u64) -> u64 {
    if factorial <= 1 {
        return 0;
    }

    let mut product = 1;
    let mut i = 1;

    while product < factorial {
        i += 1;
        product *= i;
    }

    if product == factorial {
        return i;
    }

    0
}
