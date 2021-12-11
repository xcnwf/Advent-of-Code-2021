// d11.rs
use std::fs;

const DAY: &'static str = "d11";
const SIZE: usize = 10;

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

	// Day 11 Challenge : ??
    let mut octopuses: [usize; SIZE*SIZE] = [0; SIZE*SIZE];

    for (i,line) in content.lines().enumerate() {
        for (j,n) in line.chars().enumerate() {
            octopuses[i*SIZE + j] = n.to_digit(10).unwrap() as usize;
        }
    } 

    //input parsed!

    //part 1 
    let mut total_flashes: u64 = 0;
    for _ in 0..100 {

        //increase all octopuses
        for i in 0..SIZE {
            for j in 0..SIZE {
                octopuses[i*SIZE + j] += 1;
            }
        }

        //make them all flash if they haven't recursively
        for i in 0..SIZE {
            for j in 0..SIZE {
                if octopuses[i*SIZE + j] > 9 {
                    total_flashes += octopus_flash(&mut octopuses, i, j) as u64;
                }
            }
        }
    }

    println!("First challenge - result : {}", total_flashes);

    //part 2 - Continuation of part1, bu need to know when they all flash simultaneously.
    let mut n = 100;
    loop {
        let mut flashes = 0;
        n+=1;

        //increase all octopuses
        for i in 0..SIZE {
            for j in 0..SIZE {
                octopuses[i*SIZE + j] += 1;
            }
        }

        //make them all flash if they haven't recursively
        for i in 0..SIZE {
            for j in 0..SIZE {
                if octopuses[i*SIZE + j] > 9 {
                    flashes += octopus_flash(&mut octopuses, i, j) as u64;
                }
            }
        }
        if flashes == 100 {
            break;
        }
    }

    println!("Second challenge - result: {}",n);
}

//make the octopus flash, resetting it and increasing the level of nerby octopuses, 
// making them recursively flash if needed
fn octopus_flash(octopuses: &mut [usize; SIZE*SIZE],i: usize, j: usize) -> usize{
    //First, reset to 0
    octopuses[i*SIZE + j] = 0;
    let mut flashes: usize = 1;

    let (istart, iend) = (if i == 0 {0} else {i-1}, if i == SIZE-1 {SIZE} else {i+2});
    let (jstart, jend) = (if j == 0 {0} else {j-1}, if j == SIZE-1 {SIZE} else {j+2});
    for i2 in istart..iend {
        for j2 in jstart..jend {

            //if an octopus didn't just flash, increase it
            if octopuses[i2 * SIZE + j2] != 0 {
                octopuses[i2 * SIZE + j2] += 1;
            }

            //then make it flash if it needs to
            if octopuses[i2 * SIZE + j2] > 9 {
                flashes += octopus_flash(octopuses, i2, j2);
            }
        }
    }

    flashes
}