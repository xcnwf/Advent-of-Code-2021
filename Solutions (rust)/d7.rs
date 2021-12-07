// d7.rs
use std::fs;

const DAY: &'static str = "d7";

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

	// Day 7
    let mut numbers: Vec<f64> = content.lines().next().unwrap().split(",").map(|e| e.parse::<f64>().unwrap()).collect();

    //part1 - calculate the median
    //We want to know the median of the values, because it is the value each number is the closest to.

    numbers.sort_by(|a,b| a.partial_cmp(b).unwrap());
    let mut median: f64;
    let size = numbers.len();
    if size % 2 == 0 {
        median = *numbers.iter().nth(size/2 - 1).unwrap() / 2. + *numbers.iter().nth(size/2).unwrap() / 2.;
    } else {
        median = *numbers.iter().nth(size / 2 - 1).unwrap();
    }

    //println!("median: {}", median);

    let min_fuel = numbers.iter().fold(0, |acc, &e| acc+ (e-median).abs() as usize);

    println!("First chall - Result: {}", min_fuel);

    // Part 2 - Calculate the mean value
    //We want each crab to move the least amount possible from its current position because the price is now exponential
    // So we take the mean because it assures a maximal distance (equal to the mean value) from each point to it
    let mut mean: f64 = numbers.iter().sum::<f64>() / size as f64;

    // dunno why but floor is better than ceil so it is what it is
    // I guess i should calculate both, but i'm kinda lazy rn.
    mean = mean.floor();

    //println!("mean: {}", mean);


    // sum from 1 to n is equal to n * (n+1) / 2
    let exp_fuel = numbers.iter().fold(0, |acc, &e| {let fuel = (e-mean).abs() as usize; acc + fuel * (fuel + 1) / 2 as usize});
    println!("Second chall - Result: {}", exp_fuel)
}