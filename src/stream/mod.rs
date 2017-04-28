pub mod read_stream;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn vec_contains_slice() {
        let mut vec: Vec<u8> = Vec::new();
        vec.push(1);
        vec.push(6);
        vec.push(9);
        vec.push(0);
        vec.push(2);
        vec.push(13);
        vec.push(17);
        vec.push(18);

        let slice: &[u8] = &[2, 13, 17];

        assert!(read_stream::vec_contains_slice(&vec, slice));
    }

        #[test]
    fn vec_does_not_contain_slice() {
        let mut vec: Vec<u8> = Vec::new();
        vec.push(1);
        vec.push(6);
        vec.push(9);
        vec.push(0);
        vec.push(2);
        vec.push(13);
        vec.push(17);
        vec.push(18);

        let slice: &[u8] = &[2, 13, 13];

        assert!(!read_stream::vec_contains_slice(&vec, slice));

    }

        #[test]
    fn vec_contains_two_slice() {
        let mut vec: Vec<u8> = Vec::new();
        vec.push(1);
        vec.push(2);
        vec.push(13);
        vec.push(17);
        vec.push(2);
        vec.push(13);
        vec.push(17);
        vec.push(18);

        let slice: &[u8] = &[2, 13, 17];

        assert!(read_stream::vec_contains_slice(&vec, slice));

    }

        #[test]
    fn empty_vec() {
        let vec: Vec<u8> = Vec::new();
        let slice: &[u8] = &[2, 13, 17];

        assert!(!read_stream::vec_contains_slice(&vec, slice));

    }

        #[test]
    fn empty_slice() {
        let mut vec: Vec<u8> = Vec::new();
        vec.push(1);
        vec.push(6);
        vec.push(9);
        vec.push(0);
        vec.push(2);
        vec.push(13);
        vec.push(17);
        vec.push(18);

        let slice: &[u8] = &[];

        assert!(!read_stream::vec_contains_slice(&vec, slice));

    }

    #[test]
    fn vec_slice_location() {
        let mut vec: Vec<u8> = Vec::new();
        vec.push(2);
        vec.push(13);
        vec.push(17);
        vec.push(18);

        let slice: &[u8] = &[2, 13];

        assert_eq!(read_stream::find_where_slice_intersects(&vec, slice), Some(2));
    }

    #[test]
    fn vec_slice_no_loc() {
        let mut vec: Vec<u8> = Vec::new();
        vec.push(2);
        vec.push(13);
        vec.push(17);
        vec.push(18);

        let slice: &[u8] = &[2, 2];

        assert_eq!(read_stream::find_where_slice_intersects(&vec, slice), None);
    }

        #[test]
    fn vec_slice_begin_location() {
        let mut vec: Vec<u8> = Vec::new();
        vec.push(2);
        vec.push(13);
        vec.push(17);
        vec.push(18);

        let slice: &[u8] = &[2, 13];

        assert_eq!(read_stream::find_where_slice_begins(&vec, slice), Some(0));
    }

    #[test]
    fn vec_slice_begin_no_loc() {
        let mut vec: Vec<u8> = Vec::new();
        vec.push(2);
        vec.push(13);
        vec.push(17);
        vec.push(18);

        let slice: &[u8] = &[2, 2];

        assert_eq!(read_stream::find_where_slice_begins(&vec, slice), None);
    }

}