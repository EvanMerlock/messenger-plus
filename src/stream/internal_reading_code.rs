use std::mem;
use std::io::Read;

use super::super::utils::{vec_contains_slice, find_where_slice_begins, locate_items_between_delimiters};
use super::{Error, Result, ErrorKind};

/// Reads the next message from the given Reader
///
/// This method reads the next message from the previously created Reader
/// The message is formatted with 2 boundaries.
///
/// # Errors
/// This method will return None if it cannot find a message and the stream ends (typically due to EOF).
/// This method can hang if no new data is sent through the pipe as `Read` can block.
/// This method can produce irratic results if the `boundary_start` or `boundary_end` is found within the message.
pub fn read_message_from_reader_old(reader: &mut Read, delimiter_string: String, ending_boundary: String, beginning_boundary: String) -> Option<Vec<u8>> {
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
            let _ = reader.read(message.as_mut_slice());
            // look for ending, accumulate message
            let delim = delimiter_string.clone();
            let end = ending_boundary.clone();
            let proper_delim = delim + end.as_str();
            if vec_contains_slice(&message, proper_delim.as_ref()) {
                // end code found. filter it out and send the message.
                if let Some(v) = find_where_slice_begins(&message, proper_delim.as_ref()) {
                    let _ = message.split_off(v as usize);
                    return Some(message);
                }
            }
        } else {
            // beginning message NOT found, look for it
            // read the provided stream
            let res = reader.read(&mut buffer);
            // If no bytes have been sent, return None. We probably reached the end of the stream.
            if let Ok(v) = res {
                if v <= 0 {
                    return None;
                }
            }
            search_vec.append(&mut Vec::from(buffer.as_ref()));
            let delim = delimiter_string.clone();
            let begin = beginning_boundary.clone();
            let proper_delim = delim + begin.as_str();
            if let Some(size) = locate_items_between_delimiters(&search_vec, proper_delim.as_ref(), delimiter_string.as_ref()) {
                // grab everything after, then push it into the message buffer
                if let Ok(v) = String::from_utf8(size) {
                    if let Ok(num) = str::parse::<usize>(v.as_str()) {
                        // append the remainder found that isn't the boundary as part of the message
                        // mark the beginning of found
                        beginning_found = true;
                        // clear the search vector
                        search_vec.clear();
                        // set the capacity of the message vector
                        // by pushing requisite amount of elements
                        let delim = delimiter_string.clone();
                        let end = ending_boundary.clone();
                        let end_delim = delim.as_str().to_owned() + end.as_str() + delim.as_str();
                        for _ in 0..(num + mem::size_of_val(end_delim.as_bytes())) {
                            message.push(0);
                        }
                    }
                }
            } 
        }
    }
}

pub fn read_message_from_reader(reader: &mut Read, delimiter_string: String, ending_boundary: String, beginning_boundary: String) -> Result<Vec<u8>> {
    // initalize a buffer for storing beginning delimiter
    let mut buffer = [0; 1];
    // the total size of the beginning of the delimiter (delimiter_string)
    let mut delimiter_sized_vec = create_empty_vec_of_size(mem::size_of_val(delimiter_string.as_bytes()));
    // read the beginning from the reader
    let res = reader.read(delimiter_sized_vec.as_mut_slice());

    match res {
        Ok(v) => {
            if v <= 0 {
                return Err(Error::from(ErrorKind::BufferEmpty));
            }
        }
        Err(e) => {
            return Err(Error::from(e));
        }
    }

    // if the delimiter is equal to the string
    if delimiter_sized_vec.as_slice() == delimiter_string.as_bytes() {
        // read the beginning boundary from the reader
        let mut beg_bound_sized_vec = create_empty_vec_of_size(mem::size_of_val(beginning_boundary.as_bytes()));
        let _ = reader.read_exact(beg_bound_sized_vec.as_mut_slice())?;
        // if it matches
        if beg_bound_sized_vec.as_slice() == beginning_boundary.as_bytes() {
            // start scanning for the byte size
            let mut acc_buff = Vec::new();
            // while we haven't found delimiter_string
            loop {
                // read one byte at a time
                reader.read_exact(&mut buffer)?;
                // push the byte to the acc_buffer
                acc_buff.push(buffer[0]);
                // if the acc_buffer ends with the delimiter string
                if acc_buff.ends_with(delimiter_string.as_bytes()) {
                    // we have found the delimiter, end the loop
                    break;
                }
            }
            // find where the delimiter slice begins
            if let Some(v) = find_where_slice_begins(&acc_buff, delimiter_string.as_bytes()) {
                // split the number off
                acc_buff.split_off(v);
                // convert the string to a number
                let buf_str = String::from_utf8(acc_buff.clone())?;
                let num = str::parse::<usize>(&buf_str)?;
                // create the message vector
                let mut message_vec = create_empty_vec_of_size(num);
                // this is the message, read the remainder                        
                reader.read_exact(message_vec.as_mut_slice())?;

                // trash the remaining message
                let remaining_bytes = (mem::size_of_val(delimiter_string.as_bytes()) * 2) + mem::size_of_val(ending_boundary.as_bytes());
                let mut trash_vec = create_empty_vec_of_size(remaining_bytes);
                reader.read_exact(trash_vec.as_mut_slice())?;
                return Ok(message_vec);
            } else {
                return Err(Error::from(ErrorKind::BufferDoesntContainDelimiter));
            }
        } else {
            return Err(Error::from(ErrorKind::BeginningDoesntMatch));
        }
    } else {
        return Err(Error::from(ErrorKind::DelimiterDoesntMatch));
    }
}

fn create_empty_vec_of_size(size: usize) -> Vec<u8> {
    let mut vec = Vec::new();
    for _ in 0..size {
        vec.push(0);
    }
    return vec;
}