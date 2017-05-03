extern crate messenger_plus;

use std::io::{Read, Write, Result};

#[derive(Debug)]
struct RandomReadWrite {
    info: Vec<u8>,
}

impl RandomReadWrite {
    fn new() -> RandomReadWrite {
        RandomReadWrite {
            info: Vec::new(),
        }
    }
}

impl Write for RandomReadWrite {

    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let mut count = 0;
        for i in buf {
            self.info.push(*i);
            count = count + 1;
        }
        Ok(count)
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }

}

impl Read for RandomReadWrite {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
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
fn dual_messenger_test() {
    let mut random_reader = RandomReadWrite::new();
    let mut message_reader = messenger_plus::stream::DualMessenger::new(String::from("--"), String::from("bound"), String::from("endbound"), &mut random_reader);
    let buf: &[u8] = "hello, world!".as_ref();


    assert_eq!(message_reader.write(buf).unwrap(), Vec::from("--bound12--hello, world!--endbound--").len());
    assert_eq!(message_reader.read_next_message(), Some(Vec::from("hello, world!")));
    assert_eq!(message_reader.read_next_message(), None);
}

#[test]
fn dual_message_multi_test() {
    let mut random_reader = RandomReadWrite::new();
    let mut message_reader = messenger_plus::stream::DualMessenger::new(String::from("--"), String::from("bound"), String::from("endbound"), &mut random_reader);
    let buf: &[u8] = "hello, world!".as_ref();

    assert_eq!(message_reader.write(buf).unwrap(), Vec::from("--bound12--hello, world!--endbound--").len());
    assert_eq!(message_reader.write(buf).unwrap(), Vec::from("--bound12--hello, world!--endbound--").len());
    assert_eq!(message_reader.write(buf).unwrap(), Vec::from("--bound12--hello, world!--endbound--").len());
    assert_eq!(message_reader.read_next_message(), Some(Vec::from("hello, world!")));
    assert_eq!(message_reader.read_next_message(), Some(Vec::from("hello, world!")));
    assert_eq!(message_reader.read_next_message(), Some(Vec::from("hello, world!")));
    assert_eq!(message_reader.read_next_message(), None);
}