
use std::io;


fn cals_per_elf(stdin: String) -> Vec<i32> {
    // stdin looks like "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000"
    
    let mut number = String::new();
    let mut elf = Vec::new();
    let mut sums = Vec::new();
    let mut first_n = true;

    for ch in stdin.chars() {
        if ch != '\n' {
            number.push(ch);
            first_n = true;
        } else if first_n {
            elf.push(number.parse::<i32>().unwrap());
            number = String::new();
            first_n = false;
        } else {
            sums.push(elf.iter().sum());
            elf = Vec::new();
        }
    }

    if let Ok(n) = number.parse::<i32>() {
        elf.push(n);
    }
    sums.push(elf.iter().sum());
    
    sums
}

fn main() -> io::Result<()> {
    let stdin = io::read_to_string(io::stdin())?;

    // p1
    let cpe = &mut cals_per_elf(stdin)[..];
    cpe.sort_unstable();
    let part_1 = cpe[cpe.len() - 1];

    // p2
    let mut sorted_cals = Vec::from(cpe);
    let mut top_3 = Vec::new();
    for _ in 0..3 {
        top_3.push(sorted_cals.pop().unwrap());
    }
    let part_2: i32 = top_3.iter().sum();

    println!("part 1:\t{:?}",part_1);
    println!("part 2:\t{:?}",part_2);
    Ok(())
}






