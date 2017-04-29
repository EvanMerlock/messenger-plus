use std::io::{Read, Write, Result};
use super::{vec_contains_slice, find_where_slice_begins, find_where_slice_intersects};

#[derive(Debug)]
pub struct DualMessenger<'a, T> where T: 'a + Read + Write {
    beginning_boundary: String,
    ending_boundary: String,
    channel: &'a mut T
}

impl<'a, T> DualMessenger<'a, T> where T: Read + Write {

    /// Initializes a new MessageReader
    ///
    /// MessageReaders read a given `Read` trait-object for any messages between the given boundaries.
    pub fn new(beg_bound: String, end_bound: String, channel: &mut T) -> DualMessenger<T> {
        DualMessenger {
            beginning_boundary: beg_bound,
            ending_boundary: end_bound,
            channel: channel,
        }
    }

    /// Reads the next message from the DualMessenger
    ///
    /// This method reads the next message from the previously created DualMessenger
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
            // read the provided stream
            let res = self.channel.read(&mut buffer);

            // If no bytes have been sent, return None. We probably reached the end of the stream.
            if let Ok(v) = res {
                if v <= 0 {
                    return None;
                }
            }

            if beginning_found {
                // look for ending, accumulate message
                message.append(&mut Vec::from(buffer.as_ref()));
                if vec_contains_slice(&message, self.ending_boundary.as_ref()) {
                    // end code found. filter it out and send the message.
                    if let Some(v) = find_where_slice_begins(&message, self.ending_boundary.as_ref()) {
                        let _ = message.split_off(v as usize);
                        return Some(message);
                    }
                }
            } else {
                // beginning message NOT found, look for it
                search_vec.append(&mut Vec::from(buffer.as_ref()));
                if vec_contains_slice(&search_vec, self.beginning_boundary.as_ref()) {
                    // grab everything after, then push it into the message buffer
                    if let Some(v) = find_where_slice_intersects(&search_vec, self.beginning_boundary.as_ref()) {
                        // append the remainder found that isn't the boundary as part of the message
                        message.append(&mut search_vec.split_off(v as usize));
                        // mark the beginning of found
                        beginning_found = true;
                        // clear the search vector
                        search_vec.clear();
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
        let size1 = self.channel.write(self.beginning_boundary.as_ref())?;
        let size2 = self.channel.write(buf)?;
        let size3 = self.channel.write(self.ending_boundary.as_ref())?;

        Ok(size1 + size2 + size3)

    }

    fn flush(&mut self) -> Result<()> {
        self.channel.flush()
    }
}