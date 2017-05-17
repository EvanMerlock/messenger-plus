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
/// assert!(messenger_plus::utils::vec_contains_slice(&vec, &[2, 13, 17]));
/// assert!(messenger_plus::utils::vec_contains_slice(&vec, &[1, 6, 9]));
/// # }
/// ```
///
/// These will return false:
///
/// ```
/// # extern crate messenger_plus;
/// # fn main() {
/// # let vec = vec![1, 6, 9, 0, 2, 13, 17, 18];
/// assert!(!messenger_plus::utils::vec_contains_slice(&vec, &[2, 2, 17]));
/// assert!(!messenger_plus::utils::vec_contains_slice(&vec, &[]));
/// assert!(!messenger_plus::utils::vec_contains_slice(&Vec::new(), &[2, 2]));
/// assert!(!messenger_plus::utils::vec_contains_slice::<i32>(&Vec::new(), &[]));
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
/// assert_eq!(messenger_plus::utils::find_where_slice_intersects(&vec, &[2, 13, 17]), Some(7));
/// assert_eq!(messenger_plus::utils::find_where_slice_intersects(&vec, &[1, 6, 9]), Some(3));
/// # }
/// ```
///
/// These will return None:
///
/// ```
/// # extern crate messenger_plus;
/// # fn main() {
/// # let vec = vec![1, 6, 9, 0, 2, 13, 17, 18];
/// assert_eq!(messenger_plus::utils::find_where_slice_intersects(&vec, &[2, 2, 17]), None);
/// assert_eq!(messenger_plus::utils::find_where_slice_intersects(&vec, &[]), None);
/// assert_eq!(messenger_plus::utils::find_where_slice_intersects(&Vec::new(), &[2, 2]), None);
/// assert_eq!(messenger_plus::utils::find_where_slice_intersects::<i32>(&Vec::new(), &[]), None);
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
/// assert_eq!(messenger_plus::utils::find_where_slice_begins(&vec, &[2, 13, 17]), Some(4));
/// assert_eq!(messenger_plus::utils::find_where_slice_begins(&vec, &[1, 6, 9]), Some(0));
/// # }
/// ```
///
/// These will return None:
///
/// ```
/// # extern crate messenger_plus;
/// # fn main() {
/// # let vec = vec![1, 6, 9, 0, 2, 13, 17, 18];
/// assert_eq!(messenger_plus::utils::find_where_slice_begins(&vec, &[2, 2, 17]), None);
/// assert_eq!(messenger_plus::utils::find_where_slice_begins(&vec, &[]), None);
/// assert_eq!(messenger_plus::utils::find_where_slice_begins(&Vec::new(), &[2, 2]), None);
/// assert_eq!(messenger_plus::utils::find_where_slice_begins::<i32>(&Vec::new(), &[]), None);
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
/// assert_eq!(messenger_plus::utils::locate_items_between_delimiters(&vec, &[2], &[18]), Some(vec![13, 17]));
/// assert_eq!(messenger_plus::utils::locate_items_between_delimiters(&vec, &[1], &[9]), Some(vec![6]));
/// # }
/// ```
///
/// These will return None:
///
/// ```
/// # extern crate messenger_plus;
/// # fn main() {
/// # let vec = vec![1, 6, 9, 0, 2, 13, 17, 18];
/// assert_eq!(messenger_plus::utils::locate_items_between_delimiters(&vec, &[2], &[3]), None);
/// assert_eq!(messenger_plus::utils::locate_items_between_delimiters(&vec, &[1], &[1]), None);
/// assert_eq!(messenger_plus::utils::locate_items_between_delimiters(&vec, &[], &[6]), None);
/// assert_eq!(messenger_plus::utils::locate_items_between_delimiters(&vec, &[1], &[]), None);
/// assert_eq!(messenger_plus::utils::locate_items_between_delimiters(&Vec::new(), &[1], &[1]), None);
/// assert_eq!(messenger_plus::utils::locate_items_between_delimiters(&Vec::new(), &[1], &[]), None);
/// assert_eq!(messenger_plus::utils::locate_items_between_delimiters(&Vec::new(), &[], &[1]), None);
/// assert_eq!(messenger_plus::utils::locate_items_between_delimiters::<i32>(&Vec::new(), &[], &[]), None);
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