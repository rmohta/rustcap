use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use dump_decoder::DumpDecoder;

mod pretty_hex;
mod dump_decoder;
mod rec_descriptor;
mod prototype;

fn main() {
    let argums: Vec<String> = env::args().collect();
    if argums.len() != 2 {
        println!("Command syntax: rustycap filename");
        panic!("Invalid command syntax");
    }

    let mut file = File::open(&Path::new(&argums[1]));
    match file.read_to_end() {
        Err(e)  => panic!("file error: {}", e),
        Ok(buf) => {
            let rdr = BufReader::new(buf);
            let mut decoder = DumpDecoder::new(rdr);
            match decoder.decode(){
                Err(e) => panic!("Failed to decode dump: {}", e),
                Ok(()) => decoder.display()
            }
        }
    }
}
