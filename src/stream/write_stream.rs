use std::io::{Write, Result};

pub struct MessageWriter<'a> {
    beginning_boundary: String,
    ending_boundary: String,
    writer: &'a mut Write,
}

impl<'a> MessageWriter<'a> {
    pub fn new(beg_bound: String, end_bound: String, writer: &mut Write) -> MessageWriter {
        MessageWriter {
            beginning_boundary: beg_bound,
            ending_boundary: end_bound,
            writer: writer
        }
    }
}

impl<'a> Write for MessageWriter<'a> {
    
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let size1 = self.writer.write(self.beginning_boundary.as_ref())?;
        let size2 = self.writer.write(buf)?;
        let size3 = self.writer.write(self.ending_boundary.as_ref())?;

        Ok(size1 + size2 + size3)

    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }

}