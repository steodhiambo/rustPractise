pub fn check_ms(message: &str) -> Result<&str, &str> {
    if message.is_empty() || message.contains("stupid") {
        Err("ERROR: illegal")
    } else {
        Ok(message)
    }
}

fn main() {
    ["hello there", "", "you are stupid", "stupid"]
        .into_iter()
        .for_each(|m| println!("{:?}", check_ms(m)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_message() {
        assert_eq!(check_ms("hello there"), Ok("hello there"));
        assert_eq!(check_ms("this is a good message"), Ok("this is a good message"));
        assert_eq!(check_ms("nice weather today"), Ok("nice weather today"));
    }

    #[test]
    fn test_empty_message() {
        assert_eq!(check_ms(""), Err("ERROR: illegal"));
    }

    #[test]
    fn test_message_with_stupid() {
        assert_eq!(check_ms("you are stupid"), Err("ERROR: illegal"));
        assert_eq!(check_ms("stupid"), Err("ERROR: illegal"));
        assert_eq!(check_ms("that's stupid behavior"), Err("ERROR: illegal"));
        assert_eq!(check_ms("stupid people everywhere"), Err("ERROR: illegal"));
    }

    #[test]
    fn test_case_sensitivity() {
        // The function should be case-sensitive based on the requirements
        assert_eq!(check_ms("you are STUPID"), Ok("you are STUPID"));
        assert_eq!(check_ms("Stupid"), Ok("Stupid"));
        assert_eq!(check_ms("STUPID"), Ok("STUPID"));
    }

    #[test]
    fn test_partial_matches() {
        // Should not block words that contain "stupid" as a substring
        assert_eq!(check_ms("stupidity"), Ok("stupidity"));
        assert_eq!(check_ms("stupidly"), Ok("stupidly"));
        assert_eq!(check_ms("astupid"), Ok("astupid"));
    }

    #[test]
    fn test_whitespace_only() {
        // Whitespace-only messages should be allowed (not empty)
        assert_eq!(check_ms(" "), Ok(" "));
        assert_eq!(check_ms("   "), Ok("   "));
        assert_eq!(check_ms("\t"), Ok("\t"));
        assert_eq!(check_ms("\n"), Ok("\n"));
    }
}
