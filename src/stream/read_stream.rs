use std::io::{Read};
use super::stream_configuration::StreamConfiguration;
use super::{read_message_from_reader, Result};

pub struct MessageReader<T> where T: Read {
    delimiter_string: String,
    beginning_boundary: String,
    ending_boundary: String,
    reader: T,
}

impl<T: Read> MessageReader<T> {

    /// Initializes a new MessageReader
    ///
    /// MessageReaders read a given `Read` trait-object for any messages between the given boundaries.
    pub fn new<V: Into<String>>(delimiter_string: V, beg_bound: V, end_bound: V, reader: T) -> MessageReader<T> {
        MessageReader {
            delimiter_string: delimiter_string.into(),
            beginning_boundary: beg_bound.into(),
            ending_boundary: end_bound.into(),
            reader: reader,
        }
    }

    pub fn new_from_config(config: StreamConfiguration, reader: T) -> MessageReader<T> {
        MessageReader {
            delimiter_string: config.delimiter_string,
            beginning_boundary: config.beginning_boundary,
            ending_boundary: config.ending_boundary,
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
        read_message_from_reader(&mut self.reader, self.delimiter_string.clone(), self.ending_boundary.clone(), self.beginning_boundary.clone())
    }
}