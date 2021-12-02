// d1.rs
use std::fs;

const DAY: &'static str = "d2";

fn read_file() -> String {
    print!("Reading input...");

    let content: std::io::Result<String> = fs::read_to_string("../Inputs/".to_owned()+DAY+".txt");

    println!("Done!");

    if let Ok(s) = content {
        s
    } else {
        panic!("File Reading Error! Aborting!")
    }
}

fn main() {
    println!("Day {}\n", DAY);

    let content = read_file();

    // Day 2 Challenge : Multiply horizontal distance by depth.

    let mut h_dist: usize = 0;
    let mut depth: usize = 0;
    for (i, l) in content.lines().enumerate() {
        let res = l.split(" ").collect::<Vec<&str>>();
        match res[0] {
            "forward" => {h_dist += res[1].parse::<usize>().unwrap();},
            "down" => {depth += res[1].parse::<usize>().unwrap();},
            "up" => {depth -= res[1].parse::<usize>().unwrap();},
            _ => {panic!("Bad input");},
        }
    }

    println!("Result is : {}", h_dist*depth);
}