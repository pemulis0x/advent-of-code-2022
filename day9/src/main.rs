use std::fs;
use std::cmp::max;


// will clean this up in a few hours. 
// pt 1 and pt 2 should not have so much code duplication obviously,
// but this is the mvp solution


const FILE_PATH: &str = "input.txt";

// type Point = (i32, i32);

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

// sets tail position appropriately based on the location of 'head'
fn follow_head(head: (i32, i32), tail: &mut (i32, i32)) {
    let dx = head.0 - tail.0;
    let dy = head.1 - tail.1;
    let ab_dist = dx.abs() + dy.abs(); 

    if ab_dist == 2 || ab_dist == 4 {
        tail.0 += dx / 2;
        tail.1 += dy / 2;
    } else if ab_dist == 3 {
        tail.0 += if dx > 0 {1} else {-1};
        tail.1 += if dy > 0 {1} else {-1}
    } 
}

fn main() {
    let file_in = fs::read_to_string(FILE_PATH).expect("missing first half of problem input");
    let motions = file_in.split('\n').map(str_to_motion).collect::<Vec<_>>();
    // println!("{:?}", &motions);

    // (x, y)
    let mut head = (0,0);
    let mut tail = (0,0);
    let mut pt_1 = Vec::from([(0,0)]);

    //pt 1
    for motion in &motions {
        let steps = max(motion.0.abs(), motion.1.abs());
        for _ in 0..steps {
            
            head.0 += motion.0 / max(motion.0.abs(), 1);
            head.1 += motion.1 / max(motion.1.abs(), 1);

            follow_head(head, &mut tail);

            if !pt_1.contains(&tail) {
                pt_1.push(tail);
            }
        }
    }

    //pt 2
    let mut rope = Vec::new();
    for _ in 0..10 { rope.push((0,0)) }
    let mut pt_2 = Vec::from([(0,0)]); //places rope[10] has visited

    for motion in &motions {
        let steps = max(motion.0.abs(), motion.1.abs());
        for _ in 0..steps {
            {
                let head = &mut rope[0];
                head.0 += motion.0 / max(motion.0.abs(), 1);
                head.1 += motion.1 / max(motion.1.abs(), 1);
            }

            for i in 0..rope.len() - 1 {
                follow_head(rope[i], &mut rope[i+1]);
            }
            
            //println!("{:?}", rope);

            let tail = rope[rope.len()-1];
            // println!("{:?}", tail);
            if !pt_2.contains(&tail) {
                pt_2.push(tail);
            }
        }
    }

    println!("{:?}", pt_1.len());
    println!("{:?}", pt_2.len());

}





