mod char_stream;

#[derive(Debug)]
pub enum CfgType {
    IntNumber(i32),
    FloatNumber(f32),
    Literal(String),
    String(String)
}

impl CfgType {
    fn parse(s: String) -> Option<Self> {
        let is_int = s.parse::<i32>();
        let is_float = s.parse::<f32>();
        let is_literal = {
            s.find(char::is_whitespace).is_none() &&
            s.find(|c: char| {!c.is_ascii_alphabetic()}).is_none()
        };
        let is_string = {
            if s.len() < 2 {
                false
            }
            else {
                s.starts_with('\"') && s.ends_with('\"') &&
                s.chars().filter(|c| *c == '\"').collect::<Vec<char>>().len() == 2
            }
        };
        
        if is_int.is_ok() {
            Some(Self::IntNumber(is_int.unwrap()))
        }
        else if is_float.is_ok() {
            Some(Self::FloatNumber(is_float.unwrap()))
        }
        else if is_literal {
            Some(Self::Literal(s))
        }
        else if is_string {
            Some(Self::String(s[1 ..= s.len()-2].to_string()))
        }
        else {
            None
        }
    }
    
    // TODO: сделать при помощи макроса
    
    pub fn is_int(&self) -> bool {
        match self {
            CfgType::IntNumber(_) => true,
            _ => false
        }
    }
    
    pub fn is_float(&self) -> bool {
        match self {
            CfgType::FloatNumber(_) => true,
            _ => false
        }
    }
    
    pub fn is_literal(&self) -> bool {
        match self {
            CfgType::Literal(_) => true,
            _ => false
        }
    }
    
    pub fn is_string(&self) -> bool {
        match self {
            CfgType::String(_) => true,
            _ => false
        }
    }
}



#[derive(Debug)]
pub struct CfgLine {
    pub name: String,
    pub value: CfgType,
}

pub trait CfgReader<'a> {
    fn get_char_stream(&'a mut self) -> &'a mut dyn char_stream::CharStream;

    fn get_next(&'a mut self) -> Option<CfgLine> {
        let char_stream = self.get_char_stream();
        loop {
            let mut raw_line = char_stream.get_line()?;
            let sharp_pos = raw_line.find('#');
            if sharp_pos.is_some() {
                raw_line = raw_line[0..sharp_pos.unwrap()].to_string();
            }

            let parts = raw_line.split('=').map(|x| x.trim()).collect::<Vec<&str>>();
            if parts.len() != 2 {
                continue;
            }
            
            let value = {
                let raw_value = parts[1].to_string();
                CfgType::parse(raw_value)
            };
            
            if value.is_none() {
                continue;
            }
            
            return Some(CfgLine {
                name: parts[0].to_string(),
                value: value.unwrap(),
            })
        }
    }
}


pub struct FileCfgReader {
    char_stream: char_stream::FileCharStream,
}

impl FileCfgReader {
    pub fn open(filename: &str) -> Option<FileCfgReader> {
        Some(Self {
            char_stream: char_stream::FileCharStream::new(filename)?
        })
    }
}

impl<'a> CfgReader<'a> for FileCfgReader {
    fn get_char_stream(&'a mut self) -> &'a mut dyn char_stream::CharStream {
        &mut self.char_stream
    }
}


#[test]
fn reader_test() {
    let mut a = FileCfgReader::open("example.cfg").unwrap();
    println!("{:?}", a.get_next());
    println!("{:?}", a.get_next());
    println!("{:?}", a.get_next());
    println!("{:?}", a.get_next());
    println!("{:?}", a.get_next());
    println!("{:?}", a.get_next());
    println!("{:?}", a.get_next());
}
