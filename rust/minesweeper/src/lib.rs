static ADJACENTS_WINDOW: [(i32, i32); 8] = [
  (-1, -1), (0, -1), (1, -1),
  (-1,  0),          (1,  0),
  (-1,  1), (0,  1), (1,  1),
];

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let height = minefield.len() as i32;
    (0..height).map(|y| {
        let width = minefield[y as usize].len() as i32;
        (0..width).map(|x| {
            if minefield[y as usize].as_bytes()[x as usize] == b'*' { '*' } 
            else {
                match ADJACENTS_WINDOW.iter()
                    .map(|&(ox, oy)| (x + ox, y + oy))
                    .filter(|&(x, y)| (0 <= x && x < width) && (0 <= y && y < height))
                    .filter(|&(x, y)| minefield[y as usize].as_bytes()[x as usize] == b'*')
                    .count() {
                        0 => ' ',
                        n => (n as u8 + '0' as u8) as char
                    }
            }
        }).collect()
    }).collect()
}