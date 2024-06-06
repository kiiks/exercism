pub fn is_armstrong_number(num: u32) -> bool {
  let i_len = num.checked_ilog10().unwrap_or(0) + 1;
  let digits = num.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect::<Vec<u32>>().into_iter();
  let armstrong_value = digits.fold(0, |acc, x| { 
    match u32::checked_add(acc, x.pow(i_len)) {
      None => 0,
      Some(v) => v 
    }
  });
  return armstrong_value == num
}