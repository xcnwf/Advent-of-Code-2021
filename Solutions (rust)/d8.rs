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

    println!("First Challenge: {}", displays.iter().sum::<usize>());

    //part2 - Find the correct mapping of signals from input, and sum the outputs
    //let mut mapping: std::collections::HashMap<char, char> = std::collections::HashMap::new();
    
    // n_mapping map for each number its segment displays.
    let mut n_mapping = std::collections::HashMap::new();
    n_mapping.insert(2, vec!['c','f']);
    n_mapping.insert(3, vec!['a','c','f']);
    n_mapping.insert(4, vec!['b','c','d','f']);

    let mut total : usize = 0;

    for line in content.lines() {
        // create a blank state of signal mapping 
        let mut all_mapping = std::collections::HashMap::new();

        //in order to separate som signals, we need to count some chars
        let mut char_count = std::collections::HashMap::new();
        let mut char_count_others = std::collections::HashMap::new();
        for el in ['a','b','c','d','e','f','g'] {
            let mut vec: Vec<char> = Vec::new();
            for c in ['a','b','c','d','e','f','g'] {
                vec.push(c);
            }
            all_mapping.insert(el, vec);
            char_count.insert(el, 0);
            char_count_others.insert(el, 0);
        }

        // Parse the input numbers to destructively associate correct mappings
        for blob in line.split("|").next().unwrap().split_ascii_whitespace() {
            let n = blob.len();
            let char_vec = blob.chars().collect::<Vec<char>>();
            if n == 2 || n == 3 || n == 4 /* || n == 7 non nécessaire car tous les segments => aucune info*/ {
                let map = n_mapping.get(&n).unwrap();

                //Pour tous les mappings
                for (&c, mapping) in all_mapping.iter_mut() {
                    // si il est présent, alors il ne mappe que ceux possibles pour afficher n.
                    if char_vec.contains(&c) {
                        update_mapping(mapping, map);

                    //Pour tous les autres mappings potentiels, l'on sait que ils ne peuvent pas être map à ces caractères
                    } else {
                        mapping.retain(|&e| !map.contains(&e));
                    }
                    
                }
                //we want to count the chars to better differenciate
                char_vec.iter().for_each(|&c| {*char_count.get_mut(&c).unwrap() += 1})
            } else {
                //we want to count the chars to better differenciate
                char_vec.iter().for_each(|&c| {*char_count_others.get_mut(&c).unwrap() += 1})
            }
        }

        
        // println!("{:?}", all_mapping);
        

        // println!("{:?}", char_count);
        // println!("{:?}", char_count_others);

        let mut mapped = Vec::new();

        for (&c,&n) in char_count_others.iter() {
            if n == 6 { //d or f
                let signal;
                if char_count[&c] == 3 { //f
                    signal = 'f';
                } else {
                    signal = 'd'
                }
                for (&c2, mapping) in all_mapping.iter_mut() {
                    // si il est présent, alors il ne mappe que ceux possibles pour afficher n.
                    if c2==c {
                        mapping.retain(|&e| e==signal);

                    //Pour tous les autres mappings potentiels, l'on sait que ils ne peuvent pas être map à ces caractères
                    } else {
                        mapping.retain(|&e| e != signal);
                    }
                }
                mapped.push(signal);
            }
            if n == 7 { //a or g
                if char_count[&c] == 0 { //g
                
                    for (&c2, mapping) in all_mapping.iter_mut() {
                        // si il est présent, alors il ne mappe que ceux possibles pour afficher n.
                        if c2==c {
                            mapping.retain(|&e| e=='g');

                        //Pour tous les autres mappings potentiels, l'on sait que ils ne peuvent pas être map à ces caractères
                        } else {
                            mapping.retain(|&e| e != 'g');
                        }
                    }
                    mapped.push('g');
                }
            }
        }

        //mapping are complete, just need to remove the wrong ones.
        for (_, mapping) in all_mapping.iter_mut() {
            if mapping.len() == 2 {
                mapping.retain(|&e| !mapped.contains(&e));
            }
        }

        //so now we do a reverse mapping
        let mut r_map = std::collections::HashMap::new();

        all_mapping.iter().for_each(|(&c, mapping)|{r_map.insert(*mapping.first().unwrap(), c);});

        //println!("{:?}", r_map);


        //All that is left is to get the numbers with the reverse mapping.
        let mut value: usize = 0;
        for blob in line.split("|").nth(1).unwrap().split_ascii_whitespace() {
            value = 10*value + get_value(&r_map, &blob.chars().collect())
        }

        total += value;
        
        //break;
    }
    println!("{}",total)
}

fn update_mapping(potential_chars: &mut Vec<char>, chars_to_keep: &Vec<char>) {
    potential_chars.retain(|&e| chars_to_keep.contains(&e));
}

fn get_value(r_mapping: &std::collections::HashMap<char,char>, signals: &Vec<char>) -> usize {
    if signals.contains(&r_mapping[&'b']) { // 'b' => 0, 4, 5, 6, 8, 9
        if signals.contains(&r_mapping[&'e']) { // => 0,6,8
            if signals.contains(&r_mapping[&'d']) { // => 6,8
                if signals.contains(&r_mapping[&'c']) { //8
                    return 8;
                } else {
                    return 6;
                }
            } else {
                return 0;
            }
        } else { // 4,5,9
            if signals.contains(&r_mapping[&'a']) { // 5,9
                if signals.contains(&r_mapping[&'c']) { //9
                    return 9;
                } else {
                    return 5;
                }
            } else {
                return 4;
            }
        }
    } else { // 1,2,3,7
        if signals.contains(&r_mapping[&'d']) { // 2,3
            if signals.contains(&r_mapping[&'e']) { // 2
                return 2;
            } else {
                return 3;
            }

        }else {
            if signals.contains(&r_mapping[&'a']) { //7
                return 7;
            } else {
                return 1;
            }
        }

    }
}