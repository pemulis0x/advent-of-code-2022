use std::fs;

const FILE_PATH: &str = "trees.txt";


// this is pretty raw and ugly, i'm too lazy to polish rn
// .. will leave as exercise to future self


fn rotate<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> 
where
    T: Clone,
{
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).rev().collect::<Vec<T>>())
        .collect()
}

fn mark_visible(v: &mut [(u8, u8)]) {
    let mut max = 0u8; 
    for num in v{
        if num.0 > max {
            max = num.0;
            num.1 = 1u8;
        }
    }
}

fn calc_views(trees: &mut [(u32, u32)]) {
    for i in 0..trees.len() {
        let mut view_dist = i;
        for j in 0..i {
            if trees[j].0 >= trees[i].0 {
                view_dist = i - j;
            }
        }
        trees[i].1 *= view_dist as u32;
        // println!("tree {} (height {}) can see {}", i, trees[i].0, view_dist);
    }
}

fn main() {
    let file_in = fs::read_to_string(FILE_PATH).expect("missing first half of problem input");


    // part 1 //
    let mut forest = file_in.split('\n')
        .map(|s| s.chars().map(|ch| (ch.to_digit(10).unwrap() as u8, 0u8)).collect::<Vec<(u8,u8)>>())
        .collect::<Vec<_>>();
    
    for _ in 0..4 {
        for trees in &mut forest[..] {
            mark_visible(trees);
        }
        if let Some(last_row) = forest.pop() {
            forest.push(last_row.iter().map(|t| (t.0, 1u8)).collect())
        }
        forest = rotate(forest);
    }

    let pt_1 = &forest.iter().map(|v| v.iter().map(|n| n.1 as u32).sum::<u32>()).sum::<u32>();


    // part 2 //
    let mut views = file_in.split('\n')
        .map(|s| s.chars().map(|ch| (ch.to_digit(10).unwrap(), 1u32)).collect::<Vec<(u32,u32)>>())
        .collect::<Vec<_>>();

    for _ in 0..4 {
        for trees in &mut views[..] {
            calc_views(trees);
        }
        if let Some(last_row) = views.pop() {
            views.push(last_row.iter().map(|t| (t.0, 0u32)).collect())
        }
        views = rotate(views);
    }

    let pt_2 = views.iter().flatten().map(|(_,b)| b).max().unwrap();

    println!("{:?}", pt_1);
    println!("{:?}", pt_2);

    //1814
    //Some(330786)

}






