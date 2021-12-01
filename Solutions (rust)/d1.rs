// d1.rs
use std::fs;

const DAY: &'static str = "d1";

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

    // Day 1 Challenge : Count the number of times the next int is bigger than the current one
    let mut n_incr = 0;
    let mut prev_value = 0;

    let mut v1 = 0; let mut v2 = 0; let mut v3 = 0;
    let mut n_group_incr = 0;

    for (i,l) in content.lines().enumerate() {

        let value = l.parse::<u32>().unwrap();
        
        //part 1
        if i>0 && value > prev_value {
            n_incr += 1;
        }
        prev_value = value;

        //part2
        if i == 0 {
            v1 = value;
        } else if i==1{
            v2 = value;
        } else if i == 2 {
            v3 = value;
        } else {
            // Pas besoin de tout sommer
            if value > v1 {
                n_group_incr += 1;
            }
            v1 = v2;
            v2 = v3;
            v3 = value;
        }
    }

    println!("First challenge: n_incr = {}",n_incr);
    println!("Second challenge : n_incr = {}", n_group_incr);
}