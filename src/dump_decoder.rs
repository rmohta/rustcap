use std::io::{MemReader, SeekSet, IoResult, IoError};
use std::io::{InvalidInput, MismatchedFileTypeForOperation};

use pretty_hex::PrettyHex;
use rec_descriptor::RecDescriptor;
use prototype::{Descriptor, Endianness, BigEndian, LittleEndian, Unknown};

pub struct DumpDecoder {
    // Global header information
    version_major : u16,  /* major version number */
    version_minor : u16,  /* minor version number */
    thiszone      : i32,  /* GMT to local correction */
    sigfigs       : u32,  /* accuracy of timestamps */
    snaplen       : u32,  /* max length of captured packets, in octets */
    network       : u32,  /* data link type */

    // Dump decoder info and states
    dump          : MemReader,
    endian        : Endianness,
    records       : Vec<RecDescriptor>
}

impl DumpDecoder {

    pub fn new(reader: MemReader) -> DumpDecoder {
        DumpDecoder {
            version_major : 0,
            version_minor : 0,
            thiszone      : 0,
            sigfigs       : 0,
            snaplen       : 0,
            network       : 0,
            dump          : reader,
            endian        : Unknown,
            records       : vec![]
        }
    }

    pub fn decode(&mut self) -> IoResult<()>{
        match self.dump.seek(0, SeekSet) {
            Ok(()) => (),
            Err(e) => {
                println!("Seek error: {}", e);
                return Err(IoError{kind: InvalidInput,
                                   desc: "Memory seek error",
                                   detail: None });
            }
        }
        let magic = try!(self.dump.read_le_u32());
        match magic {
            0xA1B2C3D4 => {
                self.version_major = try!(self.dump.read_le_u16());
                self.version_minor = try!(self.dump.read_le_u16());
                self.thiszone      = try!(self.dump.read_le_i32());
                self.sigfigs       = try!(self.dump.read_le_u32());
                self.snaplen       = try!(self.dump.read_le_u32());
                self.network       = try!(self.dump.read_le_u32());
                self.endian = LittleEndian;
            }
            0xD4C3B2A1 => {
                self.version_major = try!(self.dump.read_be_u16());
                self.version_minor = try!(self.dump.read_be_u16());
                self.thiszone      = try!(self.dump.read_be_i32());
                self.sigfigs       = try!(self.dump.read_be_u32());
                self.snaplen       = try!(self.dump.read_be_u32());
                self.network       = try!(self.dump.read_be_u32());
                self.endian = BigEndian;
            }
            _ => return Err(IoError{kind: MismatchedFileTypeForOperation,
                                    desc: "File decode: Invalid file type",
                                    detail: None })
        }
        while !self.dump.eof() {
            let mut i = RecDescriptor::new();
            try!(i.init(&mut self.dump, self.endian));
            match i.seek_next(&mut self.dump) {
                Err(e) => return Err(e),
                Ok(()) => self.records.push(i)
            }
        }
        Ok(())
    }

    pub fn display(&mut self) {
        self.display_dump();
        println!("");
        match self.endian {
            Unknown => println!("Data not decoded"),
            _       => {
                println!("PCAP GLOBAL HEADER DETAILS");
                println!("Major version  : {  }", self.version_major);
                println!("Minor version  : {  }", self.version_minor);
                println!("Time zone(s)   : {  }", self.thiszone);
                println!("Time precision : {  }", self.sigfigs);
                println!("Snap length    : {  }", self.snaplen);
                println!("Link type      : {  }", self.network);
            }
        }
        for i in self.records.iter() {
            println!("");
            i.display(&mut self.dump);
        }
    }

    pub fn display_dump(&mut self){
        match self.dump.seek(0, SeekSet) {
            Ok(()) => (),
            Err(e) => {
                println!("Seek error: {}", e);
                return;
            }
        }
        let mut prntr = PrettyHex::new();
        println!("FULL PCAP DUMP");
        prntr.display(&mut self.dump, None);
    }
}

/*
TODO:
1. Validate BEnd case
2. Add display for file type and network type
*/
