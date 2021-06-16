use core::usize;

#[cfg(feature = "std")]
use std::{fs::File, io::{BufRead, BufReader}};


pub trait CharStream {
    fn get_char(&mut self) -> Option<char>;
    fn get_line<'a>(&mut self, buf: &'a [char], max_len: usize) -> Option<&'a [char]> {
        let mut index = 0;
        while let Some(c) = self.get_char() {
            buf[index] = c;
            index += 1;
            if c == '\n' || index > max_len {
                break;
            }
        }
        
        if index == 0 {
            None
        } else {
            Some(buf)
        }
    }
}


#[cfg(feature = "std")]
pub struct FileCharStream {
    buf_reader: BufReader<File>,
    index: usize,
    current_line: String,
}

#[cfg(feature = "std")]
impl FileCharStream {
    pub fn new(filename: &str) -> Option<Self> {
        match File::open(filename) {
            Ok(f) => {
                Some(Self {
                    buf_reader: BufReader::new(f),
                    index: 0,
                    current_line: String::new(),
                })
            }
            Err(_) => None
        }
    }
}

impl CharStream for FileCharStream {
    fn get_char(&mut self) -> Option<char> {
        // TODO: проверить self.index == self.current_line.len()
        if self.current_line.is_empty() || self.index == self.current_line.len() {
            let mut buf = String::new();
            let res = self.buf_reader.read_line(&mut buf);
            match res {
                Ok(0) => return None,
                Err(_) => return None,
                Ok(_) => {
                    self.current_line = buf;
                    self.index = 0;
                }
            }
        }

        self.index += 1;
        self.current_line.chars().nth(self.index - 1)
    }
}

#[test]
fn asd() {
    let fcs = FileCharStream::new("example.cfg").unwrap();
    let mut buf = ['a'; 22];
    fcs.get_line(&buf, 22);
    buf[0] = 's';
    
    println!("{:?}", buf);
}