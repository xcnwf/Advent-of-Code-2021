// d8.rs
use std::fs;

const DAY: &'static str = "d8";

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

	// Day 8 Challenge : Count the number of chars.

    // part1 - count the outputs of 1,2,7 and 8
    let mut displays: [usize;8] = [0;8];
    for line in content.lines() {
        let output = line.split('|').nth(1).unwrap();
        for blob in output.split_ascii_whitespace() {
            let n = blob.len();
            if n == 2 || n == 3 || n == 4 || n == 7 {
                displays[n] += 1;
            }
        }
    }

    println!("First Challenge: {}", displays.into_iter().sum());

    //part2 - Find the correct mapping of signals from input, and sum the outputs
    let mut mapping: std::collections::HashMap<char, char> = std::collections::HashMap::new();
    for line in content.lines() {
        // create a blank state of signal mapping 
        let mut all_mapping = std::collections::HashMap::new();
        for el in ['a','b','c','d','e','f','g'] {
            let mut vec: Vec<char> = Vec::new();
            for c in ['a','b','c','d','e','f','g'] {
                vec.push(c);
            }
            all_mapping.insert(el, vec);
        }

        let [input, output] = line.split("|").collect();

        // Parse the input numbers to destructively associate correct mappings
        for blob in input.split_ascii_whitespace() {
            let n = blob.len();
            let char_vec = blob.chars().collect::<Vec<char>>();
            if n == 2 || n == 3 || n == 4 /* || n == 7 non nécessaire car tous les segments => aucune info*/ {
                //Pour chaque caractère on va modifier son mapping correspondant
                for c in char_vec.iter() {
                    let v1 = all_mapping.get_mut(c).unwrap();
                    // on ne garde que les caractères concernés
                    (*v1).retain(|e| char_vec.contains(e));
                    if v1.len() == 1 {
                        // si le vecteur est vide, on sait connait alors le mapping de c, et on peut retirer le caractère mappé par c des autres vecteurs
                        all_mapping.iter_mut().filter(|&mut i, &mut v| (*i)!=c).for_each(|&mut i, &mut v| v.swap_remove(v1.first().unwrap()));
                        mapping.insert(c, v1.first().unwrap());
                    }
                }
            }
        }
        println!("{:?}", all_mapping);
        break;
    }
}