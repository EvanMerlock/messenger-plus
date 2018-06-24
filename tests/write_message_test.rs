extern crate messenger_plus;

use std::io::{Write, Result};
use std::ops::Add;
use std::mem;

#[derive(Debug)]
struct RandomWrite {
    info: Vec<u8>,
}

impl RandomWrite {
    fn new() -> RandomWrite {
        RandomWrite {
            info: Vec::new(),
        }
    }
}

impl Write for RandomWrite {

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

#[test]
fn writes_message_properly() {
    let mut writer = RandomWrite::new();
    let buf: &[u8] = "hello, world!".as_ref();
    let mut message_writer = messenger_plus::stream::MessageWriter::new("--", "bound", "endbound", writer, false);
    let _ = message_writer.write(buf);

    let payload_vec = Vec::from(String::from("--bound").add(mem::size_of_val(buf).to_string().as_str()).add("--hello, world!--endbound--"));

    assert_eq!(message_writer.get_writer().info, payload_vec);
}