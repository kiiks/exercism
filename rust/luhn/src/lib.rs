/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {    
    if code.len() <= 1 { return false; }

    // Removing whitespaces
    let stripped = code.trim();
    let whitespace_free: String = stripped.chars().filter(|c| !c.is_whitespace()).collect();

    // Ensuring all chars are only numeric
    let only_digit = whitespace_free.chars().all(|c| c.is_numeric());
    if !only_digit { println!("Must only contains digits"); return false; }
    let not_unicode = whitespace_free.as_bytes().iter().all(|d| (0x30..0x40).contains(d));
    if !not_unicode { println!("Must not contain unicode"); return false; }

    // Calculating doubled digits sum
    let mut digits_iter_num = 0;
    let doubled = whitespace_free.chars().rev().enumerate().map(|(i, d)| {
        digits_iter_num += 1;
        let digit = d.to_digit(10).unwrap_or(0);
        if i % 2 != 0 {
            let mut doubled = digit * 2;
            if doubled > 9 { doubled -= 9; }
            return doubled
        } else { return digit; }
    }).collect::<Vec<u32>>();

    // Handling corner case single_zero_with_space_is_invalid
    if digits_iter_num == 1 && code.contains("0") && code.contains(" ") { return false; }

    let sum = doubled.iter().fold(0, |acc, v| acc + v);
    sum % 10 == 0
}