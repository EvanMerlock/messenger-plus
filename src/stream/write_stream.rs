use std::io::{Write, Result};
use std::mem;
use super::stream_configuration::StreamConfiguration;

pub(crate) struct InternalMessageWriter<'a, T: 'a> where T: Write {
    internal_writer: &'a mut T,
    temporary_configuration: &'a StreamConfiguration,
}

impl<'a, T: Write> InternalMessageWriter<'a, T> {
    pub(crate) fn new(config: &'a StreamConfiguration, writer: &'a mut T) -> InternalMessageWriter<'a, T> {
        InternalMessageWriter {
            internal_writer: writer,
            temporary_configuration: config
        }
    }
}

impl<'a, T: Write> Write for InternalMessageWriter<'a, T> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let mut vec = Vec::new();
        vec.push(self.internal_writer.write(self.temporary_configuration.delimiter_string.as_ref())?);
        vec.push(self.internal_writer.write(self.temporary_configuration.beginning_boundary.as_ref())?);
        vec.push(self.internal_writer.write(mem::size_of_val(buf).to_string().as_ref())?);
        vec.push(self.internal_writer.write(self.temporary_configuration.delimiter_string.as_ref())?);
        vec.push(self.internal_writer.write(buf)?);
        vec.push(self.internal_writer.write(self.temporary_configuration.delimiter_string.as_ref())?);
        vec.push(self.internal_writer.write(self.temporary_configuration.ending_boundary.as_ref())?);
        vec.push(self.internal_writer.write(self.temporary_configuration.delimiter_string.as_ref())?);

        Ok(vec.into_iter().fold(0, |x, y| x + y))

    }

    fn flush(&mut self) -> Result<()> {
        self.internal_writer.flush()
    }
}

pub struct MessageWriter<T> where T: Write {
    configuration: StreamConfiguration,
    writer: T,
}

impl<T: Write> MessageWriter<T> {

    /// Initializes a new MessageWriter
    ///
    /// MessagerWriters write to a given `Write` trait-object given the provided boundaries 
    pub fn new<V: Into<String>>(delimiter_string: V, beg_bound: V, end_bound: V, writer: T, hashing_enabled: bool) -> MessageWriter<T> {
        MessageWriter {
            configuration: StreamConfiguration::new(
                delimiter_string.into(),
                beg_bound.into(),
                end_bound.into(),
                hashing_enabled
            ),
            writer: writer
        }
    }

    pub fn new_from_config(config: StreamConfiguration, writer: T) -> MessageWriter<T> {
        MessageWriter {
            configuration: config,
            writer: writer,
        }
    }

    pub fn get_writer(&self) -> &T {
        &self.writer
    }
}

impl<T: Write> Write for MessageWriter<T> {
    
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let mut temp_writer = InternalMessageWriter::new(&self.configuration, &mut self.writer);
        Ok(temp_writer.write(buf)?)
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }

}