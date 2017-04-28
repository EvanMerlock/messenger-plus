extern crate messenger_plus;

use std::io;

#[derive(Debug)]
struct RandomRead {
    info: Vec<u8>,
}

impl RandomRead {

    fn new(message: &str, num_payloads: i32) -> RandomRead {
        let mut data: Vec<u8> = Vec::new();
        for _ in 0..num_payloads {
            data.append(&mut Vec::from("--boundary"));
            data.append(&mut Vec::from(message));
            data.append(&mut Vec::from("--endboundary"));
        }
        RandomRead {
            info: data,
        }
    }

}

impl io::Read for RandomRead {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        for i in 0..buf.len() {
            if self.info.is_empty() {
                return Ok(i as usize);
            }
            buf[i] = self.info.remove(0);
        }
        Ok(buf.len())
    }
}

#[test]
fn read_next_message_test() {
    let payload_one = "payload_one";
    let mut data = RandomRead::new(payload_one, 1);

    assert_eq!(messenger_plus::stream::read_next_message(&mut data, "--boundary", "--endboundary"), Some(Vec::from(payload_one)));
}

#[test]
fn special_characters_test() {
    let payload_one = "!@#$%^&*()_+-=[]{}|;:/?><";
    let mut data = RandomRead::new(payload_one, 1);

    assert_eq!(messenger_plus::stream::read_next_message(&mut data, "--boundary", "--endboundary"), Some(Vec::from(payload_one)));
}

#[test]
fn read_multiple_payloads_test() {
    let payload_one = "payload_one";
    let num_payloads = 3;
    let mut data = RandomRead::new(payload_one, num_payloads);


    for _ in 0..num_payloads {
        assert_eq!(messenger_plus::stream::read_next_message(&mut data, "--boundary", "--endboundary"), Some(Vec::from(payload_one)));
    }
}

#[test]
fn read_empty_payload_test() {
    let mut data = RandomRead::new("", 0);
    assert_eq!(messenger_plus::stream::read_next_message(&mut data, "--boundary", "--endboundary"), None);
}