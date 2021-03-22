
use std::path::Path;
use std::io::{self, Read};
use std::fmt;
use std::error;
use std::fs::{self, File};


#[derive(Debug)]
pub enum ROMError {
   IOError(io::Error),
   BadError,
}

impl fmt::Display for ROMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ROMError::IOError(_) =>
                write!(f, "ROMError::IOError could not ROM"),
            ROMError::BadError =>
                write!(f, "ROMError::BadError")
        }
    }
}

impl error::Error for ROMError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            ROMError::IOError(ref e) =>
                Some(e),
            ROMError::BadError => 
                None,
        }
    }
}

impl From<io::Error> for ROMError {
    fn from(err: io::Error) -> ROMError {
        ROMError::IOError(err)
    }
}

pub struct ROM {
    data: Vec<u8>
}

impl ROM {
    fn new(data: Vec<u8>) -> ROM {
        ROM { data: data }
    }
   
    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
}

impl IntoIterator for &ROM {
    type Item = u16;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut i = 0;
        let mut romdata : Vec<Self::Item> = vec![];

        println!("size of ROM: {}", self.size());
        while i < self.data.len() {
            let mut word : u16 = (self.data[i] as u16) << 8;
            if i + 1 < self.size() {
                word = word | (self.data[i+1] as u16);
            }
            i += 2;
            romdata.push(word);
        }
        romdata.into_iter()
    }
}

pub fn read_rom(path: &Path) -> Result<ROM, ROMError> {
    let mut f = File::open(path)?;
    let fmeta = fs::metadata(path);
    let fsize = fmeta.unwrap().len();
    let mut buf : Vec<u8> = vec![0u8; fsize as usize];
    let len = f.read(&mut buf[..]);
    if let Err(_) = len {
        Err(ROMError::BadError)
    } else {
        Ok(ROM::new(buf))
    }
}

