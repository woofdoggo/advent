use std::{io::{self, Read}, fmt};

type EmptyResult = Result<(), Box<dyn std::error::Error>>;

trait Packet {
    fn get_packet(&self) -> &dyn std::any::Any;
    fn get_total_version(&self) -> u32;
}

#[derive(Debug)]
struct Group {
    last_group: bool,
    number: u32
}

#[derive(Debug)]
struct GroupPacket {
    groups: Vec<Group>,
    version: u32
}

impl Packet for GroupPacket {
    fn get_packet(&self) -> &dyn std::any::Any {
        self
    }
    
    fn get_total_version(&self) -> u32 {
        self.version
    }
}

struct OperatorPacket {
    length_type: bool,
    length: u32,
    version: u32,
    subpackets: Vec<Box<dyn Packet>>
}

impl fmt::Debug for OperatorPacket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OperatorPacket")
            .field("length_type", &self.length_type)
            .field("length", &self.length)
            .field("packet_count", &self.subpackets.len())
            .finish()
    }
}

impl Packet for OperatorPacket {
    fn get_packet(&self) -> &dyn std::any::Any {
        self
    }

    fn get_total_version(&self) -> u32 {
        self.subpackets.iter().fold(self.version, |v, packet| v + packet.get_total_version())
    }
}

#[derive(Debug)]
struct PacketReader {
    packet: Vec<bool>,
    position: usize
}

impl PacketReader {
    fn new(packet: Vec<bool>) -> Self {
        PacketReader {
            packet,
            position: 0
        }
    }

    fn read(&mut self) -> Option<bool> {
        if self.position > self.packet.len() - 1 {
            None
        } else {
            self.position += 1;
            Some(self.packet[self.position - 1])
        }
    }

    fn read_int(&mut self, bit_count: u32) -> Option<u32> {
        let mut result = 0;

        for i in 1 ..= bit_count {
            if self.read()? {
                result |= 1 << bit_count - i;
            }
        }

        Some(result)
    }

    fn read_group(&mut self) -> Option<Group> {
        let last_group = !self.read()?;
        match self.read_int(4) {
            Some(number) => Some(Group {
                last_group,
                number
            }),
            None => None
        }
    }

    fn read_groups(&mut self) -> Vec<Group> {
        let mut result = Vec::new();

        loop {
            let group = self.read_group().expect("failed to read group");
            let last = group.last_group;
            result.push(group);

            if last { break; }
        }

        result
    }

    fn read_packet(&mut self) -> Box<dyn Packet> {
        let version = self.read_int(3).expect("failed to read version");
        let packet_type = self.read_int(3).expect("failed to read packet type");

        match packet_type {
            4 => {
                // group packet
                Box::new(GroupPacket {
                    groups: self.read_groups(),
                    version
                })
            },
            _ => {
                // operator packet
                let length_type = self.read().expect("packet terminated early");
                let length = match length_type {
                    true => self.read_int(11).expect("couldn't read length"),
                    false => self.read_int(15).expect("couldn't read length")
                };
                let mut subpackets: Vec<Box<dyn Packet>> = Vec::new();

                if length_type {
                    // length: sub-packet count
                    // read sub-packets
                    for _ in 0 .. length {
                        subpackets.push(self.read_packet());
                    }
                } else {
                    let starting_position = self.position;
                    while self.position != starting_position + length as usize {
                        subpackets.push(self.read_packet());
                    }
                }

                Box::new(OperatorPacket {
                    length_type,
                    length,
                    version,
                    subpackets
                })
            }
        }
    }
}

fn main() -> EmptyResult {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn parse(input: &String) -> Vec<bool> {
    const F: bool = false;
    const T: bool = true;
    let mut result = Vec::new();

    for char in input.chars() {
        if char == '\n' { continue; }
        result.extend_from_slice(match char {
            '0' => &[F, F, F, F],
            '1' => &[F, F, F, T],
            '2' => &[F, F, T, F],
            '3' => &[F, F, T, T],
            '4' => &[F, T, F, F],
            '5' => &[F, T, F, T],
            '6' => &[F, T, T, F],
            '7' => &[F, T, T, T],
            '8' => &[T, F, F, F],
            '9' => &[T, F, F, T],
            'A' => &[T, F, T, F],
            'B' => &[T, F, T, T],
            'C' => &[T, T, F, F],
            'D' => &[T, T, F, T],
            'E' => &[T, T, T, F],
            'F' => &[T, T, T, T],
            _ => panic!("???")
        });
    }

    result
}

fn part1(input: &String) -> EmptyResult {
    let mut reader = PacketReader::new(parse(input));
    let packet = reader.read_packet();
    
    println!("part 1: {}", packet.get_total_version());
    Ok(())
}

fn part2(input: &String) -> EmptyResult {

    Ok(())
}
