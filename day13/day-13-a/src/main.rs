// https://adventofcode.com/2022/day/13

struct Packet {
    data1: Vec<String>,
    data2: Vec<String>,
}

impl Packet {
    pub fn new(data1: Vec<String>, data2: Vec<String>) -> Self {
        Self { data1, data2 }
    }
}

fn main() {
    let mut data1maybe: Option<Vec<String>> = None;
    let mut data2maybe: Option<Vec<String>> = None;
    let mut packets: Vec<Packet> = vec![];
    include_str!("../input.txt")
        .lines()
        .for_each(|x| {
            if x.is_empty() {
                packets.push(Packet::new(data1maybe.clone().unwrap(), data2maybe.clone().unwrap()));
                data1maybe = None;
                data2maybe = None;
            } else if data1maybe.is_none() {
                data1maybe = Some(vec![x.to_owned()])
            } else {
                data2maybe = Some(vec![x.to_owned()])
            }
        });
    for x in packets {
        println!("{:?}   -   {:?}", x.data1, x.data2);
    }
}
