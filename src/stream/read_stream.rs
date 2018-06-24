use std::io::{Read};
use super::stream_configuration::StreamConfiguration;
use super::{read_message_from_reader, Result};

pub(crate) struct InternalMessageReader<'a, T: 'a> where T: Read {
    internal_reader: &'a mut T,
    configuration: &'a StreamConfiguration,
}

impl<'a, T: Read> InternalMessageReader<'a, T> {
    pub(crate) fn new(internal_reader: &'a mut T, configuration: &'a StreamConfiguration) -> InternalMessageReader<'a, T> {
        InternalMessageReader {
            internal_reader: internal_reader,
            configuration: configuration,
        }
    }

    pub fn read_next_message(&mut self) -> Result<Vec<u8>> {
        read_message_from_reader(self.internal_reader, self.configuration)
    }
}

pub struct MessageReader<T> where T: Read {
    configuration: StreamConfiguration,
    reader: T,
}

impl<T: Read> MessageReader<T> {

    /// Initializes a new MessageReader
    ///
    /// MessageReaders read a given `Read` trait-object for any messages between the given boundaries.
    pub fn new<V: Into<String>>(delimiter_string: V, beg_bound: V, end_bound: V, reader: T, hashing_enabled: bool) -> MessageReader<T> {
        MessageReader {
            configuration: StreamConfiguration {
                delimiter_string: delimiter_string.into(),
                beginning_boundary: beg_bound.into(),
                ending_boundary: end_bound.into(),
                hashing_enabled: hashing_enabled,
            },
            reader: reader,
        }
    }

    pub fn new_from_config(config: StreamConfiguration, reader: T) -> MessageReader<T> {
        MessageReader {
            configuration: config,
            reader: reader,
        }
    }

    pub fn get_reader(&self) -> &T {
        &self.reader
    }

    /// Reads the next message from the MessageReader
    ///
    /// This method reads the next message from the previously created MessageReader
    /// The message is formatted with 2 boundaries.
    ///
    /// # Errors
    /// This method will return Err if it cannot find a message and the stream ends (typically due to EOF).
    /// This method can hang if no new data is sent through the pipe as `Read` can block.
    /// This method can produce irratic results if the `boundary_start` or `boundary_end` is found within the message.
    pub fn read_next_message(&mut self) -> Result<Vec<u8>> {
        let mut internal_reader = InternalMessageReader::new(&mut self.reader, &self.configuration);
        internal_reader.read_next_message()
    }
}