use std::mem;
use std::io::Read;

use super::super::utils::{find_where_slice_begins};
use super::{Error, Result, ErrorKind, StreamConfiguration};

pub fn read_message_from_reader(reader: &mut Read, configuration: &StreamConfiguration) -> Result<Vec<u8>> {

    let beginning_boundary = configuration.beginning_boundary.clone();
    let ending_boundary = configuration.ending_boundary.clone();
    let delimiter_string = configuration.delimiter_string.clone();

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