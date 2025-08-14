// Include the inv_pyramid function from the previous context to ensure the file remains complete
fn inv_pyramid(v: String, i: u32) -> Vec<String> {
    let mut result = Vec::new();
    
    for n in 1..=i {
        let spaces = " ".repeat((i - n) as usize);
        let symbols = v.repeat(n as usize);
        result.push(format!("{}{}", spaces, symbols));
    }
    
    for n in (1..i).rev() {
        let spaces = " ".repeat((i - n) as usize);
        let symbols = v.repeat(n as usize);
        result.push(format!("{}{}", spaces, symbols));
    }
    
    result
}