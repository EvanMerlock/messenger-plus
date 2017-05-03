use std::io::{Read};
use std::mem;

pub struct MessageReader<'a> {
    delimiter_string: String,
    beginning_boundary: String,
    ending_boundary: String,
    reader: &'a mut Read,
}

impl<'a> MessageReader<'a> {

    /// Initializes a new MessageReader
    ///
    /// MessageReaders read a given `Read` trait-object for any messages between the given boundaries.
    pub fn new(delimiter_string: String, beg_bound: String, end_bound: String, reader: &mut Read) -> MessageReader {
        MessageReader {
            delimiter_string: delimiter_string,
            beginning_boundary: beg_bound,
            ending_boundary: end_bound,
            reader: reader,
        }
    }

    /// Reads the next message from the MessageReader
    ///
    /// This method reads the next message from the previously created MessageReader
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
                let _ = self.reader.read(message.as_mut_slice());
                // look for ending, accumulate message
                let delim = self.delimiter_string.clone();
                let end = self.ending_boundary.clone();
                let proper_delim = delim + end.as_str();
                println!("proper delim: {:?}", proper_delim);
                if vec_contains_slice(&message, proper_delim.as_ref()) {
                    // end code found. filter it out and send the message.
                    if let Some(v) = find_where_slice_begins(&message, proper_delim.as_ref()) {
                        let other_half = message.split_off(v as usize);
                        println!("message: {:?}, other half: {:?}", message, other_half);
                        return Some(message);
                    }
                }
            } else {
                // beginning message NOT found, look for it
                // read the provided stream
                let res = self.reader.read(&mut buffer);

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
}

/// Determines if a given Vector contains a given slice
///
/// # Arguments
/// 
/// * `vec` - a vector that may contain the given slice
/// * `slice` - a slice that the vector may contain
///
/// # Examples
/// Given a vector
///
/// ```
/// let vec = vec![1, 6, 9, 0, 2, 13, 17, 18];
/// ```
///
/// These will return true:
///
/// ```
/// # extern crate messenger_plus;
/// # fn main() {
/// # let vec = vec![1, 6, 9, 0, 2, 13, 17, 18];
/// assert!(messenger_plus::stream::vec_contains_slice(&vec, &[2, 13, 17]));
/// assert!(messenger_plus::stream::vec_contains_slice(&vec, &[1, 6, 9]));
/// # }
/// ```
///
/// These will return false:
///
/// ```
/// # extern crate messenger_plus;
/// # fn main() {
/// # let vec = vec![1, 6, 9, 0, 2, 13, 17, 18];
/// assert!(!messenger_plus::stream::vec_contains_slice(&vec, &[2, 2, 17]));
/// assert!(!messenger_plus::stream::vec_contains_slice(&vec, &[]));
/// assert!(!messenger_plus::stream::vec_contains_slice(&Vec::new(), &[2, 2]));
/// assert!(!messenger_plus::stream::vec_contains_slice::<i32>(&Vec::new(), &[]));
/// # }
/// ```
pub fn vec_contains_slice<T>(vec: &Vec<T>, slice: &[T]) -> bool where T: Copy + PartialEq {
    let mut clone_vec: Vec<T> = vec.clone();

    if slice.is_empty() || vec.is_empty() {
        return false;
    }

    for i in 0..vec.len() {
        if vec[i] == slice[0] {
            let mut cont_vec = clone_vec.split_off(i);
            if cont_vec.starts_with(slice) {
                return true;
            }
            clone_vec.append(&mut cont_vec);
        }
    }

    false
}

/// Returns the index of the element AFTER the slice if the slice exists within the vector
///
/// # Arguments
/// * `vec` - a vector that may contain the given slice
/// * `slice` - a slice that may be contained within the given vector
///
/// # Examples
///
/// Given a vector
///
/// ```
/// let vec = vec![1, 6, 9, 0, 2, 13, 17, 18];
/// ```
///
/// These will return Some and a number relating to the location:
///
/// ```
/// # extern crate messenger_plus;
/// # fn main() {
/// # let vec = vec![1, 6, 9, 0, 2, 13, 17, 18];
/// assert_eq!(messenger_plus::stream::find_where_slice_intersects(&vec, &[2, 13, 17]), Some(7));
/// assert_eq!(messenger_plus::stream::find_where_slice_intersects(&vec, &[1, 6, 9]), Some(3));
/// # }
/// ```
///
/// These will return None:
///
/// ```
/// # extern crate messenger_plus;
/// # fn main() {
/// # let vec = vec![1, 6, 9, 0, 2, 13, 17, 18];
/// assert_eq!(messenger_plus::stream::find_where_slice_intersects(&vec, &[2, 2, 17]), None);
/// assert_eq!(messenger_plus::stream::find_where_slice_intersects(&vec, &[]), None);
/// assert_eq!(messenger_plus::stream::find_where_slice_intersects(&Vec::new(), &[2, 2]), None);
/// assert_eq!(messenger_plus::stream::find_where_slice_intersects::<i32>(&Vec::new(), &[]), None);
/// # }
/// ```
pub fn find_where_slice_intersects<T>(vec: &Vec<T>, slice: &[T]) -> Option<usize> where T: Copy + PartialEq {
    let mut clone_vec: Vec<T> = vec.clone();

    if slice.is_empty() || vec.is_empty() {
        return None;
    }

    for i in 0..vec.len() {
        if vec[i] == slice[0] {
            let mut cont_vec = clone_vec.split_off(i);
            if cont_vec.starts_with(slice) {
                return Some(i + slice.len());
            }
            clone_vec.append(&mut cont_vec);
        }
    }

    None
}

/// Returns the index of the element BEFORE the slice if the slice exists within the vector
///
/// # Arguments
/// * `vec` - a vector that may contain the given slice
/// * `slice` - a slice that may be contained within the given vector
///
/// # Examples
///
/// Given a vector
///
/// ```
/// let vec = vec![1, 6, 9, 0, 2, 13, 17, 18];
/// ```
///
/// These will return Some and a number relating to the location:
///
/// ```
/// # extern crate messenger_plus;
/// # fn main() {
/// # let vec = vec![1, 6, 9, 0, 2, 13, 17, 18];
/// assert_eq!(messenger_plus::stream::find_where_slice_begins(&vec, &[2, 13, 17]), Some(4));
/// assert_eq!(messenger_plus::stream::find_where_slice_begins(&vec, &[1, 6, 9]), Some(0));
/// # }
/// ```
///
/// These will return None:
///
/// ```
/// # extern crate messenger_plus;
/// # fn main() {
/// # let vec = vec![1, 6, 9, 0, 2, 13, 17, 18];
/// assert_eq!(messenger_plus::stream::find_where_slice_begins(&vec, &[2, 2, 17]), None);
/// assert_eq!(messenger_plus::stream::find_where_slice_begins(&vec, &[]), None);
/// assert_eq!(messenger_plus::stream::find_where_slice_begins(&Vec::new(), &[2, 2]), None);
/// assert_eq!(messenger_plus::stream::find_where_slice_begins::<i32>(&Vec::new(), &[]), None);
/// # }
/// ```
pub fn find_where_slice_begins<T>(vec: &Vec<T>, slice: &[T]) -> Option<usize> where T: Copy + PartialEq {
    find_where_slice_intersects(vec, slice).map(|x| x - slice.len())
}

/// Returns all items between two given slices of items
///
/// # Arguments
/// * `vec` - the vector that may contain items between two delimiter slices
/// * `delimiter_slice` - the beginning of the delimiter
/// * `slice` - the end of the delimiter
///
/// # Examples
///
/// Given a vector
///
/// ```
/// let vec = vec![1, 6, 9, 0, 2, 13, 17, 18];
/// ```
///
/// These will return Some and a slice of all the elements in between:
///
/// ```
/// # extern crate messenger_plus;
/// # fn main() {
/// # let vec = vec![1, 6, 9, 0, 2, 13, 17, 18];
/// assert_eq!(messenger_plus::stream::locate_items_between_delimiters(&vec, &[2], &[18]), Some(vec![13, 17]));
/// assert_eq!(messenger_plus::stream::locate_items_between_delimiters(&vec, &[1], &[9]), Some(vec![6]));
/// # }
/// ```
///
/// These will return None:
///
/// ```
/// # extern crate messenger_plus;
/// # fn main() {
/// # let vec = vec![1, 6, 9, 0, 2, 13, 17, 18];
/// assert_eq!(messenger_plus::stream::locate_items_between_delimiters(&vec, &[2], &[3]), None);
/// assert_eq!(messenger_plus::stream::locate_items_between_delimiters(&vec, &[1], &[1]), None);
/// assert_eq!(messenger_plus::stream::locate_items_between_delimiters(&vec, &[], &[6]), None);
/// assert_eq!(messenger_plus::stream::locate_items_between_delimiters(&vec, &[1], &[]), None);
/// assert_eq!(messenger_plus::stream::locate_items_between_delimiters(&Vec::new(), &[1], &[1]), None);
/// assert_eq!(messenger_plus::stream::locate_items_between_delimiters(&Vec::new(), &[1], &[]), None);
/// assert_eq!(messenger_plus::stream::locate_items_between_delimiters(&Vec::new(), &[], &[1]), None);
/// assert_eq!(messenger_plus::stream::locate_items_between_delimiters::<i32>(&Vec::new(), &[], &[]), None);
/// # }
/// ```
pub fn locate_items_between_delimiters<T>(vec: &Vec<T>, delimiter_slice: &[T], slice: &[T]) -> Option<Vec<T>> where T: Copy + PartialEq {
    let mut clone_vec: Vec<T> = vec.clone();
    let mut delimiter_found = false;
    let mut delimiter_location: usize = 0;

    if slice.is_empty() || vec.is_empty() || delimiter_slice.is_empty() {
        return None;
    }

    for i in 0..vec.len() {

        if !delimiter_found {
            if vec[i] == delimiter_slice[0] {
                let mut cont_vec = clone_vec.split_off(i);
                if cont_vec.starts_with(delimiter_slice) {
                    delimiter_found = true;
                    delimiter_location = i;
                    clone_vec.append(&mut cont_vec);
                    continue;
                }
                clone_vec.append(&mut cont_vec);
            }
        } else {
            if vec[i] == slice[0] {
                let mut cont_vec = clone_vec.split_off(i);
                if cont_vec.starts_with(slice) {
                    let mut new_vec: Vec<T> = Vec::new();
                    for x in 0..vec.len() {
                        if x >= (delimiter_location + delimiter_slice.len()) && x < i {
                            new_vec.push(clone_vec[x]);
                        }
                    }
                    return Some(new_vec);
                }
                clone_vec.append(&mut cont_vec);
            }
        }
    }

    None
}