#[derive(PartialEq, Eq, Debug)]
pub enum PrimeErr {
    Even,
    Divider(u32),
}

pub fn prime_checker(nb: u32) -> Option<Result<u32, PrimeErr>> {
    // Return None if the number is less than or equal to 1
    if nb <= 1 {
        return None;
    }

    // 2 is the only even prime number
    if nb == 2 {
        return Some(Ok(2));
    }

    // All other even numbers are not prime
    if nb % 2 == 0 {
        return Some(Err(PrimeErr::Even));
    }

    // Check for odd divisors up to sqrt(nb)
    // We only need to check up to the square root because if nb has a divisor
    // greater than sqrt(nb), it must also have a corresponding divisor less than sqrt(nb)
    let mut i = 3;
    while i * i <= nb {
        if nb % i == 0 {
            return Some(Err(PrimeErr::Divider(i)));
        }
        i += 2;
    }

    // If no divisors found, the number is prime
    Some(Ok(nb))
}

fn main() {
    println!("Is {} prime? {:?}", 2, prime_checker(2));
    println!("Is {} prime? {:?}", 14, prime_checker(14));
    println!("Is {} prime? {:?}", 2147483647, prime_checker(2147483647));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numbers_less_than_or_equal_to_one() {
        assert_eq!(prime_checker(0), None);
        assert_eq!(prime_checker(1), None);
    }

    #[test]
    fn test_prime_number_two() {
        assert_eq!(prime_checker(2), Some(Ok(2)));
    }

    #[test]
    fn test_even_numbers() {
        assert_eq!(prime_checker(4), Some(Err(PrimeErr::Even)));
        assert_eq!(prime_checker(14), Some(Err(PrimeErr::Even)));
        assert_eq!(prime_checker(100), Some(Err(PrimeErr::Even)));
    }

    #[test]
    fn test_odd_composite_numbers() {
        assert_eq!(prime_checker(9), Some(Err(PrimeErr::Divider(3))));
        assert_eq!(prime_checker(15), Some(Err(PrimeErr::Divider(3))));
        assert_eq!(prime_checker(21), Some(Err(PrimeErr::Divider(3))));
        assert_eq!(prime_checker(25), Some(Err(PrimeErr::Divider(5))));
        assert_eq!(prime_checker(35), Some(Err(PrimeErr::Divider(5))));
    }

    #[test]
    fn test_prime_numbers() {
        assert_eq!(prime_checker(3), Some(Ok(3)));
        assert_eq!(prime_checker(5), Some(Ok(5)));
        assert_eq!(prime_checker(7), Some(Ok(7)));
        assert_eq!(prime_checker(11), Some(Ok(11)));
        assert_eq!(prime_checker(13), Some(Ok(13)));
        assert_eq!(prime_checker(17), Some(Ok(17)));
        assert_eq!(prime_checker(19), Some(Ok(19)));
        assert_eq!(prime_checker(23), Some(Ok(23)));
    }

    #[test]
    fn test_large_prime() {
        // 2147483647 is a known large prime (2^31 - 1, Mersenne prime)
        assert_eq!(prime_checker(2147483647), Some(Ok(2147483647)));
    }

    #[test]
    fn test_large_composite() {
        // Test a large composite number
        assert_eq!(prime_checker(2147483646), Some(Err(PrimeErr::Even)));
        assert_eq!(prime_checker(2147483645), Some(Err(PrimeErr::Divider(3))));
    }

    #[test]
    fn test_smallest_divider_returned() {
        // 77 = 7 * 11, should return 7 as the smallest divider
        assert_eq!(prime_checker(77), Some(Err(PrimeErr::Divider(7))));

        // 91 = 7 * 13, should return 7 as the smallest divider
        assert_eq!(prime_checker(91), Some(Err(PrimeErr::Divider(7))));
    }
}