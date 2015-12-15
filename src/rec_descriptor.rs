use std::io::{MemReader, SeekCur, SeekSet, IoResult, IoError, InvalidInput};
use std::io::BufReader;
use std::option::Option;
use prototype::{Descriptor, Endianness, LittleEndian, BigEndian, Unknown};

use pretty_hex::PrettyHex;

pub struct RecDescriptor {
    // Generic descriptor information (super struct use case)
    pl_begin : u64, /* position of packet payload in the buffer */
    pl_size  : u32, /* number of octets of packet saved in file */
    pl_desc  : Option<Box<Descriptor>>, /* Payload descriptor */

    // Packet header specific information
    ts_sec   : u32, /* timestamp seconds (unix time) */
    ts_usec  : u32, /* timestamp microseconds */
    orig_len : u32  /* actual length of packet */
}

impl RecDescriptor {
    pub fn new() -> RecDescriptor {
        RecDescriptor {
            pl_begin : 0, pl_size : 0, pl_desc : None,
            ts_sec   : 0, ts_usec : 0, orig_len : 0
        }
    }

    pub fn seek_next(&self, reader: &mut MemReader) -> IoResult<()> {
        reader.seek(self.pl_size as i64, SeekCur)
    }
}

impl Descriptor for RecDescriptor {

    fn init(&mut self, reader: &mut MemReader, order: Endianness) -> IoResult<()> {
        match order {
            LittleEndian => {
                self.ts_sec   = try!(reader.read_le_u32());
                self.ts_usec  = try!(reader.read_le_u32());
                self.pl_size  = try!(reader.read_le_u32());
                self.orig_len = try!(reader.read_le_u32());
                self.pl_begin = try!(reader.tell());
            }
            BigEndian => {
                self.ts_sec   = try!(reader.read_le_u32());
                self.ts_usec  = try!(reader.read_le_u32());
                self.pl_size  = try!(reader.read_le_u32());
                self.orig_len = try!(reader.read_le_u32());
                self.pl_begin = try!(reader.tell());
            }
            Unknown => return Err(IoError{kind: InvalidInput,
                                          desc: "Record decode: Unknown endianness",
                                          detail: None })
        }
        self.pl_desc  = None;
        Ok(())
    }

    fn display_details(&self) {
        println!("RECORD HEADER DETAILS");
        println!("Position     : {}", self.pl_begin);
        println!("Time(unix-s) : {}", self.ts_sec);
        println!("Time(us)     : {}", self.ts_usec);
        println!("Payload size : {}", self.pl_size);
        println!("Packet size  : {}", self.orig_len);
    }

    fn display_raw_hdr(&self, reader: &mut MemReader) {
        match reader.seek((self.pl_begin - 16) as i64, SeekSet) {
            Ok(()) => (),
            Err(e) => {
                println!("Seek error: {}", e);
                return;
            }
        }
        let mut prntr = PrettyHex::new();
        println!("RAW HEADER VALUES");
        prntr.display(reader, Some(16));
    }

    fn display_raw_pl(&self, reader: &mut MemReader){
        match reader.seek((self.pl_begin) as i64, SeekSet) {
            Ok(()) => (),
            Err(e) => {
                println!("Seek error: {}", e);
                return;
            }
        }
        let mut prntr = PrettyHex::new();
        println!("RAW PAYLOAD VALUES");
        prntr.display(reader, Some(self.pl_size as u64));
    }
}

/* TODO:
1. Implement super-struct for all decriptors when inheritance matures
*/
