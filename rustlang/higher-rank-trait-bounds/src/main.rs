use std::io::Read;
use rand;

trait Checksum<R:Read> {
    fn calc(&mut self, r:R) -> Vec<u8>;
}

struct Xor;

impl <R:Read> Checksum<R> for Xor {
    fn calc(&mut self, mut r:R) -> Vec<u8> {
        let mut res: u8 = 0;
        let mut buf = [0u8;8];
        loop {
            let read = r.read(&mut buf).unwrap();
            if read == 0 { break }
            for b in &buf[..read] {
                res ^= b;
            }
        }

        vec![res]
    }
}

struct Add;

impl <R:Read> Checksum<R> for Add {
    fn calc(&mut self, mut r:R) -> Vec<u8> {
        let mut res: u8 = 0;
        let mut buf = [0u8;8];
        loop {
            let read = r.read(&mut buf).unwrap();
            if read == 0 { break }
            for b in &buf[..read] {
                let tmp = res as u16 + *b as u16;
                res = tmp as u8;
            }
        }

        vec![res]
    }
}


fn main() {
    let mut buf = [0u8;8];
    let mut checker: Box<dyn Checksum<&[u8]>> = if rand::random() {

        println!("Initializing Xor Checksum");
        Box::new(Xor)

    } else {
        println!("Initializing Add Checksum");
        Box::new(Add)

    };

    let mut data = "Sedm lumpu slohlo pumpu za uplnku".as_bytes();
    let mut i = 0;

    loop {
        let chunk_size = data.read(&mut buf).unwrap();
        if chunk_size == 0 { break }
        let cs = checker.calc(&buf[..chunk_size]);
        println!("Checksum {} is {:?}", i, cs);
        i+= 1;
    }

}