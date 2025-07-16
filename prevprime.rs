pub fn prev_prime(nbr: u64) -> u64 {
    for n in (2..=nbr).rev() {
        if is_prime(n) {
            return n;
        }
    }
    0
}

fn is_prime(n: u64) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}
fn main() {
    println!("The previous prime number before 34 is: {}", prev_prime(34)); // 31
    println!("The previous prime number before 2 is: {}", prev_prime(2));   // 2
    println!("The previous prime number before 1 is: {}", prev_prime(1));   // 0
}
