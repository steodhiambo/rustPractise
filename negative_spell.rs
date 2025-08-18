pub fn negative_spell(n: i64) -> String {
    if n > 0 {
        return "error: positive number".to_string();
    }
    if n == 0 {
        return "zero".to_string();
    }
    format!("minus {}", spell(-n))
}
fn spell(n: i64) -> String {
    const SMALL: [&str; 20] = [
        "zero","one","two","three","four","five","six","seven","eight","nine",
        "ten","eleven","twelve","thirteen","fourteen","fifteen","sixteen",
        "seventeen","eighteen","nineteen"
    ];
    const TENS: [&str; 10] = [
        "","", "twenty","thirty","forty","fifty","sixty","seventy","eighty","ninety"
    ];

    match n {
        0..=19 => SMALL[n as usize].to_string(),
        20..=99 => {
            let t = TENS[(n / 10) as usize];
            if n % 10 == 0 { t.to_string() }
            else { format!("{}-{}", t, SMALL[(n % 10) as usize]) }
        }
        100..=999 => {
            let h = format!("{} hundred", SMALL[(n / 100) as usize]);
            if n % 100 == 0 { h }
            else { format!("{} {}", h, spell(n % 100)) }
        }
        1_000..=999_999 => {
            let th = format!("{} thousand", spell(n / 1_000));
            if n % 1_000 == 0 { th }
            else { format!("{} {}", th, spell(n % 1_000)) }
        }
        1_000_000 => "one million".to_string(),
        _ => String::new(), // won't happen here
    }
}
