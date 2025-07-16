pub fn lucas_number(n: u32) -> u32 {
    match n {
        0 => 2,
        1 => 1,
        _ => {
            let mut a = 2; // L(0)
            let mut b = 1; // L(1)
            let mut c = 0;

            for _ in 2..=n {
                c = a + b;
                a = b;
                b = c;
            }

            c
        }
    }
}
