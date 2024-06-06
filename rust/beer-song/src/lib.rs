pub fn verse(n: u32) -> String {
    match n {
      0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
      1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
      2 => "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_string(),
      _ => format!("{n} bottles of beer on the wall, {n} bottles of beer.\nTake one down and pass it around, {next} bottles of beer on the wall.\n", n = n, next = n-1),
    }
}

pub fn sing(start: u32, end: u32) -> String {
  // NB1: game logic match the range pattern `start > end` but range doesn't work with this pattern.
  // So we need to create a valid range with `end..start`, then reverse iteration order with `rev()`.
  // NB2: `fold()` init accumulator with `verse(start)` value, thus we ignore the first iteration with `start - 1`.
  (end..=(start - 1))
    .rev()
    .fold(verse(start), |acc, n| acc + "\n" + &verse(n))
}


