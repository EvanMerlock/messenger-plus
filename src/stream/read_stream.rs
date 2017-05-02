use std::io::{Read};

pub struct MessageReader<'a> {
    beginning_boundary: String,
    ending_boundary: String,
    reader: &'a mut Read,
}

impl<'a> MessageReader<'a> {

    /// Initializes a new MessageReader
    ///
    /// MessageReaders read a given `Read` trait-object for any messages between the given boundaries.
    pub fn new(beg_bound: String, end_bound: String, reader: &mut Read) -> MessageReader {
        MessageReader {
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
            // read the provided stream
            let res = self.reader.read(&mut buffer);

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
}

/// Reads the next message from any Reader
///
/// This method reads the next message in any given reader.
/// The message is formatted with 2 boundaries. The first boundary is defined in boundary_start, and the second is defined in boundary_end.
///
/// # Arguments
/// * `stream` - a `Read` trait-object that the message will be read from
/// * `boundary_start` - the marker of the beginning of the message. Ensure this is unique and will not be contained within the message!
/// * `boundary_end` - the marker of the end of the message. Ensure this is unique and will not be contained within the message!
///
/// # Errors
/// This method will return None if it cannot find a message and the stream ends (typically due to EOF).
/// This method can hang if no new data is sent through the pipe as `Read` can block.
/// This method can produce irratic results if the `boundary_start` or `boundary_end` is found within the message.
pub fn read_next_message<T>(stream: &mut T, boundary_start: &str, boundary_end: &str) -> Option<Vec<u8>> where T: Read {

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
        let res = stream.read(&mut buffer);

        // If no bytes have been sent, return None. We probably reached the end of the stream.
        if let Ok(v) = res {
            if v <= 0 {
                return None;
            }
        }

        if beginning_found {
            // look for ending, accumulate message
            message.append(&mut Vec::from(buffer.as_ref()));
            if vec_contains_slice(&message, boundary_end.as_ref()) {
                // end code found. filter it out and send the message.
                if let Some(v) = find_where_slice_begins(&message, boundary_end.as_ref()) {
                    let _ = message.split_off(v as usize);
                    return Some(message);
                }
            }
        } else {
            // beginning message NOT found, look for it
            search_vec.append(&mut Vec::from(buffer.as_ref()));
            if vec_contains_slice(&search_vec, boundary_start.as_ref()) {
                // grab everything after, then push it into the message buffer
                if let Some(v) = find_where_slice_intersects(&search_vec, boundary_start.as_ref()) {
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
                        if x > delimiter_location && x < i {
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