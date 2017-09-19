use std::io::{Read, Write, Result};
use std::mem;
use super::stream_configuration::StreamConfiguration;
use super::read_message_from_reader;

#[derive(Debug)]
pub struct DualMessenger<T> where T: Read + Write {
    delimiter_string: String,
    beginning_boundary: String,
    ending_boundary: String,
    channel: Box<T>
}

impl<T> DualMessenger<T> where T: Read + Write {

    /// Initializes a new MessageReader
    ///
    /// MessageReaders read a given `Read` trait-object for any messages between the given boundaries.
    pub fn new<V: Into<String>>(delimiter_string: V, beg_bound: V, end_bound: V, channel: T) -> DualMessenger<T> {
        DualMessenger {
            delimiter_string: delimiter_string.into(),
            beginning_boundary: beg_bound.into(),
            ending_boundary: end_bound.into(),
            channel: Box::new(channel),
        }
    }

    pub fn new_from_config(config: StreamConfiguration, channel: T) -> DualMessenger<T> {
        DualMessenger {
            delimiter_string: config.delimiter_string,
            beginning_boundary: config.beginning_boundary,
            ending_boundary: config.ending_boundary,
            channel: Box::new(channel),
        }
    }
 
    /// Reads the next message from the DualReader
    ///
    /// This method reads the next message from the previously created DualReader
    /// The message is formatted with 2 boundaries.
    ///
    /// # Errors
    /// This method will return None if it cannot find a message and the stream ends (typically due to EOF).
    /// This method can hang if no new data is sent through the pipe as `Read` can block.
    /// This method can produce irratic results if the `boundary_start` or `boundary_end` is found within the message.
    pub fn read_next_message(&mut self) -> Option<Vec<u8>> {
         read_message_from_reader(self.channel.as_mut(), self.delimiter_string.clone(), self.ending_boundary.clone(), self.beginning_boundary.clone())
    }

    pub fn release(self) -> Box<T> {
        self.channel
    }
}

impl<T> Write for DualMessenger<T> where T: Read + Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let size1 = self.channel.write(self.delimiter_string.as_ref())?;
        let size2 = self.channel.write(self.beginning_boundary.as_ref())?;
        let size3 = self.channel.write(mem::size_of_val(buf).to_string().as_ref())?;
        let size4 = self.channel.write(self.delimiter_string.as_ref())?;
        let size5 = self.channel.write(buf)?;
        let size6 = self.channel.write(self.delimiter_string.as_ref())?;
        let size7 = self.channel.write(self.ending_boundary.as_ref())?;
        let size8 = self.channel.write(self.delimiter_string.as_ref())?;

        Ok(size1 + size2 + size3 + size4 + size5 + size6 + size7 + size8)

    }

    fn flush(&mut self) -> Result<()> {
        self.channel.flush()
    }
}