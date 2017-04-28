use std::io::{Read};

pub fn read_next_message<T>(stream: &mut T, boundary_start: &str, boundary_end: &str) -> Option<Vec<u8>> where T: Read {

    let mut buffer = [0; 1];
    let mut message: Vec<u8> = Vec::new();
    let mut search_vec: Vec<u8> = Vec::new();


    let mut beginning_found = false;

    loop {
        let res = stream.read(&mut buffer);

        if let Ok(v) = res {
            if v <= 0 {
                return None;
            }
        }

        if beginning_found {
            message.append(&mut Vec::from(buffer.as_ref()));
            // look for ending, accumulate message
            if vec_contains_slice(&message, boundary_end.as_ref()) {
                // end code found! filter it out and send the message!
                println!("end code found!");
                if let Some(v) = find_where_slice_begins(&message, boundary_end.as_ref()) {
                    println!("found boundary slice");
                    let _ = message.split_off(v as usize);
                    return Some(message);
                }
                println!("didn't find the boundary slice");
            }
        } else {
            // beginning message NOT found, look for it
            search_vec.append(&mut Vec::from(buffer.as_ref()));
            if vec_contains_slice(&search_vec, boundary_start.as_ref()) {
                // grab everything after, then push it into the message buffer
                if let Some(v) = find_where_slice_intersects(&search_vec, boundary_start.as_ref()) {
                    println!("found beginning");
                    message.append(&mut search_vec.split_off(v as usize));
                    beginning_found = true;
                    search_vec.clear();
                }
            } 
        }

    }
}

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

pub fn find_where_slice_begins<T>(vec: &Vec<T>, slice: &[T]) -> Option<usize> where T: Copy + PartialEq {
    find_where_slice_intersects(vec, slice).map(|x| x - slice.len())
}