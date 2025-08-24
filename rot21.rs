// pub fn rot21(input: &str) -> String {
//     input
//         .chars()
//         .map(|c| {
//             if c.is_ascii_alphabetic() {
//                 let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
//                 let shifted = (c as u8 - base + 21) % 26;
//                 (base + shifted) as char
//             } else {
//                 c
//             }
//         })
//         .collect()
// }

// fn main() {
//     println!("The letter \"a\" becomes: {}", rot21("a"));
//     println!("The letter \"m\" becomes: {}", rot21("m"));
//     println!("The word \"MISS\" becomes: {}", rot21("MISS"));
//     println!("Your cypher wil be: {}", rot21("Testing numbers 1 2 3"));
//     println!("Your cypher wil be: {}", rot21("rot21 works!"));
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_single_lowercase_letters() {
//         assert_eq!(rot21("a"), "v");
//         assert_eq!(rot21("m"), "h");
//         assert_eq!(rot21("z"), "u");
//     }

//     #[test]
//     fn test_single_uppercase_letters() {
//         assert_eq!(rot21("A"), "V");
//         assert_eq!(rot21("M"), "H");
//         assert_eq!(rot21("Z"), "U");
//     }

//     #[test]
//     fn test_uppercase_word() {
//         assert_eq!(rot21("MISS"), "HDNN");
//     }

//     #[test]
//     fn test_mixed_case_with_numbers() {
//         assert_eq!(rot21("Testing numbers 1 2 3"), "Oznodib iphwzmn 1 2 3");
//     }

//     #[test]
//     fn test_with_punctuation() {
//         assert_eq!(rot21("rot21 works!"), "mjo21 rjmfn!");
//     }

//     #[test]
//     fn test_empty_string() {
//         assert_eq!(rot21(""), "");
//     }

//     #[test]
//     fn test_only_numbers() {
//         assert_eq!(rot21("12345"), "12345");
//     }

//     #[test]
//     fn test_only_punctuation() {
//         assert_eq!(rot21("!@#$%"), "!@#$%");
//     }

//     #[test]
//     fn test_alphabet_rotation() {
//         // Test that each letter shifts correctly
//         let alphabet = "abcdefghijklmnopqrstuvwxyz";
//         let expected = "vwxyzabcdefghijklmnopqrstu";
//         assert_eq!(rot21(alphabet), expected);
//     }

//     #[test]
//     fn test_uppercase_alphabet_rotation() {
//         let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
//         let expected = "VWXYZABCDEFGHIJKLMNOPQRSTU";
//         assert_eq!(rot21(alphabet), expected);
//     }

//     #[test]
//     fn test_mixed_content() {
//         assert_eq!(rot21("Hello, World! 123"), "Czggj, Rjmgy! 123");
//     }

//     #[test]
//     fn test_spaces_preserved() {
//         assert_eq!(rot21("a b c"), "v w x");
//     }
// }
pub fn rot21(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            'a'..='z' => (((c as u8 - b'a' + 21) % 26) + b'a') as char,
            'A'..='Z' => (((c as u8 - b'A' + 21) % 26) + b'A') as char,
            _ => c,
        })
        .collect()
}
