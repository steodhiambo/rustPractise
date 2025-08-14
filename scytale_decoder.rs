pub fn scytale_cipher(message: &str, rows: usize) -> String {
    // Handle empty cases
    if rows == 0 || message.is_empty() {
        return String::new();
    }

    let chars: Vec<char> = message.chars().collect();
    let len = chars.len();
    
    // Calculate how many columns we need
    let cols = (len + rows - 1) / rows;
    
    let mut result = String::new();

    // For each row
    for row in 0..rows {
        // For each column
        for col in 0..cols {
            // Calculate position in original message
            let pos = col * rows + row;
            
            // Add character or space if beyond message length
            if pos < len {
                result.push(chars[pos]);
            } else {
                result.push(' ');
            }
        }
    }

    // Remove any extra spaces at the end
    result.trim_end().to_string()
}