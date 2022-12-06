
use std::io;

fn find_marker(chars: &[char], n: usize) -> usize {
    for (i, win) in chars.windows(n).enumerate() {
        let mut buf = String::from("");
        for (j, ch) in win.iter().enumerate() {
            if !buf.contains(*ch) {
                buf.push(*ch);
            }
            if buf.len() == n {
                return i + j + 1
            }
        }
    } 0
}


fn main() -> io::Result<()> {
    let stdin = io::read_to_string(io::stdin())?;

    let chars = &stdin.chars().collect::<Vec<char>>()[..];
    let part_1 = find_marker(chars, 4);
    let part_2 = find_marker(chars, 14);

    println!("part 1:\t{:?}", part_1);
    println!("part 2:\t{:?}", part_2);
    Ok(())
}



