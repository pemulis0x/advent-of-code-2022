use std::fs;
use std::cmp::max;


const FILE_PATH: &str = "input.txt";

fn str_to_motion(s: &str) -> (i32, i32) {
    if s.is_empty() {
        return (0,0)
    }
    let mut parts = s.split(' ');
    let (direction, distance) = (parts.next().unwrap().chars().next().unwrap(), 
                                         parts.next().unwrap().parse::<i32>().unwrap());
    // (x, y)
    match direction {
        'D' => (0, -distance),
        'U' => (0, distance),
        'L' => (-distance, 0),
        'R' => (distance, 0),
        _ => (69, 69)
    }
}


fn main() {
    let file_in = fs::read_to_string(FILE_PATH).expect("missing first half of problem input");
    let motions = file_in.split('\n').map(str_to_motion).collect::<Vec<_>>();
    // println!("{:?}", &motions);

    // (x, y)
    let mut head = (0,0);
    let mut tail = (0,0);
    let mut visited: Vec<(i32,i32)> = Vec::new();

    for motion in &motions {
        let steps = max(motion.0.abs(), motion.1.abs());
        for _ in 0..steps {
            // println!("H{:?}, T{:?}", head, tail);
            if !visited.contains(&tail) {
                visited.push(tail);
            }
            head.0 += motion.0 / max(motion.0.abs(), 1);
            head.1 += motion.1 / max(motion.1.abs(), 1);

            let dx = head.0 - tail.0;
            let dy = head.1 - tail.1;

            if dx.abs() + dy.abs() == 2 {
                tail.0 += dx / 2;
                tail.1 += dy / 2;
            } else if dx.abs() + dy.abs() == 3 {
                tail.0 += if dx > 0 {1} else {-1};
                tail.1 += if dy > 0 {1} else {-1}
            }

        }
    }
    println!("{:?}", visited);
    println!("{:?}", visited.len() + 1);

}
