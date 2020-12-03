use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Reading file");
    const STEP_SIZE: usize = 3;
    const STEP_DOWN: usize = 1;
    let mut vec: Vec<Vec<bool>> = Vec::new();
    const TREE_CHAR: char = '#';
    if let Ok(lines) = read_lines("input.3") {
        for line in lines {
            if let Ok(line) = line {
                let mut local_vec: Vec<bool> = Vec::new();
                for c in line.chars(){
                    local_vec.push(c == TREE_CHAR);
                }
                vec.push(local_vec);
            }
        }
    }
    let length: usize = vec[0].len();
    let mut point = 0;
    let mut number_of_trees = 0;
    for i in (0..vec.len()).step_by(STEP_DOWN) {
        if vec[i][point]{
            number_of_trees += 1;
        }
        point = (point + STEP_SIZE) % length;
    }

    println!("Solution for the first question");
    println!("Number of trees crossed is {}", number_of_trees);

    let mut counts: usize = 1;
    let configs = [(1,1), (3,1), (5,1), (7,1), (1,2)];
    let length: usize = vec[0].len();
    for (side,down) in &configs{
        let mut point = 0;
        let mut number_of_trees = 0;
        for i in (0..vec.len()).step_by(*down) {
            if vec[i][point]{
                number_of_trees += 1;
            }
            point = (point + side) % length;
        }
        counts *= number_of_trees
    }

    println!("\n");
    println!("Solution for the second question");
    println!("The multiplication is {}", counts);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
