// d14.rs
use std::fs;
use std::collections::HashMap;

const DAY: &'static str = "d14";

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

	// Day 14 Challenge : ??
	let mut lines = content.lines();
    let mut polymer : Vec<char> = lines.next().unwrap().chars().collect();

    //skip empty line
    lines.next();

    //parse rules
    let mut rules = HashMap::new();
    for line in lines {
        let mut chars = line.chars();
        let c1 = chars.next().unwrap();
        let c2 = chars.next().unwrap();
        let nc = chars.last().unwrap();
        rules.insert((c1,c2), nc);
    }

    for _ in 0..10 {
        round(&mut polymer, &rules);
    }
    
    // We use reduce to make start from the first value ine the iter
    let freqs = frequency(&polymer);
    let max = freqs.values().reduce(|prev, curr| if curr > prev {curr} else {prev}).unwrap();
    let min = freqs.values().reduce(|prev, curr| if curr < prev {curr} else {prev}).unwrap();
    
    println!("First challenge - result: {}", max-min);

    // part2 - let's be smarter (first part was done that way to code differently and have more experience)
    // let's store the number of letters as well as the number of pairs at each step
    let mut letters: HashMap<char, u64> = HashMap::new();
    let mut pairs: HashMap<(char, char), u64> = HashMap::new();

    for (&p, &c) in rules.iter() {
        letters.insert(c, 0u64);
        pairs.insert(p, 0u64);
    }

    // restart polymer
    polymer = content.lines().next().unwrap().chars().collect();
    for i in 0..(polymer.len()-1) {
        *pairs.get_mut(&(polymer[i], polymer[i+1])).unwrap() += 1;
        *letters.get_mut(&polymer[i]).unwrap() += 1;
    } 
    
    //add the last letter
    *letters.get_mut(&polymer.last().unwrap()).unwrap() += 1;

    for _ in 0..40 {
        let mut new_pairs: HashMap<(char, char), u64> = pairs.clone();
        for (&p, &c) in rules.iter() {
            let (c1, c2) = p;
            let n = pairs[&p];

            // we add n new letters
            *letters.get_mut(&c).unwrap() += n;

            // and constitute new pairs
            *new_pairs.get_mut(&(c1, c)).unwrap() += n;
            *new_pairs.get_mut(&(c, c2)).unwrap() += n;

            // and brek the old ones
            *new_pairs.get_mut(&p).unwrap() -= n;
        }
        pairs = new_pairs;

        /*println!("{:?}", letters);
        println!("{:?}", pairs);
        break;*/
    }

    //let's do the calculation
    let max = letters.values().reduce(|prev, curr| if curr > prev {curr} else {prev}).unwrap();
    let min = letters.values().reduce(|prev, curr| if curr < prev {curr} else {prev}).unwrap();

    println!("Second challenge - result: {}", max-min);
}

fn round (polymer: &mut Vec<char>, rules: &HashMap<(char, char), char>) {
    let mut new_chars = Vec::new();

    for i in 0..(polymer.len()-1) {
        match rules.get(&(polymer[i], polymer[i+1])) {
            None => (),
            Some(&nc) => new_chars.push((i+1, nc)),
        }
    }

    //insert the new chars
    polymer.reserve(new_chars.len());
    for (j, &(i, nc)) in new_chars.iter().enumerate() {
        polymer.insert(i+j, nc);
    }

}

// get the frequency of apparition of chars in a vec
fn frequency<T>(polymer: &Vec<T>) -> HashMap<T, u32>  
where T: std::hash::Hash,
      T: Ord, 
      T: Copy,  {
    let mut freq = HashMap::new();
    for &c in polymer {
        if !freq.contains_key(&c) {
            freq.insert(c, 1);
        } else {
            *freq.get_mut(&c).unwrap() += 1;
        }
    }
    freq
}