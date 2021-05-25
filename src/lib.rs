use std::{fs::File, io::{BufRead, BufReader}};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

trait CharStream {
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

struct CfgLine {
    name: String,
    value: String,
}

struct CfgReader<'a> {
    char_stream: &'a mut dyn CharStream,
}

impl<'a> CfgReader<'a> {
    fn new(char_stream: &'a mut dyn CharStream) -> Self {
        Self { char_stream }
    }

    fn get_cfg_line(&mut self) -> Option<CfgLine> {
        let raw_line = self.char_stream.get_line()?;
        let eq_pos = raw_line.find('#')?;
        let raw_line = raw_line[0 .. eq_pos].to_string();
        
        let parts = raw_line.split('=').collect::<Vec<&str>>();
        if parts.len() != 2 {
            return  None;
        }
        
        Some(CfgLine {
            name: parts[0].to_string(),
            value: parts[1].to_string()
        })
    }
}

#[test]
fn reader_test() {
    struct A {
        br: BufReader<File>,
        index: usize,
        current_line: String
    }
    
    impl A {
        fn new() -> Self {
            let f = File::open("example.txt").unwrap();
            Self {
                br: BufReader::new(f),
                index: 0,
                current_line: String::new()
            }
        }
    }
    
    impl CharStream for A {
        fn get_char(&mut self) -> Option<char> {
            if self.current_line.is_empty() {
                let mut buf = String::new();
                let res = self.br.read_line(&mut buf);
                match res {
                    Ok(0) => return None,
                    Err(_) => return None,
                    Ok(_) => {self.index = 0;}
                }
            }
            self.index += 1;
            self.current_line.chars().nth(self.index - 1)
        }
    }

    let mut a = A::new();

    while let Some(line) = a.get_line() {
        print!("{}", line);
    }

    let mut c = CfgReader::new(&mut a);
    c.get_cfg_line();
}
