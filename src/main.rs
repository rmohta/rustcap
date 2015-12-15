use dump_decoder::DumpDecoder;

use std::fs::File;
use std::io::MemReader;
use std::path::Path;
use std::os::args;

mod pretty_hex;
mod dump_decoder;
mod rec_descriptor;
mod prototype;

fn main() {
    let argums = args();
    if argums.len() != 2 {
        println!("Command syntax: rustycap filename");
        panic!("Invalid command syntax");
    }

    let mut file = File::open(&Path::new(argums.get(1).as_slice()));
    match file.read_to_end() {
        Err(e)  => panic!("file error: {}", e),
        Ok(buf) => {
            let rdr = MemReader::new(buf);
            let mut decoder = DumpDecoder::new(rdr);
            match decoder.decode(){
                Err(e) => panic!("Failed to decode dump: {}", e),
                Ok(()) => decoder.display()
            }
        }
    }
}
