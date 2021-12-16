// d16.rs
use std::fs;

const DAY: &'static str = "d16";

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

	// Day 16 Challenge : ??
    let line = content.lines().next().unwrap();

    //Decode the hex input in bits
    let bits: String = line.chars().fold("".to_owned(),|s,c| format!("{}{:04b}",s, c.to_digit(16).unwrap()));


    //analyse the input
    let (packet, _) = analyse_packet(&bits); 

    println!("First challenge - result: {}", get_version(&packet));

    println!("Second challenge - result: {}", get_value(&packet));
}

fn analyse_packet(s: &str) -> (Packet, &str) {
    let mut chars = s.chars().enumerate();

    //parse header
    let version = u32::from_str_radix(&s[0..(0+3)],2).unwrap();
    
    let type_id = u32::from_str_radix(&s[(0+3)..(0+6)],2).unwrap();
    
    chars.by_ref().take(6).count(); //skip header

    if type_id == 4 { //ValuePacket
        let mut val = 0;

        // While we don't encounter a zero, bits are part of the value
        loop {
            let (mut i,last_hex) = chars.next().unwrap();

            //Increase the value by the following 4 bits
            for (_,c) in chars.by_ref().take(4) {
                val = 2*val + if c == '1' {1} else {0};
            }

            // If it was the last group
            if last_hex == '0' {
                i += 5; // skip the five bits we just parsed
                return (Packet::Value(ValuePacket{version: version, value: val}), &s[i..]);
            }
        }
    } else { //Op packet

        let (mut i, len_type_id) = chars.next().unwrap();
        let mut sub = Vec::new();
        let mut ss = &s[(i+1)..];

        // Length-based subpackets
        if len_type_id == '0' {
            //next 15 bits are length of sub-packets
            let length = usize::from_str_radix(&ss[..15],2).unwrap(); 
            
            // the next length bits are packets
            ss = &ss[15..(15+length)];
            while ss.len() > 0 {
                let (p,ns) = analyse_packet(ss);
                sub.push(p);
                ss = ns;
            }

            // next packet start
            i += 15+length+1;
            return (Packet::Operation(OperationPacket {version: version, type_id: type_id, sub_packets: sub}),&s[i..]);
        
        // Number-based subpackets
        } else {
            let nb = u32::from_str_radix(&ss[..11],2).unwrap(); //next 11 bits are nb of sub packets
            ss = &ss[11..];
            for _ in 0..nb {
                let (p,ns) = analyse_packet(ss);
                sub.push(p);
                ss = ns;
            }
            return (Packet::Operation(OperationPacket {version: version, type_id: type_id, sub_packets: sub}), ss);
        }
    }
}

fn get_version(p: &Packet) -> u32 {
    let mut tot: u32 = 0;
    tot += match p {
        Packet::Value(p) => p.version,
        Packet::Operation(p) => p.version + p.sub_packets.iter().map(get_version).sum::<u32>()
    };
    tot
}

fn get_value(p: &Packet) -> u64 {
    match p {
        Packet::Value(p) => p.value,
        Packet::Operation(p) => {
            let mut values = p.sub_packets.iter().map(get_value);
            match p.type_id {
            0 => values.sum::<u64>(),
            1 => values.product::<u64>(),
            2 => values.min().unwrap(),
            3 => values.max().unwrap(),
            5 => if values.next().unwrap() > values.next().unwrap() {1} else {0},
            6 => if values.next().unwrap() < values.next().unwrap() {1} else {0},
            7 => if values.next().unwrap() == values.next().unwrap() {1} else {0},
            _ => 0,
            }
        }
    }
}

struct ValuePacket {
    version: u32,
    value: u64
}

struct OperationPacket {
    version: u32,
    type_id: u32,
    sub_packets: Vec<Packet>
}

enum Packet {
    Value(ValuePacket),
    Operation(OperationPacket)
}