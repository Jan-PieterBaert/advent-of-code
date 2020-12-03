use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    const SUM: i32 = 2020;
    println!("Reading file");
    let mut vec: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines("input.1") {
        for line in lines {
            if let Ok(line) = line {
                let number = line.parse::<i32>().unwrap();
                vec.push(number);
            }
        }
    }

    println!("Solution for the first question");
    for i in 1..vec.len() - 1 {
        for j in i..=vec.len() - 1 {
            if vec[i] + vec[j] == SUM {
                println!("SUM of {} and {} is {}", vec[i], vec[j], SUM);
                println!(
                    "Product of {} and {} is {}",
                    vec[i],
                    vec[j],
                    vec[i] * vec[j]
                );
                break;
            }
        }
    }

    println!("\n");
    println!("Solution for the second question");
    for i in 1..vec.len() {
        for j in i..vec.len() {
            for k in j..vec.len() {
                if vec[i] + vec[j] + vec[k] == SUM {
                    println!("SUM of {}, {} and {} is {}", vec[i], vec[j], vec[k], SUM);
                    println!(
                        "Product of {}, {} and {} is {}",
                        vec[i],
                        vec[j],
                        vec[k],
                        vec[i] * vec[j] * vec[k]
                    );
                    break;
                }
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
