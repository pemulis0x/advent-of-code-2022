
use std::io;
use std::collections::HashMap;


// i hope this solution isn't reflective of my personality
fn solve(stdin: String, part: i32) -> i32 {
    
    let mut outcomes = HashMap::new();
    // A = rock, b = paper, c = scissors (opponent)
    // X = rock, Y = paper, Z = scissors (you, pt 1)
    // X = lose, Y = draw, Z = win       (you, pt 2)
    // rock = 1, paper = 2, scissors = 3 (extra pts)
    if part == 1 {
        outcomes.insert(('A', 'X'), 4);
        outcomes.insert(('A', 'Y'), 8);
        outcomes.insert(('A', 'Z'), 3);
        outcomes.insert(('B', 'X'), 1);
        outcomes.insert(('B', 'Y'), 5);
        outcomes.insert(('B', 'Z'), 9);
        outcomes.insert(('C', 'X'), 7);
        outcomes.insert(('C', 'Y'), 2);
        outcomes.insert(('C', 'Z'), 6);
    } else {
        outcomes.insert(('A', 'X'), 3); 
        outcomes.insert(('A', 'Y'), 4);
        outcomes.insert(('A', 'Z'), 8);
        outcomes.insert(('B', 'X'), 1);
        outcomes.insert(('B', 'Y'), 5);
        outcomes.insert(('B', 'Z'), 9);
        outcomes.insert(('C', 'X'), 2);
        outcomes.insert(('C', 'Y'), 6);
        outcomes.insert(('C', 'Z'), 7);
    }

    // stdin looks like "C X\nC X\nC X\nA Z\nC X\nC Z\nC X\nB Y\nC X\nC X\nC X\nB Y\nC X"
    let mut total = 0;
    let mut current_move: (char, char) = ('🥵', '🥵');
    let mut my_move = false;
    for ch in stdin.chars() {
        if ch != ' ' {
            if my_move {
                current_move.1 = ch;
                total += outcomes.get(&current_move).unwrap(); 
                my_move = false;
            } else if ch != '\n' {
               current_move.0 = ch; 
               my_move = true;
            }
        }
    }
    total
}

fn main() -> io::Result<()> {
    let stdin = io::read_to_string(io::stdin())?;

    println!("part 1: {:?}", solve(stdin.clone(), 1));
    println!("part 2: {:?}", solve(stdin, 2));

    Ok(())
}
