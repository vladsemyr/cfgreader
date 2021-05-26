use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub trait CharStream {
    fn get_char(&mut self) -> Option<char>;
    fn get_line(&mut self) -> Option<String> {
        let mut line = String::new();
        while let Some(c) = self.get_char() {
            line.push(c);
            if c == '\n' {
                break;
            }
        }
        if line.is_empty() {
            None
        } else {
            Some(line)
        }
    }
}

pub struct FileCharStream {
    buf_reader: BufReader<File>,
    index: usize,
    current_line: String,
}

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