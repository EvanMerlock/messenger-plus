extern crate messager_plus;

use std::io;

struct RandomRead {
    info: Vec<u8>,
}

impl RandomRead {

    fn new(message: &str) -> RandomRead {
        let mut data: Vec<u8> = Vec::new();
        data.append(&mut Vec::from("--boundary"));
        data.append(&mut Vec::from(message));
        data.append(&mut Vec::from("--endboundary"));
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
    let data = RandomRead::new(payload_one);

    assert_eq!(messager_plus::read_stream::read_next_message(data, "--boundary", "--endboundary"), Some(Vec::from(payload_one)));
}

#[test]
fn special_characters_test() {
    let payload_one = "!@#$%^&*()_+-=[]{}|;:/?><";
    let data = RandomRead::new(payload_one);

    assert_eq!(messager_plus::read_stream::read_next_message(data, "--boundary", "--endboundary"), Some(Vec::from(payload_one)));
}