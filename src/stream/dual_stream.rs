use std::io::{Read, Write};
use std::io;
use super::stream_configuration::StreamConfiguration;
use super::{Result, InternalMessageReader, InternalMessageWriter};

#[derive(Debug)]
pub struct DualMessenger<T> where T: Read + Write {
    configuration: StreamConfiguration,
    channel: Box<T>,
}

impl<T> DualMessenger<T> where T: Read + Write {

    /// Initializes a new MessageReader
    ///
    /// MessageReaders read a given `Read` trait-object for any messages between the given boundaries.
    pub fn new<V: Into<String>>(delimiter_string: V, beg_bound: V, end_bound: V, channel: T, hashing_enabled: bool) -> DualMessenger<T> {
        DualMessenger {
            configuration: StreamConfiguration {
                delimiter_string: delimiter_string.into(),
                beginning_boundary: beg_bound.into(),
                ending_boundary: end_bound.into(),
                hashing_enabled: hashing_enabled,
            },
            channel: Box::new(channel),
        }
    }

    pub fn new_from_config(config: StreamConfiguration, channel: T) -> DualMessenger<T> {
        DualMessenger {
            configuration: config,
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
    pub fn read_next_message(&mut self) -> Result<Vec<u8>> {
        let mut internal_reader = InternalMessageReader::new(self.channel.as_mut(), &self.configuration);
        internal_reader.read_next_message()
    }

    pub fn release(self) -> Box<T> {
        self.channel
    }
}

impl<T> Write for DualMessenger<T> where T: Read + Write {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut internal_writer = InternalMessageWriter::new(&self.configuration, self.channel.as_mut());
        Ok(internal_writer.write(buf)?)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.channel.flush()
    }
}