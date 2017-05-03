use std::io::{Write, Result};
use std::mem;

pub struct MessageWriter<'a> {
    delimiter_string: String,
    beginning_boundary: String,
    ending_boundary: String,
    writer: &'a mut Write,
}

impl<'a> MessageWriter<'a> {

    /// Initializes a new MessageWriter
    ///
    /// MessagerWriters write to a given `Write` trait-object given the provided boundaries 
    pub fn new(delimiter_string: String, beg_bound: String, end_bound: String, writer: &mut Write) -> MessageWriter {
        MessageWriter {
            delimiter_string: delimiter_string,
            beginning_boundary: beg_bound,
            ending_boundary: end_bound,
            writer: writer
        }
    }
}

impl<'a> Write for MessageWriter<'a> {
    
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let size1 = self.writer.write(self.delimiter_string.as_ref())?;
        let size2 = self.writer.write(self.beginning_boundary.as_ref())?;
        let size3 = self.writer.write(mem::size_of_val(buf).to_string().as_ref())?;
        let size4 = self.writer.write(self.delimiter_string.as_ref())?;
        let size5 = self.writer.write(buf)?;
        let size6 = self.writer.write(self.delimiter_string.as_ref())?;
        let size7 = self.writer.write(self.ending_boundary.as_ref())?;
        let size8 = self.writer.write(self.delimiter_string.as_ref())?;

        Ok(size1 + size2 + size3 + size4 + size5 + size6 + size7 + size8)

    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }

}