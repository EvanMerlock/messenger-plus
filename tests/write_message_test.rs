extern crate messenger_plus;

use std::io::{Write, Result};

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
    {
        let mut message_writer = messenger_plus::write_stream::MessageWriter::new(String::from("--boundary"), String::from("--endboundary"), &mut writer);
        let _ = message_writer.write(b"hello, world!");
    }

    let payload_vec = Vec::from(String::from("--boundaryhello, world!--endboundary"));

    assert_eq!(writer.info, payload_vec);
}