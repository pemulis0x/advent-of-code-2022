use std::io;
use std::fs;
extern crate regex;

const INPUT_CRATE_LEN: usize = 4;
const FILE_PATH: &str = "crates.txt";


// reverses initial columns and transposes a matrix
fn rev_transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).rev().collect::<Vec<T>>())
        .collect()
}

// removes "crates" which are '   '
fn drop_empty(v: &Vec<char>) -> Vec<char> {
    let mut buf = Vec::new();
    for ch in v {
        if ch != &' ' {
            buf.push(*ch);
        }
    } buf
}

// converts stdin string to vec of chars, where "[A] " -> 'A'
fn str_to_vchar(s: String) -> Vec<char> {
    let mut buf = Vec::new();
    let slice = &s.chars().collect::<Vec<char>>()[..];
    let iter = slice.chunks(INPUT_CRATE_LEN); 

    for ch in iter{
        buf.push(ch[1]);
    } buf
}

// uses regex to convert "move 1 from 3 to 2" to (1, 3, 2)
// shoutout gpt3 for doing the basic regex for my lazy self
fn txt_to_instruction(s: String) -> (usize, usize, usize) {
    let re = regex::Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let c = re.captures(&s).unwrap();

    (c[1].parse::<usize>().unwrap(), 
     c[2].parse::<usize>().unwrap(),
     c[3].parse::<usize>().unwrap())
}

// factorio inserter
fn move_crates_9000(instr: &(usize, usize, usize), crates: &mut [Vec<char>]) {
    for _ in 0..instr.0 {
        let buf = crates[instr.1 - 1].pop();
        if let Some(ch) = buf {
            crates[instr.2 - 1].push(ch);
        }
    }
}

// factorio stack inserter
fn move_crates_9001(instr: &(usize, usize, usize), crates: &mut [Vec<char>]) {
    let mut stack = Vec::new();
    for _ in 0..instr.0 {
        let buf = crates[instr.1 - 1].pop();
        if let Some(ch) = buf {
            stack.push(ch);
        }
    }
    stack.reverse();
    crates[instr.2 - 1].extend_from_slice(&stack);
}

// expects instructions as stdin, i.e. run with pbpaste | cargo r
// also expects starting crate array as a file FILE_PATH
fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let instructions: Vec<(usize, usize, usize)> = stdin.lines().map(|l| l.unwrap()).map(txt_to_instruction).collect();
    let file_in = fs::read_to_string(FILE_PATH).expect("missing first half of problem input");
    
    // vec of strings each with length divisible by four 
    let mut crates = file_in.split('\n')
                        .map(|s| {
                            let mut buf = String::from(s);
                            if buf.len() % INPUT_CRATE_LEN != 0 {
                                buf.push(' ');
                            }
                            assert!(buf.len() % INPUT_CRATE_LEN == 0);
                            buf
                        }).map(str_to_vchar)
                        .collect::<Vec<_>>();

    let _ = crates.pop(); // drop trailing \n
    let _ = crates.pop(); // drop the number annotation
    
    let mut pt_1_crates = rev_transpose(crates).iter().map(drop_empty).collect::<Vec<_>>();
    let mut pt_2_crates = pt_1_crates.clone();

    for i in &instructions {
        move_crates_9000(i, &mut pt_1_crates);
    }

    for i in &instructions {
        move_crates_9001(i, &mut pt_2_crates);
    }

    let pt_1 = pt_1_crates.into_iter().map(move |mut v| v.pop().unwrap()).collect::<String>();
    let pt_2 = pt_2_crates.into_iter().map(move |mut v| v.pop().unwrap()).collect::<String>();
    
    println!("pt_1 {:?}", pt_1);
    println!("pt_2 {:?}", pt_2);

    Ok(())
}




