pub fn scytale_decoder(s: String, letters_per_turn: u32) -> Option<String> {
    // Return None if the message is empty or letters_per_turn is 0
    if s.is_empty() || letters_per_turn == 0 {
        return None;
    }

    let letters_per_turn = letters_per_turn as usize;
    let chars: Vec<char> = s.chars().collect();
    let total_chars = chars.len();

    // Calculate number of columns needed
    let num_cols = (total_chars + letters_per_turn - 1) / letters_per_turn;

    let mut result = Vec::new();

    // Read row by row
    for row in 0..letters_per_turn {
        for col in 0..num_cols {
            let index = col * letters_per_turn + row;
            if index < total_chars {
                result.push(chars[index]);
            }
        }
    }

    Some(result.into_iter().collect())
}

fn main() {
    println!("\"sec yCtoadle\" size=2 -> {:?}",
        scytale_decoder("sec yCtoadle".to_string(), 2));

    println!("\"steoca dylCe\" size=4 -> {:?}",
        scytale_decoder("steoca dylCe".to_string(), 4));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        assert_eq!(scytale_decoder("".to_string(), 2), None);
    }

    #[test]
    fn test_zero_letters_per_turn() {
        assert_eq!(scytale_decoder("hello".to_string(), 0), None);
    }

    #[test]
    fn test_example_case_1() {
        // "sec yCtoadle" with letters_per_turn=2 should decode to "scytale Code"
        assert_eq!(
            scytale_decoder("sec yCtoadle".to_string(), 2),
            Some("scytale Code".to_string())
        );
    }

    #[test]
    fn test_example_case_2() {
        // "steoca dylCe" with letters_per_turn=4 should decode to "scytale Code"
        assert_eq!(
            scytale_decoder("steoca dylCe".to_string(), 4),
            Some("scytale Code".to_string())
        );
    }

    #[test]
    fn test_single_character() {
        assert_eq!(
            scytale_decoder("a".to_string(), 1),
            Some("a".to_string())
        );
    }

    #[test]
    fn test_letters_per_turn_equals_length() {
        // When letters_per_turn equals the string length, it should return the same string
        assert_eq!(
            scytale_decoder("hello".to_string(), 5),
            Some("hello".to_string())
        );
    }

    #[test]
    fn test_simple_case() {
        // "acbd" with letters_per_turn=2
        // Cipher arranged as: a c
        //                     b d
        // Should decode to "abcd"
        assert_eq!(
            scytale_decoder("acbd".to_string(), 2),
            Some("abcd".to_string())
        );
    }
}