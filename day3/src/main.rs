
use std::io;

const LC_ASCII_SHIFT: u32 = 96;
const UC_ASCII_SHIFT: u32 = 38;

// this is disgustingly bad but i didn't have much time today

// returns items in both compartments of a rucksack
fn to_common(rucksack: &String) -> String {
    let mut common = String::new();
    let d = rucksack.len() / 2;
    let c1 = &rucksack[..d];
    let c2 = &rucksack[d..];
    for ch in c1.chars() {
        if c2.contains(ch) && !common.contains(ch) {
            common.push(ch);
        }
    } common
}

fn c_pt_2(rucksacks: &[String] ) -> String {
    let v3 = &rucksacks[2];
    let v2 = &rucksacks[1];
    let v1 = &rucksacks[0];
    let mut common = String::new();
    for ch in v1.chars(){
        if v2.contains(ch) && !common.contains(ch) {
            common.push(ch);
        }
    }
    let mut c2 = String::new();
    for ch in v3.chars(){
        if common.contains(ch) && !c2.contains(ch) {
            c2.push(ch);
        }
    } c2

}

// returns priority values from a string of items
fn to_prio(items: String) -> u32 {
    let mut total: u32 = 0;
    for ch in items.chars() {
        let shift: u32 = if ch.is_uppercase() {UC_ASCII_SHIFT} else {LC_ASCII_SHIFT};
        total += ch as u32 - shift;
    } total
}

// input string to vec of supplies
fn to_supplies(stdin: &String) -> Vec<String> {
    let mut supplies = Vec::new();
    let mut rucksack = String::new();
    for ch in stdin.chars() {
        if ch == '\n' {
            supplies.push(rucksack.clone());
            rucksack = String::new();
        } else {
            rucksack.push(ch);
        } 
    } supplies
}

fn pt_1(stdin: &String) -> u32 {
    // stdin: vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n ...
    
    let supplies = to_supplies(&stdin);
    supplies.iter().map(to_common).map(to_prio).sum()
}

fn pt_2(stdin: &String) -> u32 {
    let supplies = to_supplies(&stdin);
    supplies.chunks(3).map(c_pt_2).map(to_prio).sum()
}



fn main() -> io::Result<()> {
    let mut stdin = io::read_to_string(io::stdin())?;
    if let Some(ch) = stdin.pop() {
        if ch != '\n' {
            stdin.push(ch);
            stdin.push('\n');
        }
    }

    // let stdin = String::new();
    let s = pt_2(&stdin);
    println!("part 1: {:?}", s);

    Ok(())
}
