use std::io::{Read, Write, Result};
use std::mem;
use super::{vec_contains_slice, find_where_slice_begins, locate_items_between_delimiters};

#[derive(Debug)]
pub struct DualMessenger<'a, T> where T: 'a + Read + Write {
    delimiter_string: String,
    beginning_boundary: String,
    ending_boundary: String,
    channel: &'a mut T
}

impl<'a, T> DualMessenger<'a, T> where T: Read + Write {

    /// Initializes a new MessageReader
    ///
    /// MessageReaders read a given `Read` trait-object for any messages between the given boundaries.
    pub fn new(delimiter_string: String, beg_bound: String, end_bound: String, channel: &mut T) -> DualMessenger<T> {
        DualMessenger {
            delimiter_string: delimiter_string,
            beginning_boundary: beg_bound,
            ending_boundary: end_bound,
            channel: channel,
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
        // initializes a buffer for bytes coming from the reader
        let mut buffer = [0; 1];
        // initializes the message buffer for containing the final message
        let mut message: Vec<u8> = Vec::new();
        // initializes the search buffer for placing data the program is currently searching through
        let mut search_vec: Vec<u8> = Vec::new();
        // whether or not the boundary_start has been found
        let mut beginning_found = false;

        loop {
            if beginning_found {
                let _ = self.channel.read(message.as_mut_slice());
                // look for ending, accumulate message
                let delim = self.delimiter_string.clone();
                let end = self.ending_boundary.clone();
                let proper_delim = delim + end.as_str();
                if vec_contains_slice(&message, proper_delim.as_ref()) {
                    // end code found. filter it out and send the message.
                    if let Some(v) = find_where_slice_begins(&message, proper_delim.as_ref()) {
                        let other_half = message.split_off(v as usize);
                        return Some(message);
                    }
                }
            } else {
                // beginning message NOT found, look for it
                // read the provided stream
                let res = self.channel.read(&mut buffer);

                // If no bytes have been sent, return None. We probably reached the end of the stream.
                if let Ok(v) = res {
                    if v <= 0 {
                        return None;
                    }
                }
                search_vec.append(&mut Vec::from(buffer.as_ref()));
                let delim = self.delimiter_string.clone();
                let begin = self.beginning_boundary.clone();
                let proper_delim = delim + begin.as_str();
                if let Some(size) = locate_items_between_delimiters(&search_vec, proper_delim.as_ref(), self.delimiter_string.as_ref()) {
                    // grab everything after, then push it into the message buffer
                    if let Ok(v) = String::from_utf8(size) {
                        if let Ok(num) = str::parse::<usize>(v.as_str()) {
                            // append the remainder found that isn't the boundary as part of the message
                            // message.append(&mut search_vec.split_off(v as usize));
                            // mark the beginning of found
                            beginning_found = true;
                            // clear the search vector
                            search_vec.clear();
                            // set the capacity of the message vector
                            // by pushing requisite amount of elements
                            let delim = self.delimiter_string.clone();
                            let end = self.ending_boundary.clone();
                            let end_delim = delim.as_str().to_owned() + end.as_str() + delim.as_str();
                            for _ in 0..(num + mem::size_of_val(end_delim.as_bytes())) {
                                message.push(0);
                            }
                            // message.reserve(size.len());
                        }
                    }
                } 
            }
        }
    }

    pub fn release(mut self) -> &'a mut T {
        self.channel
    }
}

impl<'a, T> Write for DualMessenger<'a, T> where T: Read + Write {
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