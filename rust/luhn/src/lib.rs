/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // TODO: Find better explanation of luhn algo (difference between sources)
    
    if code.len() <= 1 { return false; }
    let stripped = code.trim();
    let whitespace_free: String = stripped.chars().filter(|c| !c.is_whitespace()).collect();
    let only_digit = whitespace_free.as_bytes().iter().all(|d| (0x30..0x40).contains(d));
    println!("original: {}", code);
    if !only_digit { println!("NOT only digits"); return false; }
    println!("only digits");
    println!("whitespace_free: {}", whitespace_free);
    let doubled = whitespace_free.chars().enumerate().map(|(i, d)| {
        let digit = d.to_digit(10).unwrap_or(0);
        if i % 2 != 0 {
            println!("double {}", digit);
            let mut doubled = digit * 2;
            if doubled > 9 { doubled -= 9; }
            return doubled
        } else { return digit; }
    }).collect::<Vec<u32>>();
    let sum = doubled.iter().fold(0, |acc, v| acc + v);
    
    println!("sum: {}", sum);

    sum != 0 && sum % 10 != 0
}