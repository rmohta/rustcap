//A 'trait' is a collection of methods declared/defined for an unknown type: Self. Traits can be implemented for any data type

use std::io::{BufReader, Result};

pub enum Endianness { 
                BigEndian, 
                LittleEndian, 
                Unknown,
}

///Define a trait to display parts of a captured packet
pub trait Descriptor {
    fn init(&mut self, reader: &mut BufReader, order: Endianness) -> Result<()>;
    fn display_details(&self);
    ///
    fn display_raw_hdr(&self, reader: &mut BufReader);
    fn display_raw_pl(&self, reader: &mut BufReader);

    fn display(&self, reader: &mut BufReader) {
        println!("++++ START OF DATA DETAILS ++++");
        self.display_details();
        println!("");
        self.display_raw_hdr(reader);
        println!("");
        self.display_raw_pl(reader);
        println!("----- END OF DATA DETAILS -----\n");
    }
}
