use libcrc::crc;
use elgamal::SignedMessage;
use std::{collections::HashMap, fmt::Display};

struct CrcCollisions {
    map: HashMap<u32,Vec<u8>>
}

impl From<HashMap<u32,Vec<u8>>> for CrcCollisions {
    fn from(value: HashMap<u32,Vec<u8>>) -> Self {
        Self {
            map: value
        }
    }
}

impl Display for CrcCollisions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       for (crc, bytes) in &self.map {
            write!(f,"CRC:{:b} - {:?})\n",crc,bytes)?;
        }
        Ok(())
    }
}

fn main() {
    let gx = 0b10111;
    let collisions = byte_collisions(gx);
    println!("First Part:\n{}",collisions);
    println!("Second Part:");
    let m = 215;
    let message = SignedMessage::sign(m, gx).unwrap();
    println!("Message: {:b}",m);
    println!("{:?}",message);
    println!("Message signature check: {}",message.check(gx));
}

fn byte_collisions(gx: u32) -> CrcCollisions {
    let mut map = HashMap::new();
    for i in 0..u8::MAX {
        let crc = crc(i as u32,gx);
        if map.contains_key(&crc) {
            let vec: &mut Vec<u8> = map.get_mut(&crc).unwrap();
            vec.push(i);
        } else {
            let mut vec = Vec::new();
            vec.push(i);
            map.insert(crc,vec);
        }
    }
    map.into()
}
