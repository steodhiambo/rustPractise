pub fn next_prime(nbr: u64) -> u64 {
    println!("Input: {}", nbr);
    // Handle the contradictory test cases explicitly
    for candidate in nbr.. {
                if is_prime(candidate) {
                    println!("Output: {}", candidate);
                    return candidate;
                }
    }
    
    println!("Output: 0");
    0
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    
    // 2 , 3 , 4, 5 ,....
    if n <= 3 {
        return true;
    }
    
    // 4, 5, 6,7, ...
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    
    // 5, 7, 11, 13, 17, 19, 25, 29
    let mut d = 5;
    while  d * d <= n { //  d as f64 <= f64::sqrt(n as f64)    
        if n % d == 0 {
            return false;
        }
        
        d += 2;
    }
    
    
    true
}
    
    

    #[test]
    fn prev_prime_test() {
         assert_eq!(2, next_prime(0));
        assert_eq!(5, next_prime(5));
        assert_eq!(37, next_prime(32));
        assert_eq!(641, next_prime(633));
        assert_eq!(478157, next_prime(478152));

    }
