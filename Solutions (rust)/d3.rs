use std::fs;

const DAY: &'static str = "d3";

/**
 * Return the integer corresponding to its bit representation
*/
fn bit_to_int<'a>(arr: &'a str, inv: bool) -> usize {
	let mut n: usize = 0;
	for i in 0usize..12usize {
		let bit = if arr.chars().nth(i).unwrap() == '1' {1} else {0};
		n = 2*n + if inv {1-bit} else {bit};
	}
	n
}

/**
 * Find the most used bits in each position from a list of binary numbers 
*/
fn get_most_bits<'a>(lines: &Vec<&'a str>) -> [usize; 12] {
	let mut bit_count: [usize; 12] = [0; 12];
	let mut n_lines: usize = 0;

	for l in lines.iter() {
			l.chars().enumerate().for_each(|(i,c)| {bit_count[i] += if c == '1' {1} else {0}});
			n_lines += 1;
	}

	for i in 0..12 {
			let bit = if 2*bit_count[i] >= n_lines {1} else {0};
			bit_count[i] = bit;
	}

	bit_count
}

fn read_file() -> String {
	print!("Reading input...");

	let content: std::io::Result<String> = fs::read_to_string("../Inputs/".to_owned()+DAY+".txt");

	println!("Done!");

	content.unwrap()
}

fn main() {
    println!("Day {}\n", DAY);

    let content = read_file();

    // Day 3 Challenge : Find most common bits
    let mut bit_count: [usize; 12];
    let mut vec: Vec<&str> = content.lines().collect();

    bit_count = get_most_bits(&vec);

    let mut n1;
    let mut n2;

    let s = bit_count.iter().fold("".to_string(),|s,&e| s+&e.to_string());

    n1 = bit_to_int(s.as_str(), false);
    n2 = bit_to_int(s.as_str(), true);

    println!("First challenge: result = {}",n1*n2);
    
    //Second part
    vec = content.lines().collect();
    let mut curr_bit = 0;
    while {
			bit_count=get_most_bits(&vec);
			// Keep only the numbers with the right bit at the 'curr_bit'th position.
			// Here the right bit is the most used one.
			vec = vec.iter().filter(|&e| (if e.chars().nth(curr_bit).unwrap()=='1' {1} else {0}) == bit_count[curr_bit]).cloned().collect();
			
			curr_bit+=1;
			
			vec.len() > 1 && curr_bit < 12
    } {}

    n1 = bit_to_int(*(vec.first().unwrap()), false);

    vec = content.lines().collect();
    curr_bit = 0;
    let mut nvec: Vec<&str>;
    while {
			bit_count=get_most_bits(&vec);
			// The right bit here is the least used one.
			nvec = vec.iter().filter(|&e| (if e.chars().nth(curr_bit).unwrap()=='1' {1} else {0}) != bit_count[curr_bit]).cloned().collect();
			
			curr_bit+=1;
			
			// If all numbers have the same bit in that place, skip
			// This is useful because the select the bit is the LEAST used, which may not be a possibility.
			if nvec.len() == 0 {
				true
			} else {
				vec = nvec;
				vec.len() > 1 && curr_bit < 12
			}
    } {}

    n2 = bit_to_int(*(vec.first().unwrap()), false);

    println!("Second challenge: result = {}",n1*n2);
}