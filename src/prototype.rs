use std::io::{MemReader, IoResult};

pub enum Endianness { BigEndian, LittleEndian, Unknown }

pub trait Descriptor {
    fn init(&mut self, reader: &mut MemReader, order: Endianness) -> IoResult<()>;
    fn display_details(&self);
    fn display_raw_hdr(&self, reader: &mut MemReader);
    fn display_raw_pl(&self, reader: &mut MemReader);

    fn display(&self, reader: &mut MemReader) {
        println!("++++ START OF DATA DETAILS ++++");
        self.display_details();
        println!("");
        self.display_raw_hdr(reader);
        println!("");
        self.display_raw_pl(reader);
        println!("----- END OF DATA DETAILS -----\n");
    }
}
