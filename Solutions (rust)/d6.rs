// d6.rs
use std::fs;

const DAY: &'static str = "d6";

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

	// Day 6 Challenge : Count the number of fishes after N days
    let mut fishes: Vec<u8> = Vec::new();

    //Parsing input
    for n in content.lines().next().unwrap().split(",") {
        fishes.push(n.parse::<u8>().unwrap());
    }
    //Done!

    let mut new_fishes = 0;

    for i in 0..80 {
        for fish in fishes.iter_mut() {
            //update fishes and make them to be created
            if *fish == 0 {
                *fish = 7;
                new_fishes += 1;
            }

            *fish -= 1;

        }

        //create new fishes
        for _ in 0..new_fishes {
            fishes.push(8);
        }
        new_fishes = 0;
    }

    println!("Chall 1 - Result: {}", fishes.len());

    
    
    //part2 - We won't add fish to the board, but rather calculate new values on a 7-day period

    let mut fishes: [u64; 9] = [0;9];
    //Parsing input
    for n in content.lines().next().unwrap().split(",") {
        fishes[n.parse::<usize>().unwrap()] += 1;
    }
    //Done!

    //256 days = 36 weeks and 4 days
    for _ in 0..36 {
        let mut fishes_copy: [u64; 9] = [0;9];
        fishes_copy.copy_from_slice(&fishes);

        //In a week, 0-6 fishes don't move and have new fishes
        for i in 0..7 {
            fishes[i] += fishes_copy[(i+7)%9];
        }

        //While the 7-8 fishes moved and are only newborns of 5 and 6
        for i in 7..9 {
            fishes[i] = fishes_copy[i-2];
        }
    }

    // the 4 days    
    new_fishes = fishes[0..4].iter().sum::<u64>();

    println!("Chall 2 - Result: {}",fishes.iter().sum::<u64>()+new_fishes);
}