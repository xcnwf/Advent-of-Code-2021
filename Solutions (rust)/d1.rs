// d1.rs
use std::env;
use std::fs;

const DAY: &str = "d1";

/*fn read_file() -> Vec<&'static str> {
    print!("Reading input...");

    let lines = fs::read_to_string("../Inputs/".to_owned()+DAY+".txt").expect("wtf").split("\n").collect();

    println!("Done!");

    lines
}*/

fn main() {
    println!("Day {}\n", DAY);

    print!("Reading input...");

    let content = fs::read_to_string("../Inputs/".to_owned()+DAY+".txt").expect("wtf");
    let lines:Vec<&str> = content.split("\n").collect();

    println!("Done!");

    // Day 1 Challenge : count increases
    let mut nIncr = 0;
    let mut prevValue = 0;

    let mut v1 = 0; let mut v2 = 0; let mut v3 = 0;
    let mut nGroupIncr = 0;

    for (i,l) in lines.iter().filter(|&e| !e.is_empty()).enumerate() {

        let value = l.parse::<u32>().unwrap();
        
        //part 1
        if i>0 && value > prevValue {
            nIncr += 1;
        }
        prevValue = value;

        //part2
        if i == 0 {
            v1 = value;
        } else if i==1{
            v2 = value;
        } else if i == 2 {
            v3 = value;
        } else {
            if value+v2+v3 > v1+v2+v3 {
                nGroupIncr += 1;
            }
            v1 = v2;
            v2 = v3;
            v3 = value;
        }
    }

    println!("First challenge: nIncr = {}",nIncr);
    println!("Second challenge : nIncr = {}", nGroupIncr);
}