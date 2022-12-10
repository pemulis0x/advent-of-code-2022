use std::fs;
use std::collections::VecDeque;

const FILE_PATH:  &str   = "input.txt";
const CRT_WIDTH:  usize  = 40;
const CRT_LIT:    char   = '#';
const CRT_DARK:   char   = '.';

#[derive(Debug)]
enum Instruction {
    Addx(i32),
    Noop
}

struct Crt {
    state: String,
    width: usize,
}

impl Crt {
    fn draw(&self) {
        let mut buf = String::new();
        for (i, ch) in self.state.char_indices() {
            buf.push(ch);
            if (i + 1) % self.width == 0 {
                println!("{}", buf);
                buf = "".to_string();
            }
        }
        println!("{}", buf);
    }

    // # # # 
    // . . _
    fn update(&mut self, sprite_pos: i32) {
        let impl_cursor = self.state.len() as i32 % self.width as i32;
        if (sprite_pos - impl_cursor).abs() < 2 {
            self.state.push(CRT_LIT);
        } else {
            self.state.push(CRT_DARK);
        }
    }
}

fn to_instruction(s: &str) -> Instruction {
    if s.is_empty() {
        return Instruction::Noop
    }
    if s[0..4].contains("noop") {
        return Instruction::Noop
    } 
    
    Instruction::Addx(s.split(' ').last().unwrap().parse::<i32>().unwrap())
}

fn exec(i: Instruction, r: i32) -> i32 {
    match i {
        Instruction::Addx(data) => r + data,
        Instruction::Noop => r,
    }
}

fn main() {
    let stdin = fs::read_to_string(FILE_PATH).expect("missing instructions");
    let instructions = stdin.split('\n').map(to_instruction).collect::<Vec<_>>();

    let mut exec_queue = VecDeque::new();
    let mut register = 1;

    let debug_cycles = vec![20usize, 60usize, 100usize, 140usize, 180usize, 220usize];
    let mut snapshots = Vec::new();

    let mut my_crt = Crt {
        state: "".to_string(),
        width: CRT_WIDTH,
    };

    let mut cycle = 0;
    for instruction in instructions {
    
        cycle += 1;
        // process for cycle n
        if let Some(queued) = exec_queue.pop_front() {
            register = exec(queued, register);
        }

        my_crt.update(register);
        if debug_cycles.contains(&cycle) {
            snapshots.push((cycle, register));
        }

        // add to execution queue
        match instruction {
            Instruction::Addx(_) => {
                exec_queue.push_back(Instruction::Noop);
                exec_queue.push_back(instruction);
            },
            Instruction::Noop => exec_queue.push_back(instruction)
        }
        // println!("{:?}", register);
    }

    for remaining in exec_queue {
        // println!("{:?}", cycle);
        cycle += 1;
        register = exec(remaining, register);
        my_crt.update(register);

        if debug_cycles.contains(&cycle) {
            snapshots.push((cycle, register));
        }

    }
    my_crt.state.pop();

    // println!("{:?}", snapshots);
    let pt_1: i32 = snapshots.iter().map(|(c, r)| *c as i32 * r).sum();
    println!("{:?}", pt_1);
    // println!("{:?}", register);
    my_crt.draw();
}





