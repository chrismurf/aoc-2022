use std::str::FromStr;
use std::fs;
use std::cmp::{PartialOrd, Ordering};

#[derive(Debug, Clone)]
pub struct Error;

#[derive(Debug, Clone, PartialEq, Eq, Ord)]
enum Packet {
    List(Vec<Box<Packet>>),
    Value(i32)
}

impl FromStr for Packet {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut idx = 0;
        Self::from_stream_with_offset(s, &mut idx)
    }
}

impl Packet {
    fn list_from_value(value : i32) -> Self {
        Self::List(vec![Box::new(Packet::Value(value))])
    }

    fn from_stream_with_offset(s: &str, offset: &mut usize) -> Result<Self, Error> {
        match &s[*offset..*offset+1] {
            "[" => {
                *offset += 1;
                let mut vec = Vec::new();
                while s[*offset..*offset+1usize] != *"]" {
                    vec.push(Box::new(Packet::from_stream_with_offset(s, offset)?));
                    if s[*offset..*offset+1usize] == *"," {
                        *offset += 1usize;
                    }
                }
                *offset += 1usize;
                Ok(Packet::List(vec))
            },
            _ => {
                let mut end = *offset + 1usize;
                while s[end..end+1] != *"]" && s[end..end+1] != *"," {
                    end += 1usize;
                }
                let value : i32 = s[*offset..end].parse().or(Err(Error {}))?;
                *offset = end;
                Ok(Packet::Value(value))
            },
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::Value(x), Packet::Value(y)) => x.partial_cmp(y),

            (x@Packet::List(_), Packet::Value(y)) => x.partial_cmp(&Packet::list_from_value(*y)),

            (Packet::Value(x), y@Packet::List(_)) => Packet::list_from_value(*x).partial_cmp(&y),

            (Packet::List(x), Packet::List(y)) => {
                for i in 0..x.len() {
                    if i >= y.len() { return Some(Ordering::Greater) }
                    match x[i].partial_cmp(&y[i]) {
                        Some(Ordering::Equal) => continue,
                        ordering => return ordering,
                    }
                }
                if x.len() == y.len() { return Some(Ordering::Equal) }
                else { return Some(Ordering::Less) }
            },
        }
    }
}

pub fn day13() -> Result<(), Error> {
    let file = fs::read_to_string("input.txt").expect("File 'input.txt' not readable.");

    let mut correct_order : Vec<usize> = vec![];

    let mut all_packets = Vec::new();

    for (i, packet_strs) in file.split("\n\n").enumerate() {
        let mut packets = packet_strs.split("\n").map(|x| x.parse().unwrap()).collect::<Vec<Packet>>();
        if packets[0] <= packets[1] { correct_order.push(i+1) };
        all_packets.append(&mut packets);
    }

    println!("Part 1: {}", correct_order.iter().sum::<usize>());

    // Part 2
    let start_pkt : Packet = "[[2]]".parse()?;
    let end_pkt : Packet = "[[6]]".parse()?;

    all_packets.push(start_pkt.clone());
    all_packets.push(end_pkt.clone());
    all_packets.sort();
   
    let start_pos = all_packets.iter().position(|x| x == &start_pkt).unwrap()+1;
    let end_pos = all_packets.iter().position(|x| x == &end_pkt).unwrap()+1;
    println!("Part 2: {}", start_pos * end_pos);

    Ok(())
    
}

pub fn main() -> Result<(), Error> {
    day13()
}
