use std::io::{self};

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

fn line_to_pair(s: String) -> Vec<(i32, i32)> {
    s.split(',').collect::<Vec<_>>().iter()
        .map(|s| s.split('-').collect::<Vec<_>>())
        .map(|v| (v[0].parse::<i32>().unwrap(), v[1].parse::<i32>().unwrap()))
        .collect::<Vec<_>>()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let pairs: Vec<Vec<(i32,i32)>> = stdin.lines().map(|l| l.unwrap()).map(line_to_pair).collect();

    let pt_1: i32 = pairs.clone().into_iter().map(fully_contains).sum();
    let pt_2: i32 = pairs.into_iter().map(overlaps).sum();
    println!("pt 1: {:?}", pt_1);
    println!("pt 2: {:?}", pt_2);

    Ok(())
}




