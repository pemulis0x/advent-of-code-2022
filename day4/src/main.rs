use std::io::{self, Error};

fn fully_contains(pair: Vec<(i32, i32)>) -> i32 {
    let view = &pair[..];
    let p1 = view[0];
    let p2 = view[1];
    let c1 = p1.0 <= p2.0 && p1.1 >= p2.1;
    let c2 = p1.0 >= p2.0 && p1.1 <= p2.1;

    if c1 || c2  {
        // println!("{:?} contains {:?}", p1, p2);
        return 1
    } 0
}

fn overlaps(pair: Vec<(i32, i32)>) -> i32 {
    let view = &pair[..];
    let p1 = view[0];
    let p2 = view[1];
     if p1.1 >= p2.0 && p1.0 <= p2.1 {
         // println!("{:?} overlaps {:?}", p1, p2);
         return 1
     } 0
}

fn line_to_pair(line: Result<String, Error>) -> Vec<(i32, i32)> {
    let s = line.unwrap();
    let fmt: Vec<(i32, i32)> = s.split(',').collect::<Vec<_>>().iter()
        .map(|s| s.split('-')
        .collect::<Vec<_>>())
        .map(|v| (v[0].parse::<i32>().unwrap(), v[1].parse::<i32>().unwrap()))
        .collect::<Vec<_>>();

    fmt
}

//34-82,33-81 -> ((34,82), (33, 81))

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let pairs = stdin.lines().map(line_to_pair);
    let solution: i32 = pairs.map(fully_contains).sum();
    // let solution: i32 = pairs.map(overlaps).sum();

    println!("solution: {:?}", solution);
    Ok(())
}




