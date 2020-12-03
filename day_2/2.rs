use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Reading file");
    let mut minimums: Vec<i32> = Vec::new();
    let mut maximums: Vec<i32> = Vec::new();
    let mut letters: Vec<char> = Vec::new();
    let mut strings: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("input.2") {
        for line in lines {
            if let Ok(line) = line {
                let line = line.clone();
                let split = line.split(&['-',':',' '][..]);
                let parts: Vec<&str> = split.collect();
                minimums.push(parts[0].parse::<i32>().unwrap());
                maximums.push(parts[1].parse::<i32>().unwrap());
                let char_res = parts[2].chars().next();
                match char_res {
                    Some(x) => letters.push(x),
                    None => println!("Wrong input format!"),
                }
                strings.push(parts[4].to_string());
            }
        }
    }

    let mut number_of_good_passwords = 0;
    for i in 0..strings.len() {
        let count = strings[i].matches(letters[i]).count() as i32;
        if count >= minimums[i] && count <= maximums[i] {
            number_of_good_passwords+=1;
        }
    }

    println!("Solution for the first question");
    println!("The good ones are: {}", number_of_good_passwords);


    number_of_good_passwords = 0;
    for i in 0..strings.len() {
        let mut checkstr = String::from("");
        let chars: Vec<char> = strings[i].chars().collect();
        checkstr.push(chars[(minimums[i] - 1) as usize]);
        checkstr.push(chars[(maximums[i] - 1) as usize]);
        let count = checkstr.matches(letters[i]).count() as i32;
        if count == 1 {
            number_of_good_passwords+=1;
        }
    }
    println!("");
    println!("Solution for the second question");
    println!("The good ones are: {}", number_of_good_passwords);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
