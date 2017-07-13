use std::io::{Write, Result};
use std::mem;
use super::stream_configuration::StreamConfiguration;

pub struct MessageWriter<T> where T: Write {
    delimiter_string: String,
    beginning_boundary: String,
    ending_boundary: String,
    writer: T,
}

impl<T: Write> MessageWriter<T> {

    /// Initializes a new MessageWriter
    ///
    /// MessagerWriters write to a given `Write` trait-object given the provided boundaries 
    pub fn new<V: Into<String>>(delimiter_string: V, beg_bound: V, end_bound: V, writer: T) -> MessageWriter<T> {
        MessageWriter {
            delimiter_string: delimiter_string.into(),
            beginning_boundary: beg_bound.into(),
            ending_boundary: end_bound.into(),
            writer: writer
        }
    }

    pub fn new_from_config(config: StreamConfiguration, writer: T) -> MessageWriter<T> {
        MessageWriter {
            delimiter_string: config.delimiter_string,
            beginning_boundary: config.beginning_boundary,
            ending_boundary: config.ending_boundary,
            writer: writer,
        }
    }

    pub fn get_writer(&self) -> &T {
        &self.writer
    }
}

impl<T: Write> Write for MessageWriter<T> {
    
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