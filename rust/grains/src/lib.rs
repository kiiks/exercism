pub fn square(s: u32) -> u64 {
    if !(1..=64).contains(&s) { panic!("Square must be between 1 and 64") }
    let mut count = 1;
    for _i in 1..s {
        count *= 2
    }
    count
}

pub fn total() -> u64 {
    let mut sum: u64 = 0;
    for d in 1..=64 {
        sum += square(d);
    }
    sum
}