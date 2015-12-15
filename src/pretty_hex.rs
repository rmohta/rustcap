
use std::io::MemReader;
use std::option::Option;

pub struct PrettyHex {
    byte_no : u64
}


impl PrettyHex {

    pub fn new() -> PrettyHex {
        PrettyHex {byte_no : 0}
    }

    /*pub fn reset(&mut self) {
        self.byte_no = 0;
    }*/

    pub fn display(&mut self, reader: &mut MemReader, end: Option<u64>) {
        let endloc = end.unwrap_or(0u64);
        while ! reader.eof() {
            if end != None && self.byte_no == endloc {
                break;
            }
            match reader.read_u8() {
                Err(e) => panic!("Memory read error: {}", e),
                Ok(i)  => {
                    match self.byte_no {
                        n if n % 16 == 0 => {
                            if n != 0 {
                                println!("");
                            }
                            let msbs = self.byte_no >> 32;
                            let lsbs = self.byte_no & 0x0000FFFF;
                            print!("{:0>4X} {:0>4X}: ", msbs, lsbs);
                        }
                        n if n % 4 == 0  => print!("  "),
                        _          => print!(" ")
                    }
                    print!("{:0>2X}", i);
                    self.byte_no += 1;
                }
            }
        }
        println!("\nTotal of {} bytes", self.byte_no);
    }

}

// TODO: Print line number for first row
