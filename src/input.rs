use std::io;
use std::io::Read;
use std::fs::File;

pub enum Input {
    Stdin(io::Stdin),
    File(File)
}

impl Read for Input {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match *self {
            Input::Stdin(ref mut stdin) => stdin.read(buf),
            Input::File(ref mut file) => file.read(buf)
        }
    }
}